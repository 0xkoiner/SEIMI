use sqlx::types::chrono::Utc;

use crate::db::db_engine::sqlx_conn::DBEngine;
use crate::db::types::schema::{
    AggregateMetricsTs, Chains, MarketMetricsTs, Markets, ProtocolChains, ProtocolMetricsTs,
    Protocols, VolumeRollups,
};

impl DBEngine {
    pub async fn insert_protocols(
        &self,
        name: &str,
        display_name: &str,
        category: &str,
        abi_ref: Option<&str>,
    ) -> Result<Protocols, sqlx::Error> {
        let now = Utc::now();

        self.mutate_one(
            "INSERT INTO protocols (name, display_name, category, abi_ref, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, $6) \
             RETURNING id, name, display_name, category, abi_ref, watch, capital_target, created_at, updated_at",
            |q| {
                q.bind(name.to_owned())
                    .bind(display_name.to_owned())
                    .bind(category.to_owned())
                    .bind(abi_ref.map(|s| s.to_owned()))
                    .bind(now)
                    .bind(now)
            },
        )
        .await
    }

    pub async fn insert_chains(&self, name: &str, chain_id: i64) -> Result<Chains, sqlx::Error> {
        self.mutate_one(
            "INSERT INTO chains (name, chain_id) VALUES ($1, $2) RETURNING id, name, chain_id",
            |q| q.bind(name.to_owned()).bind(chain_id),
        )
        .await
    }

    pub async fn insert_protocol_chains(
        &self,
        protocol_id: i64,
        chain_id: i64,
    ) -> Result<ProtocolChains, sqlx::Error> {
        self.mutate_one(
            "INSERT INTO protocol_chains (protocol_id, chain_id) VALUES ($1, $2) RETURNING protocol_id, chain_id",
            |q| q.bind(protocol_id).bind(chain_id),
        )
        .await
    }

    pub async fn insert_markets(
        &self,
        protocol_id: i64,
        chain_id: i64,
        address: &str,
        market_type: &str,
        tokens: &str,
    ) -> Result<Markets, sqlx::Error> {
        let now = Utc::now();

        self.mutate_one(
            "INSERT INTO markets (protocol_id, chain_id, address, market_type, tokens, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6) \
             RETURNING id, protocol_id, chain_id, address, market_type, tokens, created_at",
            |q| {
                q.bind(protocol_id)
                    .bind(chain_id)
                    .bind(address.to_owned())
                    .bind(market_type.to_owned())
                    .bind(tokens.to_owned())
                    .bind(now)
            },
        )
        .await
    }

    #[warn(clippy::too_many_arguments)]
    pub async fn insert_market_metrics_ts(
        &self,
        market_id: i64,
        tvl_base: i64,
        volume_base: i64,
        apy_bps: i32,
        apr_bps: i32,
        source: &str,
        trust_tier: &str,
    ) -> Result<MarketMetricsTs, sqlx::Error> {
        let now = Utc::now();

        self.mutate_one(
            "INSERT INTO market_metrics_ts (market_id, observed_at, tvl_base, volume_base, apy_bps, apr_bps, source, trust_tier) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
             RETURNING id, market_id, observed_at, tvl_base, volume_base, apy_bps, apr_bps, source, trust_tier",
            |q| {
                q.bind(market_id)
                    .bind(now)
                    .bind(tvl_base)
                    .bind(volume_base)
                    .bind(apy_bps)
                    .bind(apr_bps)
                    .bind(source.to_owned())
                    .bind(trust_tier.to_owned())
            },
        )
        .await
    }

    pub async fn insert_protocol_metrics_ts(
        &self,
        protocol_id: i64,
        tvl_base: i64,
        volume_base: i64,
        source: &str,
        trust_tier: &str,
    ) -> Result<ProtocolMetricsTs, sqlx::Error> {
        let now = Utc::now();

        self.mutate_one(
            "INSERT INTO protocol_metrics_ts (protocol_id, observed_at, tvl_base, volume_base, source, trust_tier) \
             VALUES ($1, $2, $3, $4, $5, $6) \
             RETURNING id, protocol_id, observed_at, tvl_base, volume_base, source, trust_tier",
            |q| {
                q.bind(protocol_id)
                    .bind(now)
                    .bind(tvl_base)
                    .bind(volume_base)
                    .bind(source.to_owned())
                    .bind(trust_tier.to_owned())
            },
        )
        .await
    }

    pub async fn insert_aggregate_metrics_ts(
        &self,
        total_tvl_base: i64,
        total_volume_base: i64,
        protocol_count: i64,
        source: &str,
        trust_tier: &str,
    ) -> Result<AggregateMetricsTs, sqlx::Error> {
        let now = Utc::now();

        self.mutate_one(
            "INSERT INTO aggregate_metrics_ts (observed_at, total_tvl_base, total_volume_base, protocol_count, source, trust_tier) \
             VALUES ($1, $2, $3, $4, $5, $6) \
             RETURNING id, observed_at, total_tvl_base, total_volume_base, protocol_count, source, trust_tier",
            |q| {
                q.bind(now)
                    .bind(total_tvl_base)
                    .bind(total_volume_base)
                    .bind(protocol_count)
                    .bind(source.to_owned())
                    .bind(trust_tier.to_owned())
            },
        )
        .await
    }

    pub async fn insert_volume_rollups(
        &self,
        scope: &str,
        scope_id: i64,
        window_label: &str,
        volume_base: i64,
    ) -> Result<VolumeRollups, sqlx::Error> {
        let now = Utc::now();

        self.mutate_one(
            "INSERT INTO volume_rollups (scope, scope_id, window_label, volume_base, computed_at) \
             VALUES ($1, $2, $3, $4, $5) \
             RETURNING scope, scope_id, window_label, volume_base, computed_at",
            |q| {
                q.bind(scope.to_owned())
                    .bind(scope_id)
                    .bind(window_label.to_owned())
                    .bind(volume_base)
                    .bind(now)
            },
        )
        .await
    }

    pub async fn read_protocols_by_id(&self, id: i64) -> Result<Protocols, sqlx::Error> {
        self.single_read::<Protocols, _>("SELECT * FROM protocols WHERE id = $1", |q| q.bind(id))
            .await
    }

    pub async fn read_protocols_by_name(&self, name: &str) -> Result<Protocols, sqlx::Error> {
        self.single_read::<Protocols, _>("SELECT * FROM protocols WHERE name = $1", |q| {
            q.bind(name.to_owned())
        })
        .await
    }

    pub async fn read_chains_by_id(&self, id: i64) -> Result<Chains, sqlx::Error> {
        self.single_read::<Chains, _>("SELECT * FROM chains WHERE id = $1", |q| q.bind(id))
            .await
    }

    pub async fn read_chains_by_name(&self, name: &str) -> Result<Chains, sqlx::Error> {
        self.single_read::<Chains, _>("SELECT * FROM chains WHERE name = $1", |q| {
            q.bind(name.to_owned())
        })
        .await
    }

    pub async fn read_chains_by_chain_id(&self, chain_id: i64) -> Result<Chains, sqlx::Error> {
        self.single_read::<Chains, _>("SELECT * FROM chains WHERE chain_id = $1", |q| {
            q.bind(chain_id)
        })
        .await
    }

    pub async fn read_protocol_chains_by_protocol_id(&self, protocol_id: i64) -> Result<Vec<ProtocolChains>, sqlx::Error> {
        self.full_read::<ProtocolChains, _>(
            "SELECT * FROM protocol_chains WHERE protocol_id = $1",
            |q| q.bind(protocol_id),
        )
        .await
    }

    pub async fn read_protocol_chains_by_chain_id(&self, chain_id: i64) -> Result<Vec<ProtocolChains>, sqlx::Error> {
        self.full_read::<ProtocolChains, _>(
            "SELECT * FROM protocol_chains WHERE chain_id = $1",
            |q| q.bind(chain_id),
        )
        .await
    }

    pub async fn read_markets_by_id(&self, id: i64) -> Result<Markets, sqlx::Error> {
        self.single_read::<Markets, _>("SELECT * FROM markets WHERE id = $1", |q| q.bind(id))
            .await
    }

    pub async fn read_market_metrics_ts_by_id(&self, id: i64) -> Result<MarketMetricsTs, sqlx::Error> {
        self.single_read::<MarketMetricsTs, _>(
            "SELECT * FROM market_metrics_ts WHERE id = $1",
            |q| q.bind(id),
        )
        .await
    }

    pub async fn read_protocol_metrics_ts_by_id(&self, id: i64) -> Result<ProtocolMetricsTs, sqlx::Error> {
        self.single_read::<ProtocolMetricsTs, _>(
            "SELECT * FROM protocol_metrics_ts WHERE id = $1",
            |q| q.bind(id),
        )
        .await
    }

    pub async fn read_aggregate_metrics_ts_by_id(&self, id: i64) -> Result<AggregateMetricsTs, sqlx::Error> {
        self.single_read::<AggregateMetricsTs, _>(
            "SELECT * FROM aggregate_metrics_ts WHERE id = $1",
            |q| q.bind(id),
        )
        .await
    }
}
