# DeFi Machine — Data Layer Plan

How and where every piece of data lives. The guiding split:

> **Live decision loop → in-process Rust memory (never a DB).**
> **History, research, simulation, audit → a database.**
> **Shared live state across processes (later) → Redis.**

A database in the hot path would add network + serialization latency to the one thing that must be fast. A database is for data that must *outlive a process* and be *queried later*.

---

## 1. The workloads, mapped to storage

| # | Workload | Access pattern | Lives in | Why |
|---|----------|----------------|----------|-----|
| 1 | **Hot state / world model** (positions, live pool reserves, prices, derived risk) | Read on every decision tick, microsecond latency, consistent snapshot | **In-process Rust memory** (structs/`Arc<RwLock<…>>`) | The brain is a pure fn over `WorldState`. Fastest possible read, no I/O. DB only for *durability* (periodic snapshot). |
| 2 | **Event log** (every event the system observes/emits) | Append constantly, never update, replay sequentially by time | **DB, append-only, time-partitioned** | Spine of audit + deterministic backtest. Immutable rows. |
| 3 | **Time-series / market data** (prices, pool states over time, funding, PnL curve) | Heavy writes, queried by time windows + aggregations | **DB, time-series optimized** | Research, vol calc, backtest inputs. Billions of rows at scale. |
| 4 | **Relational / config** (strategy defs, adapters, token allowlist, execution records w/ tx hashes) | Low volume, relational queries, needs integrity | **DB, ACID/SQL** | "All rebalances on Aave last month with realized slippage." |
| 5 | **Shared live state** (only if multi-process later) | Fast cross-process read/write | **Redis** (deferred) | Not needed while the machine is one process. |

**Key point:** workloads 2, 3, 4 are all *database* concerns and all fit one SQL engine well. Workload 1 is *not* a DB concern. Workload 5 is a *future* concern.

---

## 2. What does NOT go in a database

- **The world model the brain reads.** Stays in Rust memory. The DB only receives periodic *snapshots* of it for crash recovery.
- **The live hot path.** Ingest → state update → decide → simulate → submit never blocks on a DB query. Writes to the DB (event log, snapshots) happen on a *separate async task* off the critical path, so disk/DB latency can never stall a decision.
- **Secrets / keys.** OS keyring or encrypted local store — never a DB column.

---

## 3. Redis — where it fits, and where it does not

Redis is tempting as a "fast layer," but in this design the hot state is **already** in-process Rust memory, which beats Redis (no network hop, no serialize/deserialize). So:

- **v1 (single process): no Redis.** It would be slower than what we already have.
- **Later (multi-process):** if the machine splits — e.g. separate ingestion service, brain service, execution service — they need a shared live view. *Then* Redis (or a Rust-native alternative) becomes the shared hot-state layer / pub-sub bus between processes.
- **Alternatives to keep in mind for that role:** NATS (message bus, excellent for the event stream between services), or a Rust-embedded approach. Decide at the point you actually split processes, not before.

**Decision: leave a clean seam (a `StateStore` trait) so swapping in-memory → Redis/NATS later is a trait impl, not a rewrite. Do not deploy Redis in v1.**

---

## 4. The database engine — two-stage plan

### Stage A — build / sim / research (Phase 0–3, no real money): **SQLite**
- Single file, **zero server to run** → the machine is a self-contained binary.
- Real SQL → queries transfer directly to Postgres later (same SQL via `sqlx`).
- Crash-safe in **WAL mode**.
- Removes the entire "is my DB server configured / running / secured" problem class while the focus is getting **state-tracking correct** (the Phase 0 gate).
- Rust: **`sqlx`** with compile-time-checked queries (catches schema drift at build time).

### Stage B — live + scaling to server (Phase 4+): **PostgreSQL + TimescaleDB**
- **Postgres** = ACID relational home for workload #4.
- **TimescaleDB** (extension) makes it a genuine time-series DB for #2 and #3: hypertables, automatic time-partitioning, continuous aggregates, compression. Not a hack — proper TSDB behavior inside Postgres.
- One engine to operate/back up/monitor → **operational simplicity is a reliability & security property for a solo self-hoster.**
- Migration from Stage A is **schema + connection-string work, not a rewrite**, *because* we wrote standard SQL through `sqlx` from day one.
- Access: **`sqlx`** (same library, swap the driver).

### Deferred — analytical scale: **ClickHouse**
- Earns its place only when analytical query load / data volume actually strains Timescale (a "good problem" milestone).
- The event log is designed (append-only, time-partitioned, immutable) so it can be **exported/replicated to ClickHouse cleanly** when that day comes.
- **Do not deploy in v1.**

### Why not "just one specialist DB now"
At <$100k personal scale (dozens of pools/positions), write volume is *nowhere near* where Postgres+Timescale struggles — you'd need millions of writes/sec. Reaching for ClickHouse/specialist engines now optimizes a scale problem you don't have, at the cost of the operational simplicity you *do* need. Match the tool to the actual scale.

---

## 5. The one schema rule that keeps every option open

**The event log is append-only, time-partitioned, immutable from day one.**

This single constraint is what makes all future paths work:
- Timescale hypertables assume it.
- ClickHouse export assumes it.
- Deterministic replay (backtest + audit) assumes it.

If you ever want to `UPDATE` a row in the event log, something is wrong in the design. Corrections are *new events*, never edits.

---

## 6. The abstraction seam (so Stage A → B → Redis is cheap)

Define storage behind Rust traits in `persistence/`, so engines swap by impl:

```text
trait EventLog        { append(event); replay(time_range) -> Stream<Event>; }
trait SnapshotStore   { save(world_state); load_latest() -> WorldState; }
trait MarketStore     { write_ts(metric, ts, value); query_window(...); }
trait ConfigStore     { strategies(); adapters(); allowlist(); ... }
trait StateStore      { /* in-mem now; Redis/NATS impl later */ }
```

- Stage A: all backed by SQLite (+ in-mem `StateStore`).
- Stage B: `EventLog`/`MarketStore` → Timescale hypertables, `ConfigStore` → Postgres tables.
- Multi-process day: `StateStore` → Redis/NATS impl. No call-site changes.

---

## 7. Data flow (writes are OFF the hot path)

```
ingest ─▶ update in-mem WorldState ─▶ brain decides ─▶ simulate ─▶ submit
   │                  │                                              │
   └──────────────────┴───────────── (async, non-blocking) ──────────┘
                                  ▼
                        write task: event log + market TS + periodic snapshot
                                  ▼
                          SQLite (Stage A) / Timescale (Stage B)
```

The decision loop never `await`s a DB write. A backpressure-bounded channel feeds the write task; if the DB stalls, the machine keeps deciding and the queue absorbs it (with an alert if the queue grows).

---

## 8. Retention (drives how soon Stage B is needed)

Open question that sets the volume curve:
- **Lean:** retain only *your own* events + periodic market snapshots → embedded SQLite carries you far.
- **Heavy:** retain *every pool state every block* for rich backtesting → pulls toward Timescale sooner and toward ClickHouse eventually.

Recommendation: start **lean** (your events + 1-minute market snapshots of pools you touch). Add finer-grained capture per-pool only where a strategy needs it. Raw full-history capture is a deliberate later decision with its own storage budget.

---

## 9. Concrete next step

Write the schema for:
1. `events` — append-only event log (the spine).
2. `market_ts` — time-series market data.
3. `executions` — tx records (intent, calldata hash, tx hash, sim vs realized outcome, gas, slippage).
4. `positions_snapshot` — periodic world-model snapshots for recovery.
5. `config` tables — strategies, adapters, token allowlist.

Written as **SQL that runs on both SQLite and Postgres**, so the Stage A → B migration is real and not aspirational. Timescale-specific bits (hypertable creation, continuous aggregates) layered as Stage-B-only migrations.
```
