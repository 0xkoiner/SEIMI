#[derive(Debug, thiserror::Error)]
pub enum RpcError {
    #[error("Failed to read config: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("Unknown network: {0}")]
    UnknownNetwork(String),
    #[error("Chain {0} not found in {1}")]
    ChainNotFound(String, String),
}

#[derive(Debug, thiserror::Error)]
pub enum PublicClientError {
    #[error("RPC config error: {0}")]
    RpcConfig(#[from] RpcError),
    #[error("Invalid RPC URL: {0}")]
    InvalidUrl(String),
    #[error("Provider error: {0}")]
    ProviderError(String),
}