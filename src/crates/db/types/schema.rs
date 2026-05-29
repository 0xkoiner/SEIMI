use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Users {
    pub id: i64,
    pub name: String,
    pub email: String,
}

// #[derive(Debug, Clone, FromRow)]
// pub struct Protocols {
//     pub id: usize,
//     pub name: String,
//     pub display_name: String,
//     pub category: String,
//     pub abi_ref: String,
//     pub watch: bool,
//     pub capital_target: bool,
//     pub created_at: Duration::Date,
//     pub updated_at: Duration::Date,
// }

// #[derive(Debug, Clone, FromRow)]
// pub struct Chains {
//     id: usize,
//     name: String,
//     chain_id: usize,
// }

// #[derive(Debug, Clone, FromRow)]
// pub struct ProtocolChains {
//     protocol_id: usize,
//     chain_id: usize,
// }

// #[derive(Debug, Clone, FromRow)]
// pub struct Markets {
//     pub id : usize,
//     protocol_id: usize,
//     chain_id: usize,
//     address: String,
//     market_type: String,
//     tokens: String,
//     pub created_at: Duration::Date,

// }

// #[derive(Debug, Clone, FromRow)]
// pub struct MarketMetricsTs{
//     pub id : usize,
//     pub market_id: usize,
//     pub observed_at: Duration::Date,
//     pub tvl_base: u128,
//     pub volume_base: u128,
//     pub apy_bps: usize,
//     pub apr_bps: usize,
//     pub source: String,
//     pub trust_tier: String,
// }

// #[derive(Debug, Clone, FromRow)]
// pub struct AggregateMetricsTs