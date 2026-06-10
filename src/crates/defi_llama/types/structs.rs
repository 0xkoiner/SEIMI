use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProtocolSummary {
    pub id: String,
    pub name: String,
    pub symbol: Option<String>,
    pub category: Option<String>,
    pub chains: Vec<String>,
    pub tvl: Option<f64>,
    #[serde(rename = "chainTvls")]
    pub chain_tvls: Option<HashMap<String, f64>>,
    pub change_1d: Option<f64>,
    pub change_7d: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceQuote {
    pub decimals: Option<u8>,
    pub price: f64,
    pub symbol: String,
    pub timestamp: f64,
    pub confidence: Option<f64>,
}
