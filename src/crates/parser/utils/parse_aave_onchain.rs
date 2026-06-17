use alloy::primitives::{Address, U256};
use alloy::providers::DynProvider;

use crate::parser::aave::data::abi::{AAVEv1Pool, AAVEv2Pool, AAVEv3Pool, AAVEv4CoreHub};
use crate::parser::aave::types::structs::{
    AssetV4, ReserveConfigurationDataV1, ReserveConfigurationDataV2, ReserveConfigurationDataV3,
    ReserveDataV1, ReserveDataV2, ReserveDataV3,
};

pub async fn parser_underlying_reserves_aave_v1(
    parser: &AAVEv1Pool::AAVEv1PoolInstance<DynProvider>,
) -> Vec<Address> {
    parser
        .getReserves()
        .call()
        .await
        .expect("Failed to get reserves data")
}

pub async fn parse_reserve_data_aave_v1(
    reserves: &Vec<Address>,
    parser: &AAVEv1Pool::AAVEv1PoolInstance<DynProvider>,
) -> Vec<ReserveDataV1> {
    let mut reserves_data: Vec<ReserveDataV1> = vec![];

    for reserve in reserves {
        let reserve_data: ReserveDataV1 = parser
            .getReserveData(*reserve)
            .call()
            .await
            .expect("Failed to get reserve data")
            .into();
        reserves_data.push(reserve_data);
    }

    reserves_data
}

pub async fn parse_reserve_conf_aave_v1(
    reserves: &Vec<Address>,
    parser: &AAVEv1Pool::AAVEv1PoolInstance<DynProvider>,
) -> Vec<ReserveConfigurationDataV1> {
    let mut reserves_conf: Vec<ReserveConfigurationDataV1> = vec![];

    for reserve in reserves {
        let reserve_conf: ReserveConfigurationDataV1 = parser
            .getReserveConfigurationData(*reserve)
            .call()
            .await
            .expect("Failed to get reserve configuration data")
            .into();
        reserves_conf.push(reserve_conf);
    }

    reserves_conf
}

pub async fn parser_underlying_reserves_aave_v2(
    parser: &AAVEv2Pool::AAVEv2PoolInstance<DynProvider>,
) -> Vec<Address> {
    parser
        .getReservesList()
        .call()
        .await
        .expect("Failed to get reserves data")
}

pub async fn parse_reserve_data_aave_v2(
    reserves: &Vec<Address>,
    parser: &AAVEv2Pool::AAVEv2PoolInstance<DynProvider>,
) -> (Vec<ReserveDataV2>, Vec<ReserveConfigurationDataV2>) {
    let mut reserves_data: Vec<ReserveDataV2> = vec![];
    let mut reserves_conf: Vec<ReserveConfigurationDataV2> = vec![];

    for reserve in reserves {
        let reserve_data: ReserveDataV2 = parser
            .getReserveData(*reserve)
            .call()
            .await
            .expect("Failed to get reserve data")
            .into();

        reserves_conf.push(reserve_data.configuration.into());
        reserves_data.push(reserve_data);
    }

    (reserves_data, reserves_conf)
}

pub async fn parser_underlying_reserves_aave_v3(
    parser: &AAVEv3Pool::AAVEv3PoolInstance<DynProvider>,
) -> Vec<Address> {
    parser
        .getReservesList()
        .call()
        .await
        .expect("Failed to get reserves data")
}

pub async fn parse_reserve_data_aave_v3(
    reserves: &Vec<Address>,
    parser: &AAVEv3Pool::AAVEv3PoolInstance<DynProvider>,
) -> (Vec<ReserveDataV3>, Vec<ReserveConfigurationDataV3>) {
    let mut reserves_data: Vec<ReserveDataV3> = vec![];
    let mut reserves_conf: Vec<ReserveConfigurationDataV3> = vec![];

    for reserve in reserves {
        let reserve_data: ReserveDataV3 = parser
            .getReserveData(*reserve)
            .call()
            .await
            .expect("Failed to get reserve data")
            .into();

        reserves_conf.push(reserve_data.configuration.into());
        reserves_data.push(reserve_data);
    }

    (reserves_data, reserves_conf)
}

pub async fn parser_assets_hub_aave_v4(
    parser: &AAVEv4CoreHub::AAVEv4CoreHubInstance<DynProvider>,
) -> (Vec<AssetV4>, Vec<Address>) {
    let asset_count = parser
        .getAssetCount()
        .call()
        .await
        .expect("Failed to get asset count");

    let mut asset_id = U256::ZERO;
    let mut assets: Vec<AssetV4> = vec![];
    let mut assets_addresses: Vec<Address> = vec![];

    while asset_id < asset_count {
        let mut asset: AssetV4 = parser
            .getAsset(asset_id)
            .call()
            .await
            .expect("Failed to call getAsset")
            .into();
        asset.asset_id = asset_id;
        assets_addresses.push(asset.underlying.clone());
        assets.push(asset);
        asset_id += U256::ONE;
    }

    (assets, assets_addresses)
}
