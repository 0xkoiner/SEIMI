CREATE TABLE IF NOT EXISTS protocols (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    display_name VARCHAR(50) NOT NULL,
    category VARCHAR(50) NOT NULL,
    abi_ref TEXT,
    watch BOOLEAN NOT NULL DEFAULT TRUE,
    capital_target BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS chains (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    chain_id BIGINT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS protocol_chains (
    protocol_id BIGINT NOT NULL REFERENCES protocols(id) ON DELETE CASCADE,
    chain_id BIGINT NOT NULL REFERENCES chains(id) ON DELETE CASCADE,
    PRIMARY KEY (protocol_id, chain_id)
);

CREATE TABLE IF NOT EXISTS markets (
    id BIGSERIAL PRIMARY KEY,
    protocol_id BIGINT NOT NULL REFERENCES protocols(id) ON DELETE CASCADE,
    chain_id BIGINT NOT NULL REFERENCES chains(id) ON DELETE CASCADE,
    address TEXT NOT NULL,
    market_type VARCHAR(50) NOT NULL,
    tokens TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    UNIQUE (chain_id, address)
);

CREATE TABLE IF NOT EXISTS market_metrics_ts (
    id BIGSERIAL PRIMARY KEY,
    market_id BIGINT NOT NULL REFERENCES markets(id) ON DELETE CASCADE,
    underlying TEXT,
    observed_at TIMESTAMPTZ NOT NULL,
    tvl_base NUMERIC(78,0) NOT NULL,
    volume_base NUMERIC(78,0) NOT NULL,
    apy_bps INTEGER,
    apr_bps INTEGER,
    source TEXT NOT NULL,
    trust_tier VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS protocol_metrics_ts (
    id BIGSERIAL PRIMARY KEY,
    protocol_id BIGINT NOT NULL REFERENCES protocols(id) ON DELETE CASCADE,
    observed_at TIMESTAMPTZ NOT NULL,
    tvl_base NUMERIC(78,0) NOT NULL,
    volume_base NUMERIC(78,0) NOT NULL,
    source TEXT NOT NULL,
    trust_tier VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS aggregate_metrics_ts (
    id BIGSERIAL PRIMARY KEY,
    observed_at TIMESTAMPTZ NOT NULL,
    total_tvl_base NUMERIC(78,0) NOT NULL,
    total_volume_base NUMERIC(78,0) NOT NULL,
    protocol_count INTEGER NOT NULL,
    source TEXT NOT NULL,
    trust_tier VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS volume_rollups (
    scope TEXT NOT NULL,
    scope_id BIGINT NOT NULL,
    window_label TEXT NOT NULL,
    volume_base NUMERIC(78,0) NOT NULL,
    computed_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (scope, scope_id, window_label)
);

CREATE INDEX IF NOT EXISTS idx_markets_protocol ON markets(protocol_id);
CREATE INDEX IF NOT EXISTS idx_markets_chain    ON markets(chain_id);
CREATE INDEX IF NOT EXISTS idx_mkt_ts   ON market_metrics_ts(market_id, observed_at);
CREATE INDEX IF NOT EXISTS idx_mkt_ts_underlying
    ON market_metrics_ts(market_id, underlying, observed_at)
    WHERE underlying IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_proto_ts ON protocol_metrics_ts(protocol_id, observed_at);
CREATE INDEX IF NOT EXISTS idx_agg_ts   ON aggregate_metrics_ts(observed_at);
