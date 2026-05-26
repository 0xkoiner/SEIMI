# DeFi Machine — Discovery & Intelligence Plane

The part of the system that *knows about the world of protocols*, so the brain can reason and the safety plane can watch. **Not** an MEV searcher — it races nothing. It gathers, structures, stores, simulates, and ranks.

**v1 posture (locked in): GATHERING-FIRST.** Accumulate clean structured data across many protocols → *then* run models + a ranking system on the accumulated store to surface opportunities. You cannot rank what you haven't been measuring.

**Data sources:** on-chain (tier-1, ground truth) + protocol APIs/subgraphs (tier-2) + off-chain DefiLlama/audits/forums (tier-3, enrichment only).

**Adapters:** config-driven for simple fork families, hand-written for complex protocols — same trait for both.

---

## 1. The core insight: three lifetimes + one judge, all separate

Four kinds of data/work, deliberately NOT one pipeline:

| Kind | Question it answers | Cadence | Trust | Component |
|------|--------------------|---------|-------|-----------|
| **Static / semantic** | *What is this protocol?* addresses, ABIs, fees, admin keys, audits | Rare, curated | High (human-gated) | `intel-registry` |
| **Live / quantitative** | *What is it doing now?* TVL, reserves, APY, utilization | Every block-ish | Verify at tier-1 | `intel-monitor` |
| **Risk / health** | *Is it safe now?* admin moves, governance, outflows, peg | Watch loop | Feeds safety | `intel-monitor` → signals |
| **What-if** | *What would happen if I…?* yield, slippage, exit liquidity | On demand | Sim, reproducible | `intel-scenario` |

And **separate from all collection**, a consumer that judges:

| **Scoring / ranking** | *Which opportunities are best?* | Re-run over stored history | Versioned, grounded in data | `intel-scoring` |

> **Why scoring is separate from collection:** collection runs continuously and dumbly (gather everything, store it well). Scoring runs *over* the accumulated store and can be re-run and improved without touching collection. Change the formula → re-run against all history → compare. Same purity principle as the brain: keep the judge separate from the observer.

---

## 2. The manual/automatic answer: tiered by trust

Neither purely manual nor purely automatic. **Manual gate on the way in, automatic everywhere after.**

- **Onboarding a protocol = MANUAL (curated).** You supply/approve canonical addresses, ABIs, config. Wrong address here = total loss, so it is human-gated at every capital scale. This populates `intel-registry` (the allowlist).
- **Gathering data on the broad watch list = AUTOMATIC.** Once you've told it "watch these," collection of state + signals + raw data runs unattended. (In v1 the watch list can be broad — gathering ≠ committing capital.)
- **Surfacing opportunities = AUTOMATIC, SUGGEST-ONLY.** The scoring engine ranks and surfaces; it never auto-activates a capital target.
- **Promoting an opportunity to an actual capital target = MANUAL.** You review the ranked candidate and promote it; promotion is itself a logged event.

> The rule that holds it together: **data flows into the *capital-target* set only through a human-gated promote step.** Gathering, monitoring, scoring all run free; nothing the brain can deploy into exists until you promote it.

---

## 3. Trust hierarchy (provenance is a first-class field)

Every stored data point carries **source + trust tier**:

- **Tier 1 — on-chain via your own Reth node.** Ground truth. The only tier allowed to *authorize* a capital action.
- **Tier 2 — protocol subgraphs / APIs.** Convenient, structured, trust-but-verify. Good for discovery and cross-checks.
- **Tier 3 — off-chain (DefiLlama, audit reports, governance forums).** Context and enrichment only. *Finds and explains*, never *authorizes*.

> Hard rule: **anything that influences a capital action must be verifiable at tier-1 before the machine acts.** DefiLlama can *tell you* a pool has $50M TVL; your node *confirms* it before a dollar moves.

---

## 4. Adapter pattern — the structural keystone

The rest of the system speaks one trait, never concrete protocols. No `if protocol == Aave` anywhere in the brain.

```rust
trait ProtocolAdapter {
    fn metadata(&self) -> ProtocolMeta;               // static knowledge
    fn fetch_state(&self, ctx) -> ProtocolState;      // live quantitative
    fn health_signals(&self, ctx) -> Vec<Signal>;     // risk
    fn build_intent(&self, action) -> Intent;         // deposit/withdraw/swap calldata
    fn simulate(&self, intent, fork) -> SimResult;    // what-if
}
```

**Config-driven vs hand-written (your "mix"):**
- **Fork families** (UniV2 clones, Compound forks): one generic adapter parameterized by a **config file** (addresses + ABI ref + declarative rules). The long tail of clones needs *zero new Rust* — just a config entry.
- **Complex protocols** (Curve math, unusual mechanics): **hand-written** adapter implementing the same trait.
- Both expose identical `ProtocolAdapter` to the system. A config-driven adapter is just a generic adapter reading a config.

```
intel-adapters/
├── families/
│   ├── univ2.rs        # generic, config-parameterized
│   ├── univ3.rs
│   └── compound_fork.rs
├── config/
│   ├── sushiswap.toml  # univ2 family + addresses (no Rust)
│   └── ...             # the long tail of clones
├── aave_v3.rs          # hand-written (complex)
└── curve.rs            # hand-written (complex math)
```

---

## 5. Component layout

```
crates/
├── intel-core/        # types: ProtocolMeta, ProtocolState, Signal, Candidate,
│                      #   ScenarioRun, Score + the ProtocolAdapter trait
├── intel-adapters/    # per-protocol modules (families+config | hand-written)
├── intel-registry/    # curated allowlist + manual promote/reject gate
├── intel-monitor/     # AUTO: poll watch list -> state_ts + signals (tier-1 primary)
├── intel-sources/     # tier-2/3 fetchers: subgraphs, DefiLlama, audits, forums
├── intel-scenario/    # what-if sim (revm + fork) -> scenario_runs
└── intel-scoring/     # CONSUMER: rank opportunities over stored history (versioned)
```

```
        ┌────────── intel-registry (curated, HUMAN-GATED capital targets) ────────────┐
        │   protocols · contracts · markets · trust tier · watch flag · target flag   │
        └───────────────┬───────────────────────────────────────┬─────────────────────┘
                watch set│                            capital-target set
        ┌────────────────┼───────────────┐                          │
        ▼                ▼               ▼                          ▼
┌──────────────┐ ┌──────────────┐ ┌───────────────┐         brain (deploys only
│ intel-monitor│ │ intel-sources│ │ intel-scenario│        into promoted targets)
│ tier-1 state │ │ tier-2/3     │ │ what-if sim   │
│ + signals    │ │ enrichment   │ │               │
└──────┬───────┘ └──────┬───────┘ └──────┬────────┘
       │ state_ts       │ enrich         │ scenario_runs
       ▼                ▼                ▼
   ┌──────────────────────────────────────────┐
   │      DATA STORE (the accumulated record) │
   └───────────────────┬──────────────────────┘
                       │ reads history
                       ▼
              ┌────────────────────┐      signals ─▶ safety plane
              │  intel-scoring     │
              │  rank opportunities│──▶ ranked candidates ─▶ YOU triage ─▶ promote
              │  (versioned models)│                          (back into registry)
              └────────────────────┘
```

---

## 6. DB tables for this plane

**Static / curated (relational, human-gated):**
- `protocols` — id, name, chain, category, governance model, admin addresses, audit status, trust tier, `watch` flag, `capital_target` flag, active.
- `contracts` — protocol_id, address, abi_ref, role (pool/router/oracle/controller), verified.
- `markets` — protocol_id, pool/pair, tokens, fee tier, decimals.
- `adapter_config` — protocol_id, family, config blob (for config-driven adapters).

**Live / time-series (high-write, automatic, Timescale hypertable in Stage B):**
- `protocol_state_ts` — protocol_id, market_id, ts, tvl, apy, utilization, reserves, **source, trust_tier**.

**Risk signals (event-like, feeds safety):**
- `protocol_signals` — protocol_id, ts, signal_type, severity, payload, **source, trust_tier**.

**Enrichment (tier-2/3, context):**
- `protocol_enrichment` — protocol_id, ts, kind (llama_tvl/audit/governance), payload, **source, trust_tier**.

**What-if research (reproducible):**
- `scenario_runs` — id, protocol_id, market_id, fork_block, hypothetical action, inputs, outcomes (yield/slippage/exit-liquidity/stress), model_version, ts.

**Scoring (versioned, re-runnable):**
- `opportunity_scores` — protocol_id, market_id, ts, score, rank, model_version, feature_snapshot. Re-running a new model_version appends; you never overwrite history.
- `discovery_candidates` — auto-found protocols/pools awaiting promote/reject, with gathered context.

> Every row that could touch a decision carries **source + trust_tier**. Append-only where it's history (state_ts, signals, scores, scenario_runs); mutable only for curated config (registry), and even those changes are logged as events.

---

## 7. The overlap with the ingestion plane — resolved deliberately

Both `ingestion` and `intel-monitor` read live on-chain state. Clean split:

- **`ingestion` (hot path):** ONLY your positions + the pools you're actually in. Lowest latency, feeds the live decision loop and the in-memory world model.
- **`intel-monitor` (off hot path):** the broad watch list — protocols you're evaluating or guarding but not (yet) deployed into. Feeds the data store + safety, not the hot decision loop.

> Same underlying Reth/Alloy read code (shared crate), two different consumers with different latency budgets. Build the reader once; point it at two scopes. Do **not** build two pollers.

---

## 8. Build order within this plane

1. **`intel-core` + `ProtocolAdapter` trait** — the contract everything else implements.
2. **`intel-registry`** — curated allowlist + promote/reject gate (manual onboarding works end to end).
3. **One hand-written adapter (e.g. Aave V3) + one family (UniV2 + a config clone)** — prove the mix pattern.
4. **`intel-monitor`** — gather tier-1 state + signals for the watch list into the store. *This is the gathering engine; get it running early so data accumulates while you build the rest.*
5. **`intel-sources`** — tier-2/3 enrichment (subgraph, DefiLlama, audits/forums), tagged with provenance.
6. **`intel-scenario`** — what-if sim over forked state.
7. **`intel-scoring`** — only after weeks of accumulated data exists to rank against. Versioned models, re-runnable over history.

> Step 4 first among the engines, on purpose: **start accumulating data the moment the registry + one adapter exist**, because scoring (step 7) is worthless until there's history to score. Every day of delayed gathering is a day of ranking data you'll never get back.

---

## 9. Open questions to finalize

1. **Watch-list breadth in v1** — a curated few dozen protocols, or cast wide (hundreds via config-driven families) to maximize gathered data for later ranking? (Wide gathering is cheap and feeds scoring better; the human gate on *capital targets* keeps it safe.)
2. **Scoring dimensions** — what does "opportunity" mean to you? risk-adjusted yield, yield stability, exit liquidity, protocol safety score, or a composite? (We can start with a transparent composite and let it evolve once data exists.)
3. **Scenario stress scenarios** — which standard shocks to model? (e.g. ±20% price move, 50% liquidity withdrawal, depeg to $0.95, gas spike.)
4. **Off-chain source list** — confirm DefiLlama + audit registries + which governance forums; any protocol APIs you specifically want.
```
