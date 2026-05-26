# DeFi Machine — v1 Architecture & Build Plan

**Profile (locked in):** Mixed multi-strategy · Ethereum L1 first, then Arb/Base/OP · <$100k personal capital · risk-management + routing brain (you make deploy decisions) · autonomous execution with hard circuit breakers + kill switch · MEV-protected via private orderflow, fast emergency-exit path.

**Stack:** Rust workspace · Alloy (`ProviderBuilder::new()`, `sol!` + `#[sol(rpc)]`) · revm for pre-send simulation · self-hosted Reth over IPC · tokio supervised tasks · fixed-point money math.

---

## 1. Design principles (the rules that keep funds safe)

1. **The brain is a pure function.** `fn decide(state: &WorldState) -> Vec<Intent>` — no network, no clock, no hidden randomness, no I/O. This makes it fully backtestable, fuzzable, and reviewable. All time/randomness is passed in as data.
2. **The safety plane is independent and supreme.** It runs as a separate supervised task with its own signing path. If the brain panics, deadlocks, or misbehaves, safety can *still* execute an emergency withdraw without asking. It never waits on the decision plane.
3. **Never broadcast an unsimulated transaction.** Every outgoing tx is simulated with revm against current state first. If it would revert, get sandwiched beyond tolerance, or breach a risk limit, it is dropped.
4. **The machine can never spend the cold reserve.** Emergency-withdraw target is an address the execution keys cannot move funds *from*. Hot balance = only what's needed to operate.
5. **No floats for money.** Fixed-point (U256 wei / `rust_decimal`) everywhere value is tracked. f64 only for display.
6. **Everything is an event, and every event is logged.** Append-only event log → deterministic replay → you can reconstruct exactly why the machine did anything. This is your audit trail and your backtest fuel.

---

## 2. The five planes

```
                ┌───────────────────────────────────────────────┐
   off-chain ──▶│  INGESTION   Reth IPC · mempool · CEX WS feeds│
   feeds        └───────────────────────┬───────────────────────┘
                                        │ normalized events
                                        ▼
                ┌───────────────────────────────────────────────┐
                │  STATE       in-memory world model            │
                │              positions · pools · prices       │◀── snapshot/recover
                └────────────────────────┬──────────────────────┘
                          reads          │
            ┌────────────────────────────┼─────────────────────────────┐
            ▼                            ▼                             ▼
 ┌──────────────────┐      ┌─────────────────────────┐    ┌─────────────────────┐
 │  BRAIN (pure)    │      │  SAFETY  (independent,  │    │  PRICING / RISK     │
 │  sizing · rebal  │      │  supreme, own signer)   │    │  pool math · MTM ·  │
 │  routing · target│      │  breakers · e-withdraw  │    │  vol · health       │
 └────────┬─────────┘      │  kill switch            │    └─────────────────────┘
          │ intents        └───────────┬─────────────┘
          ▼                            │ override / e-withdraw intent
 ┌────────────────────────────────────────────────────────────────┐
 │  EXECUTION   intent → tx · revm sim · routing · gas · nonce    │
 │              private orderflow (Flashbots/MEV-Share) · retries │
 └────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼  alerting (telegram/discord/pagerduty)
```

- **Ingestion** — Reth over IPC for blocks + state diffs + mempool; websocket feeds for off-chain data (CEX mid prices for basis, funding rates, stablecoin pegs). Normalizes everything into one internal event stream.
- **State** — single source of truth: your positions, relevant pool reserves/ticks, prices, derived risk. Snapshotted to disk so a crash recovers in seconds, not by re-scanning chain.
- **Pricing/Risk** — pool math (UniV2/V3, Curve, Aave/Compound rates), mark-to-market, volatility, position health factors. Feeds both brain and safety.
- **Brain** — *pure*. Vol-targeted/risk-parity sizing across your allocated strategies, cost-aware rebalancing (don't churn gas), slippage-aware route selection. Emits intents, never touches network.
- **Execution** — intent → calldata → revm simulate → private submission. Nonce manager, gas/priority strategy, retry/replace logic, sandwich-tolerance check.
- **Safety** — parallel, highest priority, independent signer. Watches pegs, oracle deviation, TVL drops, position health, exploit signals. Can fire emergency withdraw and flip the kill switch regardless of brain state.

---

## 3. Workspace layout

```
defi-machine/
├── crates/
│   ├── core-types/     # Position, Pool, Intent, Asset, WorldState
│   ├── ingestion/      # Reth IPC client, mempool sub, off-chain WS
│   ├── state/          # world model + snapshot/recovery
│   ├── pricing/        # pool math, MTM, vol, health metrics
│   ├── brain/          # decision plane — PURE, no I/O
│   ├── execution/      # intent→tx, revm sim, routing, gas, nonce, private submit
│   ├── safety/         # breakers, emergency withdraw, kill switch
│   ├── alerting/       # telegram/discord/pagerduty
│   └── persistence/    # append-only event log + position store (sqlite→postgres)
├── bins/
│   ├── machine/        # orchestrator daemon (wires the planes)
│   ├── backtest/       # replay historical state through brain/
│   └── cli/            # ops: status, pause, force-withdraw, inspect
└── ARCHITECTURE.md
```

---

## 4. Phased plan

Each phase has an explicit **safety gate** — you do not proceed until it passes. Money does not move until Phase 4, and only on testnet/tiny size until Phase 5.

### Phase 0 — Read-only Observatory  *(no money moves)*
Build `core-types`, `ingestion`, `state`, `pricing`, `persistence`. Connect to Reth over IPC, ingest blocks + your known positions, build the world model, compute live MTM and risk metrics, and **display them** (CLI/TUI or a tiny local web view). Append every event to the log.
**Gate:** the machine tracks "what do I own and what's it worth right now" with 100% accuracy vs manual on-chain check, across a full day, surviving a restart (snapshot recovery works). *If state is wrong, nothing downstream is trustworthy — this gate is the most important one.*

### Phase 1 — The Brain, offline  *(no money moves)*
Build `brain/` as a pure function and the `backtest` bin that replays the Phase-0 event log through it. Implement sizing (vol-target/risk-parity), cost-aware rebalance thresholds, and route selection. No execution yet — the brain just *prints the intents it would emit.*
**Gate:** run the brain over weeks of replayed history; every intent is explainable and sane; fuzz it with adversarial states (zero liquidity, depeg, stale price) and it never panics or emits a nonsensical intent.

### Phase 2 — Simulation-only Execution  *(no money moves)*
Build `execution/` through the revm-simulation step only. Intents become calldata, get simulated against forked current state, and the predicted outcome (output amount, gas, slippage, revert/no-revert) is logged. **Nothing is broadcast.**
**Gate:** simulated outcomes match what actually happens on-chain when you manually execute a sample trade; sandwich-tolerance and revert checks correctly reject bad txs.

### Phase 3 — The Safety Plane  *(built before live execution, deliberately)*
Build `safety/` and `alerting/`. Independent task, independent signer, watchdog over the whole system. Implement emergency-withdraw logic (to cold address), circuit breakers (peg/oracle/TVL/health thresholds), and the kill switch. Test by **simulating** the trigger conditions and verifying it produces a correct emergency-exit tx (simulated, not sent) and fires alerts.
**Gate:** kill the brain process mid-run → safety still detects a synthetic depeg and produces+simulates the withdraw within one block-time; alerts arrive. Safety works when everything else is dead.

### Phase 4 — Live execution, testnet then tiny  *(real txs, trivial size)*
Turn on broadcasting via **private orderflow** (Flashbots Protect / MEV-Share). First on a testnet/fork, then mainnet with **dust-sized positions** ($50–$200). Full loop live: ingest → decide → simulate → private-submit → confirm → update state. Emergency path armed.
**Gate:** a full week of autonomous operation at dust size with zero incorrect executions, correct nonce/gas handling, and at least one *successfully drilled* emergency withdraw on testnet.

### Phase 5 — Scale up, harden
Gradually raise capital. Add L2s (Arb/Base/OP) — same code, new chain configs. Add hardware-wallet/Safe cold reserve. Add monitoring dashboards, alerting escalation, and a runbook. Optionally add pluggable signal layer on top of the risk chassis.
**Gate:** ongoing — capital only scales when the prior tier has run clean for a defined period.

---

## 5. Emergency-withdraw triggers (Phase 3 detail — confirm/expand these)

The safety plane should fire on any of:
- **Stablecoin depeg** beyond X bps from $1 on a pool you're in.
- **Oracle deviation** — on-chain oracle vs your independent CEX feed diverges beyond threshold (manipulation signal).
- **Position health** — lending health factor approaches liquidation band.
- **Pool TVL drop** — sudden large outflow from a pool you're in (exploit/rug signal).
- **Protocol pause / unexpected admin event** — governance or admin function fired on a protocol you're exposed to.
- **Manual kill switch** — you, from the CLI, instantly.

> Open question for you: do you want emergency withdraw to be *fully automatic* on these triggers, or automatic-with-alert-and-short-grace-window for the softer signals (e.g. 30s to manually veto a withdraw triggered by oracle deviation, which can false-positive)? Hard exploit signals should be instant; softer ones may warrant a veto window.

---

## 6. First week of actual work

1. Scaffold the Cargo workspace + `core-types` (Position, Pool, Asset, Intent, WorldState).
2. Stand up Reth locally, sync, confirm IPC access.
3. `ingestion`: subscribe to new blocks over IPC via Alloy; decode into events.
4. `state`: build the world model from your actual current positions; compute MTM.
5. `persistence`: append-only event log to sqlite.
6. Wire `bins/machine` to run ingestion→state and print a live position/MTM table.

That is Phase 0. Everything else builds on a world model you trust.

---

## 7. Open questions to finalize before Phase 0

1. **MEV framing** — confirm: private orderflow for all execution, normal latency for rebalance, aggressive priority + private submit for emergency exit, and *no* sub-block searcher-style hot path in v1. (Leave an architectural seam for in-process Reth if you might add latency-sensitive arb later?)
2. **Which protocols in v1?** Aave/Compound (lending), Uniswap V3 / Curve (LP)? Naming them now lets us scope the pricing/adapter work precisely.
3. **Emergency auto vs. veto-window** per trigger type (see §5).
4. **Alert channel** — Telegram bot is the usual fast choice for self-hosted; confirm.
5. **Cold reserve custody** — hardware wallet or Safe multisig as the emergency-withdraw destination?
