use sqlx::types::chrono::Utc;

use crate::db::db_engine::sqlx_conn::DBEngine;
use crate::db::types::errors::DbError;
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
    ) -> Result<Protocols, DbError> {
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

    pub async fn insert_chains(&self, name: &str, chain_id: i64) -> Result<Chains, DbError> {
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
    ) -> Result<ProtocolChains, DbError> {
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
    ) -> Result<Markets, DbError> {
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
    ) -> Result<MarketMetricsTs, DbError> {
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
    ) -> Result<ProtocolMetricsTs, DbError> {
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
    ) -> Result<AggregateMetricsTs, DbError> {
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
    ) -> Result<VolumeRollups, DbError> {
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

    pub async fn read_protocols_by_id(&self, id: i64) -> Result<Protocols, DbError> {
        self.single_read::<Protocols, _>("SELECT * FROM protocols WHERE id = $1", |q| q.bind(id))
            .await
    }

    pub async fn read_protocols_by_name(&self, name: &str) -> Result<Protocols, DbError> {
        self.single_read::<Protocols, _>("SELECT * FROM protocols WHERE name = $1", |q| {
            q.bind(name.to_owned())
        })
        .await
    }

    pub async fn read_chains_by_id(&self, id: i64) -> Result<Chains, DbError> {
        self.single_read::<Chains, _>("SELECT * FROM chains WHERE id = $1", |q| q.bind(id))
            .await
    }

    pub async fn read_chains_by_name(&self, name: &str) -> Result<Chains, DbError> {
        self.single_read::<Chains, _>("SELECT * FROM chains WHERE name = $1", |q| {
            q.bind(name.to_owned())
        })
        .await
    }

    pub async fn read_chains_by_chain_id(&self, chain_id: i64) -> Result<Chains, DbError> {
        self.single_read::<Chains, _>("SELECT * FROM chains WHERE chain_id = $1", |q| {
            q.bind(chain_id)
        })
        .await
    }

    pub async fn read_protocol_chains_by_protocol_id(&self, protocol_id: i64) -> Result<Vec<ProtocolChains>, DbError> {
        self.full_read::<ProtocolChains, _>(
            "SELECT * FROM protocol_chains WHERE protocol_id = $1",
            |q| q.bind(protocol_id),
        )
        .await
    }

    pub async fn read_protocol_chains_by_chain_id(&self, chain_id: i64) -> Result<Vec<ProtocolChains>, DbError> {
        self.full_read::<ProtocolChains, _>(
            "SELECT * FROM protocol_chains WHERE chain_id = $1",
            |q| q.bind(chain_id),
        )
        .await
    }

    pub async fn read_markets_by_id(&self, id: i64) -> Result<Markets, DbError> {
        self.single_read::<Markets, _>("SELECT * FROM markets WHERE id = $1", |q| q.bind(id))
            .await
    }

    pub async fn read_market_metrics_ts_by_id(&self, id: i64) -> Result<MarketMetricsTs, DbError> {
        self.single_read::<MarketMetricsTs, _>(
            "SELECT * FROM market_metrics_ts WHERE id = $1",
            |q| q.bind(id),
        )
        .await
    }

    pub async fn read_protocol_metrics_ts_by_id(&self, id: i64) -> Result<ProtocolMetricsTs, DbError> {
        self.single_read::<ProtocolMetricsTs, _>(
            "SELECT * FROM protocol_metrics_ts WHERE id = $1",
            |q| q.bind(id),
        )
        .await
    }

    pub async fn read_aggregate_metrics_ts_by_id(&self, id: i64) -> Result<AggregateMetricsTs, DbError> {
        self.single_read::<AggregateMetricsTs, _>(
            "SELECT * FROM aggregate_metrics_ts WHERE id = $1",
            |q| q.bind(id),
        )
        .await
    }

    pub async fn read_all_protocols(&self) -> Result<Vec<Protocols>, DbError> {
        self.full_read::<Protocols, _>("SELECT * FROM protocols", |q| q).await
    }

    pub async fn read_all_chains(&self) -> Result<Vec<Chains>, DbError> {
        self.full_read::<Chains, _>("SELECT * FROM chains", |q| q).await
    }

    pub async fn read_all_markets(&self) -> Result<Vec<Markets>, DbError> {
        self.full_read::<Markets, _>("SELECT * FROM markets", |q| q).await
    }

    pub async fn read_market_metrics_ts_history(
        &self,
        market_id: i64,
    ) -> Result<Vec<MarketMetricsTs>, DbError> {
        self.full_read::<MarketMetricsTs, _>(
            "SELECT * FROM market_metrics_ts WHERE market_id = $1 ORDER BY observed_at DESC",
            |q| q.bind(market_id),
        )
        .await
    }

    pub async fn read_protocol_metrics_ts_history(
        &self,
        protocol_id: i64,
    ) -> Result<Vec<ProtocolMetricsTs>, DbError> {
        self.full_read::<ProtocolMetricsTs, _>(
            "SELECT * FROM protocol_metrics_ts WHERE protocol_id = $1 ORDER BY observed_at DESC",
            |q| q.bind(protocol_id),
        )
        .await
    }

    pub async fn read_aggregate_metrics_ts_history(
        &self,
    ) -> Result<Vec<AggregateMetricsTs>, DbError> {
        self.full_read::<AggregateMetricsTs, _>(
            "SELECT * FROM aggregate_metrics_ts ORDER BY observed_at DESC",
            |q| q,
        )
        .await
    }

    pub async fn read_market_metrics_ts_latest(
        &self,
        market_id: i64,
    ) -> Result<MarketMetricsTs, DbError> {
        self.single_read::<MarketMetricsTs, _>(
            "SELECT * FROM market_metrics_ts WHERE market_id = $1 ORDER BY observed_at DESC LIMIT 1",
            |q| q.bind(market_id),
        )
        .await
    }

    pub async fn read_protocol_metrics_ts_latest(
        &self,
        protocol_id: i64,
    ) -> Result<ProtocolMetricsTs, DbError> {
        self.single_read::<ProtocolMetricsTs, _>(
            "SELECT * FROM protocol_metrics_ts WHERE protocol_id = $1 ORDER BY observed_at DESC LIMIT 1",
            |q| q.bind(protocol_id),
        )
        .await
    }

    pub async fn read_aggregate_metrics_ts_latest(
        &self,
    ) -> Result<AggregateMetricsTs, DbError> {
        self.single_read::<AggregateMetricsTs, _>(
            "SELECT * FROM aggregate_metrics_ts ORDER BY observed_at DESC LIMIT 1",
            |q| q,
        )
        .await
    }

    pub async fn read_volume_rollups_by_scope(
        &self,
        scope: &str,
        scope_id: i64,
    ) -> Result<Vec<VolumeRollups>, DbError> {
        self.full_read::<VolumeRollups, _>(
            "SELECT * FROM volume_rollups WHERE scope = $1 AND scope_id = $2",
            |q| q.bind(scope.to_owned()).bind(scope_id),
        )
        .await
    }

    pub async fn read_chains_for_protocol(
        &self,
        protocol_id: i64,
    ) -> Result<Vec<Chains>, DbError> {
        self.full_read::<Chains, _>(
            "SELECT c.* FROM chains c \
             JOIN protocol_chains pc ON pc.chain_id = c.id \
             WHERE pc.protocol_id = $1",
            |q| q.bind(protocol_id),
        )
        .await
    }

    pub async fn read_protocols_on_chain(
        &self,
        chain_id: i64,
    ) -> Result<Vec<Protocols>, DbError> {
        self.full_read::<Protocols, _>(
            "SELECT p.* FROM protocols p \
             JOIN protocol_chains pc ON pc.protocol_id = p.id \
             WHERE pc.chain_id = $1",
            |q| q.bind(chain_id),
        )
        .await
    }

    pub async fn read_markets_by_protocol(
        &self,
        protocol_id: i64,
    ) -> Result<Vec<Markets>, DbError> {
        self.full_read::<Markets, _>(
            "SELECT * FROM markets WHERE protocol_id = $1",
            |q| q.bind(protocol_id),
        )
        .await
    }

    pub async fn read_markets_by_chain(
        &self,
        chain_id: i64,
    ) -> Result<Vec<Markets>, DbError> {
        self.full_read::<Markets, _>(
            "SELECT * FROM markets WHERE chain_id = $1",
            |q| q.bind(chain_id),
        )
        .await
    }

    pub async fn read_market_by_address(
        &self,
        chain_id: i64,
        address: &str,
    ) -> Result<Markets, DbError> {
        self.single_read::<Markets, _>(
            "SELECT * FROM markets WHERE chain_id = $1 AND address = $2",
            |q| q.bind(chain_id).bind(address.to_owned()),
        )
        .await
    }

    pub async fn update_protocols(
        &self,
        id: i64,
        display_name: &str,
        category: &str,
        abi_ref: Option<&str>,
    ) -> Result<Protocols, DbError> {
        let now = Utc::now();
        self.mutate_one(
            "UPDATE protocols SET display_name = $1, category = $2, abi_ref = $3, updated_at = $4 \
             WHERE id = $5 \
             RETURNING id, name, display_name, category, abi_ref, watch, capital_target, created_at, updated_at",
            |q| {
                q.bind(display_name.to_owned())
                    .bind(category.to_owned())
                    .bind(abi_ref.map(|s| s.to_owned()))
                    .bind(now)
                    .bind(id)
            },
        )
        .await
    }

    pub async fn update_protocol_watch(
        &self,
        id: i64,
        watch: bool,
    ) -> Result<Protocols, DbError> {
        let now = Utc::now();
        self.mutate_one(
            "UPDATE protocols SET watch = $1, updated_at = $2 WHERE id = $3 \
             RETURNING id, name, display_name, category, abi_ref, watch, capital_target, created_at, updated_at",
            |q| q.bind(watch).bind(now).bind(id),
        )
        .await
    }

    pub async fn update_protocol_capital_target(
        &self,
        id: i64,
        capital_target: bool,
    ) -> Result<Protocols, DbError> {
        let now = Utc::now();
        self.mutate_one(
            "UPDATE protocols SET capital_target = $1, updated_at = $2 WHERE id = $3 \
             RETURNING id, name, display_name, category, abi_ref, watch, capital_target, created_at, updated_at",
            |q| q.bind(capital_target).bind(now).bind(id),
        )
        .await
    }

    pub async fn update_chains(&self, id: i64, name: &str) -> Result<Chains, DbError> {
        self.mutate_one(
            "UPDATE chains SET name = $1 WHERE id = $2 RETURNING id, name, chain_id",
            |q| q.bind(name.to_owned()).bind(id),
        )
        .await
    }

    pub async fn update_markets(
        &self,
        id: i64,
        market_type: &str,
        tokens: &str,
    ) -> Result<Markets, DbError> {
        self.mutate_one(
            "UPDATE markets SET market_type = $1, tokens = $2 WHERE id = $3 \
             RETURNING id, protocol_id, chain_id, address, market_type, tokens, created_at",
            |q| {
                q.bind(market_type.to_owned())
                    .bind(tokens.to_owned())
                    .bind(id)
            },
        )
        .await
    }

    pub async fn update_volume_rollups(
        &self,
        scope: &str,
        scope_id: i64,
        window_label: &str,
        volume_base: i64,
    ) -> Result<VolumeRollups, DbError> {
        let now = Utc::now();
        self.mutate_one(
            "UPDATE volume_rollups SET volume_base = $1, computed_at = $2 \
             WHERE scope = $3 AND scope_id = $4 AND window_label = $5 \
             RETURNING scope, scope_id, window_label, volume_base, computed_at",
            |q| {
                q.bind(volume_base)
                    .bind(now)
                    .bind(scope.to_owned())
                    .bind(scope_id)
                    .bind(window_label.to_owned())
            },
        )
        .await
    }

    pub async fn delete_protocols_by_id(&self, id: i64) -> Result<u64, DbError> {
        self.delete("DELETE FROM protocols WHERE id = $1", |q| q.bind(id))
            .await
    }

    pub async fn delete_chains_by_id(&self, id: i64) -> Result<u64, DbError> {
        self.delete("DELETE FROM chains WHERE id = $1", |q| q.bind(id))
            .await
    }

    pub async fn delete_markets_by_id(&self, id: i64) -> Result<u64, DbError> {
        self.delete("DELETE FROM markets WHERE id = $1", |q| q.bind(id))
            .await
    }

    pub async fn delete_protocol_chains(
        &self,
        protocol_id: i64,
        chain_id: i64,
    ) -> Result<u64, DbError> {
        self.delete(
            "DELETE FROM protocol_chains WHERE protocol_id = $1 AND chain_id = $2",
            |q| q.bind(protocol_id).bind(chain_id),
        )
        .await
    }

    pub async fn delete_volume_rollups(
        &self,
        scope: &str,
        scope_id: i64,
        window_label: &str,
    ) -> Result<u64, DbError> {
        self.delete(
            "DELETE FROM volume_rollups WHERE scope = $1 AND scope_id = $2 AND window_label = $3",
            |q| q.bind(scope.to_owned()).bind(scope_id).bind(window_label.to_owned()),
        )
        .await
    }

    pub async fn delete_all_tables(&self) -> Result<u64, DbError> {
        self.delete(
            "TRUNCATE TABLE \
             aggregate_metrics_ts, protocol_metrics_ts, market_metrics_ts, \
             markets, protocol_chains, chains, protocols, volume_rollups \
             RESTART IDENTITY CASCADE",
            |q| q,
        )
        .await
    }

    pub async fn ensure_chain(&self, name: &str, chain_id: i64) -> Result<Chains, DbError> {
        self.mutate_one(
            "INSERT INTO chains (name, chain_id) VALUES ($1, $2) \
             ON CONFLICT (chain_id) DO UPDATE SET name = EXCLUDED.name \
             RETURNING id, name, chain_id",
            |q| q.bind(name.to_owned()).bind(chain_id),
        )
        .await
    }

    pub async fn link_protocol_to_chains(
        &self,
        protocol_id: i64,
        chain_ids: &[i64],
    ) -> Result<Vec<ProtocolChains>, DbError> {
        let mut tx = self.tx().await?;
        let mut out = Vec::with_capacity(chain_ids.len());
        for &chain_id in chain_ids {
            let result = sqlx::query_as::<_, ProtocolChains>(
                "INSERT INTO protocol_chains (protocol_id, chain_id) VALUES ($1, $2) \
                 RETURNING protocol_id, chain_id",
            )
            .bind(protocol_id)
            .bind(chain_id)
            .fetch_one(&mut *tx)
            .await;

            match result {
                Ok(row) => out.push(row),
                Err(e) => {
                    let _ = tx.rollback().await;
                    return Err(e.into());
                }
            }
        }
        tx.commit().await?;
        Ok(out)
    }
}
