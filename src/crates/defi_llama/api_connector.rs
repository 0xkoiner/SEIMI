use alloy::transports::http::reqwest::{Client, ClientBuilder};
use dotenvy::dotenv;
use std::time::Duration;

use crate::defi_llama::types::constants::{
    DEFILLAMA_API_KEY_ENV, DefiLlamaUrls, HTTP_TIMEOUT_SECS, USER_AGENT,
};
use crate::defi_llama::types::errors::DefiLlamaError;
use crate::defi_llama::types::structs::ProtocolSummary;

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

    // When an API key is present, route through pro-api with the /api/ prefix
    // (per the Free-to-Pro mapping in llms.txt — see defillama-api.md §2).
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
}
