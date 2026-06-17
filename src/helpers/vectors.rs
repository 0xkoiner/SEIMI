use crate::parser::aave::types::structs::AssetV4;
use alloy::primitives::Address;

pub async fn vec_addr_to_string(v: &Vec<Address>) -> String {
    v.iter()
        .map(Address::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

pub async fn vec_assets_v4_underlying_to_string(v: &Vec<AssetV4>) -> String {
    v.iter()
        .map(|a| a.underlying.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}
