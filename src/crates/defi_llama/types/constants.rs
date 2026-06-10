pub struct DefiLlamaUrls;

impl DefiLlamaUrls {
    pub const API: &'static str = "https://api.llama.fi";
    pub const COINS: &'static str = "https://coins.llama.fi";
    pub const STABLECOINS: &'static str = "https://stablecoins.llama.fi";
    pub const YIELDS: &'static str = "https://yields.llama.fi";
    pub const PRO: &'static str = "https://pro-api.llama.fi";
}

pub const DEFILLAMA_API_KEY_ENV: &str = "DEFILLAMA_API_KEY";
pub const USER_AGENT: &str = concat!("SEIMI/", env!("CARGO_PKG_VERSION"), " (intel-plane)");
pub const HTTP_TIMEOUT_SECS: u64 = 15;
