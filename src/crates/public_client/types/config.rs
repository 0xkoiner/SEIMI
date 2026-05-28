use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;

use alloy::providers::DynProvider;

#[derive(Deserialize)]
pub struct RpcConfig {
    pub mainnet: HashMap<String, String>,
    pub testnet: HashMap<String, String>,
}

#[derive(Debug)]
pub struct PublicClient {
    pub provider: DynProvider,
    pub chain: &'static str,
    pub network: &'static str,
    pub rpc_url: Cow<'static, str>,
}

pub struct ChainSet(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Chain {
    // mainnets
    Ethereum = 0,
    Base = 1,
    Arbitrum = 2,
    Bnb = 3,
    Avalanche = 4,
    Polygon = 5,
    Sonic = 6,
    Optimism = 7,
    Zora = 8,
    ArbitrumNova = 9,
    PolygonZkevm = 10,
    Gnosis = 11,
    Scroll = 12,
    Linea = 13,
    Plasma = 14,
    Mantle = 15,
    Monad = 16,
    Unichain = 17,
    Celo = 18,
    Zksync = 19,
    Soneium = 20,
    // testnets
    Sepolia = 21,
    BaseSepolia = 22,
    ArbitrumSepolia = 23,
    OptimismSepolia = 24,
}
