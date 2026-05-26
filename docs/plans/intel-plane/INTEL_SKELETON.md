# Intel Plane — Walking Skeleton (Implementation Notes)

Companion to `INTEL_PLANE.md` (the plan). This documents the **walking skeleton
that was actually built** — the thin end-to-end slice that runs — plus the
concrete decisions baked into it and how to take it forward. No code is
reproduced here; this is the map.

> Status: skeleton written, statically verified (imports/fields/exports
> consistent, unused imports removed). **Not yet compiled** — needs `cargo check`
> on a machine with the Rust toolchain. Two Alloy-version-sensitive spots are
> isolated to single fix-points (see §6).

---

## 1. What the skeleton proves

A walking skeleton (Cockburn's term): the thinnest end-to-end slice that *runs*
and exercises every architectural seam, so the abstractions are validated before
real logic is layered on. This one runs:

```
registry (Aave V3 adapter)
     │
     ▼
monitor.run_once ── reads via ChainReader (live Alloy OR mock) ──▶ store (SQLite, append-only)
     │
     ▼
print a pass report (protocols/markets/states/signals/errors)
```

It validates four things that matter more than any single feature:

1. **The `ProtocolAdapter` trait is shaped right** — proven against the *complex*
   case (Aave V3, hand-written), not an easy AMM, so it won't silently over-fit.
2. **The `ChainReader` seam works** — the same adapter runs against live chain
   data or a deterministic mock, selected by config.
3. **The registry's watch/capital-target split holds** — gather freely, gate capital.
4. **The append-only store round-trips** — provenance-tagged rows persist and
   accumulate across runs.

---

## 2. Crate layout (as built)

A standalone Cargo workspace (`intel-plane/`), to be merged into the full
`defi-machine` monorepo later.

| Crate | Role | Depends on |
|-------|------|-----------|
| `intel-core` | Shared vocabulary: the `ProtocolAdapter` + `ChainReader` traits, domain types (`ProtocolMeta`, `ProtocolState`, `Signal`, `Provenance`, `TrustTier`…), typed errors. **Depends on nothing else in the plane.** | (leaf) |
| `intel-adapters` | `AaveV3Adapter` (hand-written, `sol!`-based) + the two `ChainReader` impls: `AlloyReader` (live) and `MockReader` (deterministic). | intel-core |
| `intel-registry` | Holds boxed adapters; exposes `watch_set()` and `capital_targets()` (the human-gated split). | intel-core |
| `intel-store` | Stage-A SQLite persistence; append-only state + signals; portable SQL. | intel-core |
| `intel-monitor` | The gathering loop: registry × reader → store. One pass = `run_once`. | core, registry, store |
| `intel-skeleton` (bin) | Wires it all together; env-selects reader; prints the pass report. | all of the above |

Dependency graph is a clean DAG with `intel-core` as the single leaf — this is
what keeps the abstractions honest.

---

## 3. Decisions baked into the skeleton

These were settled during the build and are now reflected in code structure:

- **First adapter = Aave V3, hand-written.** The "complex" case, chosen
  deliberately to stress-test the trait. Reserves are discovered on-chain
  (dynamic `markets()`), state needs real decoding, and it has natural health
  signals (utilization spike).
- **Data source = hosted RPC now → own Reth node before real capital.** Reth and
  "RPC" are not opposites: RPC is the protocol, Reth is whose node you talk to.
  Hosted wins on time-to-first-run; your own node wins on no-rate-limits,
  tier-1 ground truth, mempool access, and IPC latency for the emergency path.
- **Transport is auto-detected.** `AlloyReader::connect(endpoint)` uses Alloy's
  `.connect()`, which picks HTTP / WS / IPC from the endpoint format. So the
  single `ETH_RPC_URL` takes a hosted HTTPS URL today and a Reth IPC socket
  path later **with no code change**. This is the entire "migrate to Reth"
  story, made free up front.
- **Reader is env-selected with a mock fallback.** `ETH_RPC_URL` set → live;
  unset → deterministic `MockReader`. So `cargo run` works on any machine with
  zero setup, and CI never depends on a network.
- **Money is `U256`, never float.** Stored as exact decimal strings in SQLite
  (no native 256-bit int); becomes Postgres `NUMERIC(78,0)` in Stage B.
- **The plane only READS.** No signing/sending lives here — execution is a
  different plane. `ChainReader` deliberately exposes only `call` + `block_number`.

---

## 4. Design rules embodied (traceable to the plan)

| Rule (from INTEL_PLANE.md / ARCHITECTURE.md) | Where it lives in the skeleton |
|----------------------------------------------|-------------------------------|
| `ProtocolAdapter` is the only seam the system speaks | `intel-core::adapter` — no `if protocol == X` anywhere |
| `ChainReader` decouples adapters from data source | live/mock both impl it; adapter is reader-agnostic |
| Provenance + trust tier on every datum | `Provenance`/`TrustTier` on `ProtocolState` & `Signal`; only `OnChain` authorizes capital |
| Store is append-only | `intel-store` has `append_state`/`append_signal` and deliberately **no** update/delete |
| `capital_target` is human-gated | `Registry::capital_targets()` filters a flag that is never auto-set |
| Gather freely, gate capital | `watch=true` drives the monitor; `capital_target=false` by default |
| Writes off the hot path | monitor writes to store; the (future) live loop never blocks on it |

---

## 5. How it runs

```bash
cd intel-plane
cargo run -p intel-skeleton                                  # mock, zero setup
ETH_RPC_URL="https://eth-mainnet.../v2/KEY" cargo run -p intel-skeleton   # hosted, live
ETH_RPC_URL="/tmp/reth.ipc"                  cargo run -p intel-skeleton   # Reth, later
```

Re-running increments the stored row count — proof of append-only persistence
(`intel.db` in the working dir).

---

## 6. Known fix-points before first green build

The skeleton targets **Alloy 1.7** (`ProviderBuilder::new()`, `sol!`, no
ethers-rs). Two spots vary across Alloy releases and are the only likely
`cargo check` snags; each is isolated to one place:

1. **`abi_decode_returns` arity** — single fix-point: the `decode_returns!`
   macro at the top of `intel-adapters/src/aave_v3.rs`. Shipped as the 1.x
   one-arg form; if the build expects two args, change only that macro body.
2. **Provider `.call()` request builder** — in `intel-adapters/src/reader.rs`;
   affects only the *live* path (mock is unaffected, so the architecture runs
   regardless).

Neither changes the *shape* of the skeleton — only exact live-call syntax.

---

## 7. Next steps (per the plan's build order)

In priority order, matching the gathering-first posture:

1. **Interval loop + backpressure** around `monitor::run_once`, off the hot path
   — so data starts *accumulating continuously*. (Most in-spirit next step:
   scoring is worthless without history, so start the firehose early.)
2. **UniV2 fork-family adapter** (config-driven) + one clone — proves the *other*
   half of the "mix: config for simple, hand-written for complex" decision.
   Currently only the hand-written path is proven.
3. **`intel-sources`** — tier-2/3 enrichment (subgraphs, DefiLlama, audits,
   forums), each datum tagged with provenance.
4. **`intel-scenario`** — what-if simulation over forked state (revm).
5. **`intel-scoring`** — only after weeks of accumulated history exist to rank
   against. Versioned, re-runnable models.
6. **Stage-B migration** — SQLite → Postgres + TimescaleDB when moving to a
   server / before real capital (per DATA_LAYER.md).

---

## 8. Document set (where this fits)

- `PLAN.md` — overall five-plane system architecture & phased build plan.
- `DATA_LAYER.md` — storage strategy (SQLite → Postgres+Timescale; Redis deferred).
- `INTEL_PLANE.md` — the Discovery & Intelligence plane design (the plan).
- **`INTEL_SKELETON.md`** — this doc: the walking skeleton as built.
- `README.md` (in `intel-plane/`) — build/run/verify instructions for the code.
