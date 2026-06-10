use alloy::primitives::Address;
use alloy::transports::http::reqwest::{Client, ClientBuilder};
use dotenvy::dotenv;
use std::collections::HashMap;
use std::time::Duration;

use crate::defi_llama::types::constants::{
    DEFILLAMA_API_KEY_ENV, DefiLlamaUrls, HTTP_TIMEOUT_SECS, USER_AGENT,
};
use crate::defi_llama::types::errors::DefiLlamaError;
use crate::defi_llama::types::structs::{PriceQuote, ProtocolSummary};

#[derive(Debug, Clone)]
pub struct DefiLlamaApiConnector {
    http: Client,
    api_key: Option<String>,
}

impl DefiLlamaApiConnector {
    pub async fn build_connection() -> Result<Self, DefiLlamaError> {
        dotenv().ok();
        let api_key = std::env::var(DEFILLAMA_API_KEY_ENV).ok();
        let http = ClientBuilder::new()
            .timeout(Duration::from_secs(HTTP_TIMEOUT_SECS))
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| DefiLlamaError::ClientBuild(e.to_string()))?;
        Ok(Self { http, api_key })
    }

    #[inline]
    #[must_use]
    pub fn has_api_key(&self) -> bool {
        self.api_key.is_some()
    }

    fn protocols_url(&self) -> String {
        match &self.api_key {
            Some(key) => format!("{}/{}/api/protocols", DefiLlamaUrls::PRO, key),
            None => format!("{}/protocols", DefiLlamaUrls::API),
        }
    }

    pub async fn get_protocols(&self) -> Result<Vec<ProtocolSummary>, DefiLlamaError> {
        let resp = self.http.get(self.protocols_url()).send().await?;
        let status = resp.status();
        if status == 404 {
            return Err(DefiLlamaError::NotFound);
        }
        if !status.is_success() {
            return Err(DefiLlamaError::BadStatus {
                status: status.as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        let bytes = resp.bytes().await?;
        let parsed: Vec<ProtocolSummary> = serde_json::from_slice(&bytes)?;
        Ok(parsed)
    }

    fn prices_current_url(&self, coin_keys: &str) -> String {
        match &self.api_key {
            Some(key) => format!(
                "{}/{}/coins/prices/current/{}",
                DefiLlamaUrls::PRO,
                key,
                coin_keys
            ),
            None => format!("{}/prices/current/{}", DefiLlamaUrls::COINS, coin_keys),
        }
    }

    pub async fn get_prices_current(
        &self,
        chain: &str,
        tokens: &[Address],
    ) -> Result<HashMap<Address, PriceQuote>, DefiLlamaError> {
        if tokens.is_empty() {
            return Ok(HashMap::new());
        }

        let coin_keys = tokens
            .iter()
            .map(|t| format!("{chain}:{t}"))
            .collect::<Vec<_>>()
            .join(",");

        let url = self.prices_current_url(&coin_keys);
        let resp = self.http.get(url).send().await?;
        let status = resp.status();
        if status == 404 {
            return Err(DefiLlamaError::NotFound);
        }
        if !status.is_success() {
            return Err(DefiLlamaError::BadStatus {
                status: status.as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        let bytes = resp.bytes().await?;

        #[derive(serde::Deserialize)]
        struct PricesCurrentResponseRaw {
            coins: HashMap<String, PriceQuote>,
        }
        let raw: PricesCurrentResponseRaw = serde_json::from_slice(&bytes)?;

        let mut out = HashMap::with_capacity(raw.coins.len());
        for (key, quote) in raw.coins {
            if let Some((_, addr_str)) = key.split_once(':') {
                if let Ok(addr) = addr_str.parse::<Address>() {
                    out.insert(addr, quote);
                }
            }
        }
        Ok(out)
    }

    pub async fn get_price_by_coingecko_id(
        &self,
        coingecko_id: &str,
    ) -> Result<Option<PriceQuote>, DefiLlamaError> {
        let coin_key = format!("coingecko:{coingecko_id}");
        let url = self.prices_current_url(&coin_key);
        let resp = self.http.get(url).send().await?;
        let status = resp.status();
        if status == 404 {
            return Err(DefiLlamaError::NotFound);
        }
        if !status.is_success() {
            return Err(DefiLlamaError::BadStatus {
                status: status.as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        let bytes = resp.bytes().await?;

        #[derive(serde::Deserialize)]
        struct PricesCurrentResponseRaw {
            coins: HashMap<String, PriceQuote>,
        }
        let mut raw: PricesCurrentResponseRaw = serde_json::from_slice(&bytes)?;
        Ok(raw.coins.remove(&coin_key))
    }
}
