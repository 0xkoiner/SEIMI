use sqlx::FromRow;
use sqlx::types::BigDecimal;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow)]
pub struct Users {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Protocols {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub category: String,
    pub abi_ref: Option<String>,
    pub watch: bool,
    pub capital_target: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Chains {
    pub id: i64,
    pub name: String,
    pub chain_id: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct ProtocolChains {
    pub protocol_id: i64,
    pub chain_id: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct Markets {
    pub id: i64,
    pub protocol_id: i64,
    pub chain_id: i64,
    pub address: String,
    pub market_type: String,
    pub tokens: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct MarketMetricsTs {
    pub id: i64,
    pub market_id: i64,
    pub underlying: Option<String>,
    pub observed_at: DateTime<Utc>,
    pub tvl_base: BigDecimal,
    pub volume_base: BigDecimal,
    pub apy_bps: Option<i32>,
    pub apr_bps: Option<i32>,
    pub source: String,
    pub trust_tier: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct ProtocolMetricsTs {
    pub id: i64,
    pub protocol_id: i64,
    pub observed_at: DateTime<Utc>,
    pub tvl_base: BigDecimal,
    pub volume_base: BigDecimal,
    pub source: String,
    pub trust_tier: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct AggregateMetricsTs {
    pub id: i64,
    pub observed_at: DateTime<Utc>,
    pub total_tvl_base: BigDecimal,
    pub total_volume_base: BigDecimal,
    pub protocol_count: i32,
    pub source: String,
    pub trust_tier: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct VolumeRollups {
    pub scope: String,
    pub scope_id: i64,
    pub window_label: String,
    pub volume_base: BigDecimal,
    pub computed_at: DateTime<Utc>,
}
