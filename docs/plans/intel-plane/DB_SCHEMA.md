# Intel Plane — Database Schema

The protocol-intelligence schema: aggregate → protocol → chain → market, split
into **identity** (stable, curated) and **time-series** (append-only history),
with **rollups** as a refreshable cache and **derived totals/regressions** as
views. Portable across Stage A (SQLite) and Stage B (Postgres + TimescaleDB) per
`DATA_LAYER.md`.

> This corrects an earlier draft. The key changes and *why* are in §6 so the
> reasoning isn't lost.

---

## 1. The model in one picture

```
IDENTITY (mutable, curated, one row each — "what a thing IS")
  protocols ──┐
              ├──< protocol_chains >── chains          (M:N: a protocol lives on many chains)
  protocols ──┴──< markets >── chains                  (1:N each side: a market = one protocol on one chain)
                    │
TIME-SERIES (append-only, many rows over time — "what was MEASURED at time T")
  aggregate_metrics_ts            (whole-portfolio totals)
  protocol_metrics_ts   ── protocol_id
  market_metrics_ts     ── market_id

ROLLUPS (refreshable cache, derivable from time-series — fast reads)
  volume_rollups        ── (protocol_id | market_id, window, value, computed_at)

DERIVED (VIEWS, never stored as truth)
  v_chain_totals        ── SUM(market tvl) per chain, latest snapshot
  v_protocol_latest     ── newest row per protocol
  v_regressions         ── diff vs previous snapshot (TVL%, volume%)
```

Two rules govern everything:
1. **Identity is mutable and singular; history is append-only and plural.**
2. **Anything that is a sum or a diff is computed (view/rollup), never stored as a column on an identity row.** Sums drift; history doesn't.

---

## 2. Identity tables (curated, human-gated)

```sql
-- A protocol. One row. Edited deliberately (the manual onboarding gate).
CREATE TABLE protocols (
    id          INTEGER PRIMARY KEY,           -- Stage B: BIGINT GENERATED ALWAYS AS IDENTITY
    name        TEXT NOT NULL UNIQUE,          -- "aave-v3"
    display_name TEXT NOT NULL,                -- "Aave V3"
    category    TEXT NOT NULL,                 -- 'lending' | 'dex' | 'staking' | 'perps' | 'other'
    abi_ref     TEXT,                          -- reference to ABI (see §5: a KEY, not a blob)
    watch       INTEGER NOT NULL DEFAULT 1,    -- monitor gathers data on it (bool)
    capital_target INTEGER NOT NULL DEFAULT 0, -- brain may deploy here — HUMAN-SET ONLY (bool)
    created_at  DATE NOT NULL,
    updated_at  DATE NOT NULL
);

-- A chain. One row per chain — NOT a table per chain.
CREATE TABLE chains (
    id        INTEGER PRIMARY KEY,
    name      TEXT NOT NULL UNIQUE,            -- "ethereum" | "base" | "arbitrum" | "optimism"
    chain_id  INTEGER NOT NULL UNIQUE          -- 1 | 8453 | 42161 | 10
);

-- M:N link: which protocols are present on which chains.
-- Replaces the `chains TEXT[]` array (an array can't be a real foreign key).
CREATE TABLE protocol_chains (
    protocol_id INTEGER NOT NULL REFERENCES protocols(id),
    chain_id    INTEGER NOT NULL REFERENCES chains(id),
    PRIMARY KEY (protocol_id, chain_id)
);

-- A market: a specific pool/vault/reserve at an address.
-- One row per market — NOT a table per contract address.
-- This is one protocol on one chain, so both FKs are single-valued.
CREATE TABLE markets (
    id           INTEGER PRIMARY KEY,
    protocol_id  INTEGER NOT NULL REFERENCES protocols(id),
    chain_id     INTEGER NOT NULL REFERENCES chains(id),
    address      TEXT NOT NULL,                -- lowercase hex, checksummed at edge
    market_type  TEXT NOT NULL,                -- 'pool' | 'vault' | 'lp' | 'reserve' | ...
    tokens       TEXT NOT NULL,                -- JSON array of token addresses (see §5)
    created_at   DATE NOT NULL,
    UNIQUE (chain_id, address)                 -- an address is unique per chain
);
CREATE INDEX idx_markets_protocol ON markets(protocol_id);
CREATE INDEX idx_markets_chain    ON markets(chain_id);
```

---

## 3. Time-series tables (APPEND-ONLY — the history)

No `UPDATE`/`DELETE` ever. Corrections are new rows. In Stage B each becomes a
TimescaleDB **hypertable** partitioned on `observed_at`.

```sql
-- Per-market measurements over time. The lowest-level truth.
CREATE TABLE market_metrics_ts (
    id           INTEGER PRIMARY KEY,
    market_id    INTEGER NOT NULL REFERENCES markets(id),
    observed_at  TEXT NOT NULL,
    tvl_base     TEXT NOT NULL,        -- NUMERIC(78,0) in PG. base units. NEVER BIGINT (see §6).
    volume_base  TEXT NOT NULL,        -- cumulative or interval volume, base units
    apy_bps      INTEGER,              -- basis points: 500 = 5.00%. NEVER float/`%`.
    apr_bps      INTEGER,
    source       TEXT NOT NULL,        -- provenance: "reth-ipc", "defillama", ...
    trust_tier   TEXT NOT NULL         -- 'OnChain' | 'ProtocolApi' | 'OffChain'
);
CREATE INDEX idx_mkt_ts ON market_metrics_ts(market_id, observed_at);

-- Per-protocol measurements over time (protocol-wide TVL + volume windows snapshot).
CREATE TABLE protocol_metrics_ts (
    id           INTEGER PRIMARY KEY,
    protocol_id  INTEGER NOT NULL REFERENCES protocols(id),
    observed_at  TEXT NOT NULL,
    tvl_base     TEXT NOT NULL,        -- NUMERIC(78,0)
    volume_base  TEXT NOT NULL,
    source       TEXT NOT NULL,
    trust_tier   TEXT NOT NULL
);
CREATE INDEX idx_proto_ts ON protocol_metrics_ts(protocol_id, observed_at);

-- Whole-portfolio aggregate snapshots. Replaces `main_db`.
-- Stored as its OWN append-only rows (clearly an aggregate), not a mutable column.
CREATE TABLE aggregate_metrics_ts (
    id                INTEGER PRIMARY KEY,
    observed_at       TEXT NOT NULL,
    total_tvl_base    TEXT NOT NULL,   -- NUMERIC(78,0)
    total_volume_base TEXT NOT NULL,
    protocol_count    INTEGER NOT NULL,
    source            TEXT NOT NULL,
    trust_tier        TEXT NOT NULL
);
CREATE INDEX idx_agg_ts ON aggregate_metrics_ts(observed_at);
```

---

## 4. Rollups + derived views

### Rollup cache (your chosen option: truth is raw, windows are cached)

```sql
-- Cached windowed volumes (24h/7d/30d/180d/365d). A CACHE, not truth —
-- always recomputable from *_metrics_ts. Refreshed on a schedule.
-- Stage B: replace this table with a TimescaleDB CONTINUOUS AGGREGATE, which
-- Timescale maintains incrementally.
CREATE TABLE volume_rollups (
    scope        TEXT NOT NULL,        -- 'market' | 'protocol'
    scope_id     INTEGER NOT NULL,     -- market_id or protocol_id
    window_label TEXT NOT NULL,        -- '24h' | '7d' | '30d' | '180d' | '365d'
    volume_base  TEXT NOT NULL,        -- NUMERIC(78,0)
    computed_at  TEXT NOT NULL,
    PRIMARY KEY (scope, scope_id, window_label)
);
```

> Why a cache and not columns on `protocols`: if "24h volume" is a column you
> `UPDATE`, you can never change its definition or backfill a gap without
> destroying the old value. As a cache over immutable history you can drop and
> rebuild every window whenever the methodology changes. The raw series is the
> only source of truth.

### Derived views (computed on read, never stored)

```sql
-- Latest snapshot per market (window-function pattern; portable).
CREATE VIEW v_market_latest AS
SELECT m.*
FROM market_metrics_ts m
JOIN (
    SELECT market_id, MAX(observed_at) AS latest
    FROM market_metrics_ts GROUP BY market_id
) x ON x.market_id = m.market_id AND x.latest = m.observed_at;

-- Per-chain TVL totals — replaces `total_tvl_inside_the_chain` as a SUM, so it
-- can never drift out of sync with the markets it sums.
CREATE VIEW v_chain_totals AS
SELECT mk.chain_id,
       COUNT(*)            AS market_count,
       SUM(CAST(l.tvl_base AS REAL)) AS total_tvl_base_approx  -- exact SUM in PG via NUMERIC
FROM v_market_latest l
JOIN markets mk ON mk.id = l.market_id
GROUP BY mk.chain_id;
```

> Note: SQLite has no 256-bit numeric, so cross-row SUMs in Stage A are either
> approximate (`REAL`, for display) or done in Rust with `U256`. In Stage B
> (`NUMERIC(78,0)`) the SUM is exact in SQL. Treat Stage-A SQL sums as
> display-only; the brain/safety read exact values via Rust.

### Regressions (diff vs previous snapshot) — also a view, never a column

```sql
-- "% change from last snapshot" for the aggregate. Replaces the `regression_*`
-- columns, which were impossible anyway (you can't diff from a row you overwrote).
CREATE VIEW v_aggregate_regression AS
SELECT
    cur.observed_at,
    cur.total_tvl_base,
    prev.total_tvl_base AS prev_tvl_base,
    cur.total_volume_base,
    prev.total_volume_base AS prev_volume_base
FROM aggregate_metrics_ts cur
JOIN aggregate_metrics_ts prev
  ON prev.observed_at = (
       SELECT MAX(observed_at) FROM aggregate_metrics_ts
       WHERE observed_at < cur.observed_at
  );
-- the actual %-diff is computed in Rust with U256 to stay exact.
```

---

## 5. Two recurring questions answered

**ABI: path or blob?** Store a **key/reference**, not the ABI text and not a
filesystem path. A path breaks the moment the binary runs elsewhere; an inlined
blob bloats every identity row. Use a stable `abi_ref` (e.g. `"aave-v3-pool@1"`)
that maps to an ABI bundled with the binary or a content-addressed store. The
DB records *which* ABI; the code owns the bytes.

**Arrays (`tokens`, the old `chains[]`/`contracts[]`):**
- A list that needs to be a **relationship** (chains a protocol is on) → a real
  link table with foreign keys (`protocol_chains`). Arrays can't be FKs.
- A list that is just an **attribute** of one row and never joined on (the token
  addresses in a pool) → fine as a JSON array column (`tokens`). Portable across
  SQLite and PG; query relationally only if you later need to.

---

## 6. What changed from the first draft, and why

| First draft | Problem | Fix |
|-------------|---------|-----|
| `create table base_chain`, `create table 0x...01` | A *table per chain / per contract* — unqueryable, needs runtime DDL | One `chains` row-per-chain; one `markets` row-per-contract |
| One mutable row per protocol/contract | Overwrites history; **`regression_*` is impossible without history** | Identity (mutable) split from `*_metrics_ts` (append-only) |
| `total_tvl`, `total_tvl_inside_the_chain` as columns | Stored sums drift out of sync | Computed views (`v_chain_totals`) / aggregate snapshot rows |
| `BIGINT` for TVL/volume | **Overflows at ~9.2 ETH** (max ~9.2e18 wei) | `NUMERIC(78,0)` (PG) / decimal-string (SQLite), i.e. `U256` |
| `apy %`, `apr BIGINT`, `regretion %` | `%` isn't a type; floats lose precision | Integer **basis points** (`apy_bps`, 500 = 5.00%) |
| `chains TEXT[]` "MUST connect" | Array columns can't be foreign keys | `protocol_chains` link table |
| `BIGSIREAL`, `UNIEQU`, `type_contacrt`, `regretion` | Typos | `BIGSERIAL`/`IDENTITY`, `UNIQUE`, `market_type`, `regression` |

---

## 7. How your four tables mapped

- `main_db` → **`aggregate_metrics_ts`** (append-only snapshots) + **`v_aggregate_regression`** (the diffs).
- `protocol_name` → **`protocols`** (identity) + **`protocol_metrics_ts`** (history) + **`volume_rollups`** (the 24h/7d/… windows).
- `base_chain` (and every chain) → **`chains`** (one row each) + **`v_chain_totals`** (the per-chain TVL sum).
- `0x...01` (and every contract) → **`markets`** (identity) + **`market_metrics_ts`** (history).

---

## 8. Stage A → Stage B notes

- Stage A (SQLite): `INTEGER PRIMARY KEY` ids, money as **decimal strings**,
  timestamps as RFC3339 **TEXT**, booleans as `INTEGER`. Cross-row money SUMs
  are display-only (do exact math in Rust with `U256`).
- Stage B (Postgres + Timescale): ids → `BIGINT … IDENTITY`; money →
  `NUMERIC(78,0)` (exact SUMs in SQL); timestamps → `TIMESTAMPTZ`; the
  `*_metrics_ts` tables → **hypertables**; `volume_rollups` → **continuous
  aggregate**. The `sqlx` query strings stay the same standard SQL.

---

## 9. Open questions

1. **Volume semantics** — is `volume_base` cumulative-since-inception or
   per-interval? (Window rollups are simpler if you store *interval* volume and
   SUM; cumulative needs last-minus-first.)
2. **Snapshot cadence** — how often does `intel-monitor` write a metrics row?
   (Drives data volume + how fine your regression diffs are. The `observed_at`
   spacing IS your diff resolution.)
3. **Token identity** — keep `tokens` as a JSON array attribute, or promote to a
   real `tokens` table + `market_tokens` link (needed only if you'll query
   "all markets containing USDC" relationally)?
4. **Per-chain rollup** — do you want `v_chain_totals` materialized into a
   `chain_metrics_ts` append-only table too (so chain-level history is queryable
   like the others), or is computing it on read enough?
```
