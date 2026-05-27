use std::fmt::{Display, Formatter};

use crate::types::config::Chain;

impl Chain {
    pub const COUNT: usize = 25;

    pub const ALL: [Self; Self::COUNT] = [
        Self::Ethereum,
        Self::Base,
        Self::Arbitrum,
        Self::Bnb,
        Self::Avalanche,
        Self::Polygon,
        Self::Sonic,
        Self::Optimism,
        Self::Zora,
        Self::ArbitrumNova,
        Self::PolygonZkevm,
        Self::Gnosis,
        Self::Scroll,
        Self::Linea,
        Self::Plasma,
        Self::Mantle,
        Self::Monad,
        Self::Unichain,
        Self::Celo,
        Self::Zksync,
        Self::Soneium,
        Self::Sepolia,
        Self::BaseSepolia,
        Self::ArbitrumSepolia,
        Self::OptimismSepolia,
    ];

    #[must_use]
    #[inline]
    pub const fn network(&self) -> &'static str {
        match self {
            Self::Sepolia | Self::BaseSepolia | Self::ArbitrumSepolia | Self::OptimismSepolia => {
                "testnet"
            }
            _ => "mainnet",
        }
    }

    #[must_use]
    #[inline]
    pub const fn as_rpc_key(&self) -> &'static str {
        match self {
            Self::Ethereum => "ethereum",
            Self::Base => "base",
            Self::Arbitrum => "arbitrum",
            Self::Bnb => "bnb",
            Self::Avalanche => "avalanche",
            Self::Polygon => "polygon",
            Self::Sonic => "sonic",
            Self::Optimism => "optimism",
            Self::Zora => "zora",
            Self::ArbitrumNova => "arbitrum_nova",
            Self::PolygonZkevm => "polygon_zkevm",
            Self::Gnosis => "gnosis",
            Self::Scroll => "scroll",
            Self::Linea => "linea",
            Self::Plasma => "plasma",
            Self::Mantle => "mantle",
            Self::Monad => "monad",
            Self::Unichain => "unichain",
            Self::Celo => "celo",
            Self::Zksync => "zksync",
            Self::Soneium => "soneium",
            Self::Sepolia => "sepolia",
            Self::BaseSepolia => "base_sepolia",
            Self::ArbitrumSepolia => "arbitrum_sepolia",
            Self::OptimismSepolia => "optimism_sepolia",
        }
    }

    #[must_use]
    #[inline]
    pub const fn flag(&self) -> &'static str {
        match self {
            Self::ArbitrumNova => "arbitrum-nova",
            Self::PolygonZkevm => "polygon-zkevm",
            Self::BaseSepolia => "base-sepolia",
            Self::ArbitrumSepolia => "arbitrum-sepolia",
            Self::OptimismSepolia => "optimism-sepolia",
            other => other.as_rpc_key(),
        }
    }

    #[must_use]
    #[inline]
    pub fn from_flag(flag: &str) -> Option<Self> {
        match flag {
            "ethereum" => Some(Self::Ethereum),
            "base" => Some(Self::Base),
            "arbitrum" => Some(Self::Arbitrum),
            "bnb" => Some(Self::Bnb),
            "avalanche" => Some(Self::Avalanche),
            "polygon" => Some(Self::Polygon),
            "sonic" => Some(Self::Sonic),
            "optimism" => Some(Self::Optimism),
            "zora" => Some(Self::Zora),
            "arbitrum-nova" | "arbitrum_nova" => Some(Self::ArbitrumNova),
            "polygon-zkevm" | "polygon_zkevm" => Some(Self::PolygonZkevm),
            "gnosis" => Some(Self::Gnosis),
            "scroll" => Some(Self::Scroll),
            "linea" => Some(Self::Linea),
            "plasma" => Some(Self::Plasma),
            "mantle" => Some(Self::Mantle),
            "monad" => Some(Self::Monad),
            "unichain" => Some(Self::Unichain),
            "celo" => Some(Self::Celo),
            "zksync" => Some(Self::Zksync),
            "soneium" => Some(Self::Soneium),
            "sepolia" => Some(Self::Sepolia),
            "base-sepolia" | "base_sepolia" => Some(Self::BaseSepolia),
            "arbitrum-sepolia" | "arbitrum_sepolia" => Some(Self::ArbitrumSepolia),
            "optimism-sepolia" | "optimism_sepolia" => Some(Self::OptimismSepolia),
            _ => None,
        }
    }
}

impl Display for Chain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.flag())
    }
}
