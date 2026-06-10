#[derive(Debug, thiserror::Error)]
pub enum DefiLlamaError {
    #[error("HTTP client build failed: {0}")]
    ClientBuild(String),
    #[error("HTTP transport: {0}")]
    Transport(#[from] alloy::transports::http::reqwest::Error),
    #[error("HTTP {status}: {body}")]
    BadStatus { status: u16, body: String },
    #[error("response decode: {0}")]
    Decode(#[from] serde_json::Error),
    #[error("not found")]
    NotFound,
}
