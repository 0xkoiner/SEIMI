use alloy::primitives::U256;

#[derive(Debug, Clone)]
pub struct Name(pub String);

impl From<String> for Name {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct Symbol(pub String);

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Decimals(pub u8);

impl From<u8> for Decimals {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TotalSupply(pub U256);

impl From<U256> for TotalSupply {
    fn from(value: U256) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BalanceOf(pub U256);

impl From<U256> for BalanceOf {
    fn from(value: U256) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Allowance(pub U256);

impl From<U256> for Allowance {
    fn from(value: U256) -> Self {
        Self(value)
    }
}
