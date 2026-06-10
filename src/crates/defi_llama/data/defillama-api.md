# DefiLlama API — Internal Reference

A summary of DefiLlama's HTTP API, written for SEIMI engineers about to build the Rust connector. The full OpenAPI 3.1.1 spec lives next to this file at `defillama-api.yaml` (11,946 lines, 114 paths) — this doc is a 5-minute orientation, not a replacement.

## 1. Why we care

DefiLlama is the **tier-3 enrichment source** in the SEIMI intel plane (see `docs/plans/intel-plane/INTEL_PLANE.md`). Concrete needs:

- **USD prices** for Aave V1 reserves (and later V2/V3) so cross-reserve `tvl_base` sums in `market_metrics_ts` become meaningful.
- **Cross-protocol TVL & yield rankings** to seed the scoring component (`intel-scoring`).
- **DEX volume / fee / revenue series** for opportunity discovery.

Trust tier is **tier 3** — useful for ranking and watch-list discovery, but never authoritative for capital actions. Cross-check against tier-1 (on-chain via our own RPC) before acting.

## 2. Two APIs, one spec

| Aspect | Free API | Pro API |
|---|---|---|
| Base URL | `https://api.llama.fi` (and 3 sibling hosts) | `https://pro-api.llama.fi/{API_KEY}/...` |
| Auth | none | API key **in the URL path**, not a header |
| Endpoints in spec | 31 | 38 pro-exclusive + 31 free re-exposed under `/api/*` prefix |
| Rate limit | "Standard" (no published number — back off on 429) | "Higher" |
| Cost | $0 | $300/mo |

Pro key holders can also call free endpoints under the `pro-api.llama.fi` host (path differs — see "Free→Pro mapping" below) to get the higher rate limit. **Never** put a key on `api.llama.fi`; **never** call `pro-api.llama.fi` without one.

### The 5-host quirk

The top-level `servers:` in the YAML lists only `https://api.llama.fi`, but many free endpoints override it per-operation. Five distinct hosts:

| Host | Endpoints | Tag |
|---|---|---|
| `api.llama.fi` | `/protocols`, `/protocol/{p}`, `/tvl/{p}`, `/v2/historicalChainTvl[/{c}]`, `/v2/chains`, `/overview/{dexs,options,open-interest,fees}*`, `/summary/{dexs,options,fees}/{p}` | TVL, volumes, fees and revenue |
| `coins.llama.fi` | `/prices/{current,historical,first}/{coins}`, `/batchHistorical`, `/chart/{coins}`, `/percentage/{coins}`, `/block/{chain}/{ts}` | coins |
| `stablecoins.llama.fi` | `/stablecoins`, `/stablecoincharts/{all,chain}`, `/stablecoin/{asset}`, `/stablecoinchains`, `/stablecoinprices` | stablecoins |
| `yields.llama.fi` | `/pools`, `/chart/{pool}` | yields |
| `pro-api.llama.fi` | everything pro-exclusive | (all 6 tags) |

The connector must encode the **endpoint→host** mapping; do not assume a single base URL.

### Free→Pro path mapping (when a Pro key wants higher RL on free endpoints)

Full table is in `llms.txt`. Headline rule: prefix the free path with the tag's `/api/`, `/coins/`, `/stablecoins/`, or `/yields/` segment. Examples:

- `api.llama.fi/protocols` → `pro-api.llama.fi/{KEY}/api/protocols`
- `coins.llama.fi/prices/current/{coins}` → `pro-api.llama.fi/{KEY}/coins/prices/current/{coins}`
- `yields.llama.fi/pools` → `pro-api.llama.fi/{KEY}/yields/pools`

## 3. Endpoints SEIMI will actually use

Filtered down from 114 paths to the ~17 that map to the intel plane's needs. `[free]` / `[pro]` flags indicate auth requirement.

### TVL (host: `api.llama.fi`)

| Path | Status | Purpose | Key response fields |
|---|---|---|---|
| `GET /protocols` | [free] | List every protocol + current TVL | `id`, `name`, `symbol`, `category`, `chains[]`, `tvl`, `chainTvls{}`, `change_1d`, `change_7d` |
| `GET /protocol/{protocol}` | [free] | Historical TVL of one protocol, broken down by token & chain. `{protocol}` is the **slug**, e.g. `aave-v2`. | `tvl[]` (date/totalLiquidityUSD), `chainTvls{}`, `tokensInUsd[]`, `tokens[]` |
| `GET /tvl/{protocol}` | [free] | Just the current TVL number for one protocol | bare `number` (USD) |
| `GET /v2/chains` | [free] | Current TVL of every chain | `gecko_id`, `tvl`, `tokenSymbol`, `name`, `chainId` |
| `GET /v2/historicalChainTvl` | [free] | Aggregate historical TVL across all chains (excludes liquid staking & double-counting) | `[{date, tvl}]` |
| `GET /v2/historicalChainTvl/{chain}` | [free] | Same, per chain | `[{date, tvl}]` |

### Coins / Prices (host: `coins.llama.fi`)

**Coin key format:** `{chain}:{address}`, e.g. `ethereum:0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2`. Also accepts `coingecko:{id}` (`coingecko:ethereum`, `coingecko:bitcoin`).

| Path | Status | Purpose | Notes |
|---|---|---|---|
| `GET /prices/current/{coins}` | [free] | Current USD price for one or many tokens | `coins` is comma-separated. Optional `?searchWidth=4h`. Response carries a `confidence` float per coin — filter `< 0.9` if precision matters. |
| `GET /prices/historical/{timestamp}/{coins}` | [free] | Same shape, at a specific Unix timestamp | `{timestamp}` is seconds. `searchWidth` default 6h. |
| `GET /batchHistorical?coins=...` | [free] | Many tokens × many timestamps in one shot | `coins` is a **JSON-encoded query string**: `{"ethereum:0x...": [ts1, ts2], "coingecko:eth": [ts3]}` |
| `GET /chart/{coins}` | [free] | Price candles at regular intervals | useful for filling backfill gaps |
| `GET /prices/first/{coins}` | [free] | Earliest known price timestamp for each coin | for discovering "since when" data exists |
| `GET /block/{chain}/{timestamp}` | [free] | Nearest block number to a Unix timestamp | useful for aligning Aave snapshots to a wall-clock |

**Critical for the Aave V1 wiring:** we already hold `Vec<Address>` for the 22 reserves. Build the coin key as `format!("ethereum:{addr}", addr = reserve)` — alloy's `Address::to_string` produces an EIP-55 mixed-case hex, which DefiLlama accepts (the API lowercases internally).

### Yields / APY (host: `yields.llama.fi`)

| Path | Status | Purpose | Key response fields |
|---|---|---|---|
| `GET /pools` | [free] | Every yield pool, latest snapshot. ~10k pools. | `chain`, `project`, `symbol`, `tvlUsd`, `apy`, `apyBase`, `apyReward`, `pool` (UUID), `apyPct1D`, `apyPct7D`, `stablecoin`, `ilRisk`, `exposure`, `predictions{}` |
| `GET /chart/{pool}` | [free] | Historical APY + TVL of one pool. `{pool}` is the UUID from `/pools`. | `[{timestamp, tvlUsd, apy}]` |

### Stablecoins (host: `stablecoins.llama.fi`)

| Path | Status | Purpose |
|---|---|---|
| `GET /stablecoins?includePrices=true` | [free] | List all tracked stablecoins, circulation, peg type, chain breakdown |
| `GET /stablecoin/{asset}` | [free] | Historical mcap & chain distribution for one stablecoin (`{asset}` = `id` from `/stablecoins`) |
| `GET /stablecoinchains` | [free] | Current stablecoin mcap totals per chain |

### Volumes (host: `api.llama.fi`)

| Path | Status | Purpose |
|---|---|---|
| `GET /overview/dexs` | [free] | All DEXes with volume summaries |
| `GET /overview/dexs/{chain}` | [free] | Same, filtered to one chain |
| `GET /summary/dexs/{protocol}` | [free] | Per-protocol DEX volume history |
| `GET /overview/open-interest` | [free] | Perp-DEX open interest snapshot |

### Fees & Revenue (host: `api.llama.fi`)

| Path | Status | Purpose | Notes |
|---|---|---|---|
| `GET /overview/fees` | [free] | All protocols, fee/revenue summary | Required query: `excludeTotalDataChart=true`, `excludeTotalDataChartBreakdown=true`. Optional `?dataType=` ∈ `{dailyFees, dailyRevenue, dailyHoldersRevenue}` (default `dailyFees`). |
| `GET /overview/fees/{chain}` | [free] | Same, filtered to one chain |
| `GET /summary/fees/{protocol}` | [free] | Per-protocol fee/revenue history |

## 4. Pro-only — available if we upgrade

One-liners only; details live in the spec. Worth knowing they exist so the scoring layer can call them later without a research round.

- **TVL extras**: `/api/tokenProtocols/{symbol}`, `/api/inflows/{protocol}/{timestamp}`, `/api/chainAssets`
- **Token emissions**: `/api/emissions`, `/api/emission/{protocol}`
- **Protocol analytics**: `/api/categories`, `/api/forks`, `/api/oracles`, `/api/hacks`, `/api/raises`, `/api/treasuries`, `/api/entities`
- **Token liquidity**: `/api/historicalLiquidity/{token}`
- **Bridges**: `/bridges/{bridges,bridge/{id},bridgevolume/{chain},bridgedaystats/{ts}/{chain},transactions/{id}}`
- **ETFs**: `/etfs/snapshot`, `/etfs/flows`
- **Yield extras**: `/yields/poolsBorrow`, `/yields/chartLendBorrow/{pool}`, `/yields/perps`, `/yields/lsdRates`
- **Equities & RWA**: `/equities/v1/*`, `/rwa/*`
- **DAT (digital-asset treasuries)**: `/dat/institutions[/{symbol}]`
- **Generic metrics catalogue**: `/api/v2/metrics/{metric}` + `/api/v2/chart/{metric}` with `{chain}` / `{category}` / `{protocol}` sub-paths. `{metric}` is a path enum (e.g. `tvl`, `fees`, `revenue`, `volume`, `treasury` — full list in the spec). This is the biggest pro-only family (~50 paths) and gives uniform breakdown queries.

## 5. Response-shape conventions

- All responses are JSON.
- **Mixed case**: many fields are camelCase (`tvlUsd`, `chainTvls`, `apyBase`), some are snake_case with leading underscore (`change_1d`, `change_7d`), some lowercase (`tvl`, `apy`). Don't assume a single style — model each response type explicitly.
- **`chainTvls`** in `/protocols` and `/protocol/{p}` is `Record<chainName, number>` (object map, not array).
- **`change_1d` / `change_7d`** are signed **percent** (not bps): `2.1` means +2.1%.
- **Timestamps** are Unix seconds.
- **Nullable fields**: many APY/reward fields use `["number","null"]` union types — handle as `Option<f64>` in Rust.
- **`confidence`** on price responses: 0.0–1.0 float. The docs recommend filtering low-confidence prices for illiquid tokens.
- **404** is the standard response for unknown protocol slug / asset / pool UUID. Surface as a domain error, not a parse error.

## 6. Auth & rate limits

- **Free**: no auth header, no API key. The "Standard" rate limit isn't published; observed limits are roughly ~30 req/s, but the official guidance is "back off on 429" rather than rely on a budget. Use exponential backoff.
- **Pro**: API key is part of the URL path (`https://pro-api.llama.fi/{KEY}/...`). **Do not** log full URLs at INFO level — they leak the key. Store the key in `.env` (consistent with `DATABASE_URL`), read once at startup, never echo.

## 7. SDKs

JS (`@defillama/api`) and Python (`defillama-sdk`) are published. **No official Rust SDK** — SEIMI will roll a minimal `reqwest`-based client and only generate types for the endpoints we actually use.

## 8. Recommended Rust connector shape (preview, not commitment)

For the next planning round:

- Single `DefiLlamaClient` struct holding a `reqwest::Client` and an `Option<ApiKey>`. Five base URLs baked in as `const &str`.
- Per-endpoint methods typed against the response (e.g. `get_protocols() -> Vec<ProtocolSummary>`, `get_prices_current(coins: &[CoinKey]) -> HashMap<CoinKey, PriceQuote>`).
- A `CoinKey` newtype to keep `ethereum:0x…` strings well-formed; `From<(chain, Address)>` impl.
- Backoff layer (`tower-retry` or hand-rolled): retry on 429 and 5xx with jitter; surface 404 as a domain error.
- Errors as a `thiserror` enum matching the per-module convention already used in `parser::aave` and `db`: `DefiLlamaError` with `Transport`, `BadStatus(u16)`, `Decode`, `NotFound`, etc.
- No `include_str!` of the YAML unless we actually want to validate requests against it at compile time (probably overkill).

## 9. Source links

- Spec on disk: `src/crates/defi_llama/data/defillama-api.yaml`
- LLM index on disk: `src/crates/defi_llama/data/llms.txt`
- Hosted docs: <https://api-docs.defillama.com/>
- Free OpenAPI JSON: <https://api-docs.defillama.com/defillama-openapi-free.json>
- Pro OpenAPI JSON: <https://api-docs.defillama.com/defillama-openapi-pro.json>
- Pricing page: <https://defillama.com/pro-api>
