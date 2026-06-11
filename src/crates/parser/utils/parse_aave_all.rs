use alloy::primitives::{Address, U256};
use alloy::providers::DynProvider;

use crate::parser::aave::types::structs::AssetV4;
use crate::parser::aave::data::abi::{AAVEv1Pool, AAVEv2Pool, AAVEv3Pool, AAVEv4CoreHub};

pub async fn parser_underlying_aave_reserves_v1(address: Address, provider: DynProvider) -> Vec<Address> {
    let aave_parser_v1 = AAVEv1Pool::new(address, provider);

    aave_parser_v1
        .getReserves()
        .call()
        .await
        .expect("Failed to get reserves data")
}

pub async fn parser_underlying_aave_reserves_v2(address: Address, provider: DynProvider) -> Vec<Address> {
    let aave_parser_v2 = AAVEv2Pool::new(address, provider);

    aave_parser_v2
        .getReservesList()
        .call()
        .await
        .expect("Failed to get reserves data")
}


pub async fn parser_underlying_aave_reserves_v3(address: Address, provider: DynProvider) -> Vec<Address> {
    let aave_parser_v3 = AAVEv3Pool::new(address, provider);

    aave_parser_v3
        .getReservesList()
        .call()
        .await
        .expect("Failed to get reserves data")
}

pub async fn parser_underlying_aave_hub_v4(address: Address, provider: DynProvider) -> Vec<Address> {
    let aave_hub_v4 = AAVEv4CoreHub::new(address, provider);

    let asset_count = aave_hub_v4
        .getAssetCount()
        .call()
        .await
        .expect("Failed to get asset count");

    let mut asset_id = U256::ZERO;
    let mut underlying_addresses: Vec<Address> = vec![];


    while asset_id < asset_count {
        let asset: AssetV4 = aave_hub_v4
            .getAsset(asset_id)
            .call()
            .await
            .expect("Failed to call getAsset")
            .into();
        underlying_addresses.push(asset.underlying);
        asset_id += U256::ONE;
    }

    underlying_addresses
}