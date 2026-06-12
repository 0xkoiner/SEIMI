use alloy::primitives::Address;
use alloy::providers::DynProvider;
use sqlx::types::BigDecimal;

use crate::db::db_engine::sqlx_conn::DBEngine;
use crate::defi_llama::api_connector::DefiLlamaApiConnector;
use crate::helpers::{
    math::{
        compute_tvl_usd, fetch_reserve_snapshots_aave_v1, fetch_reserve_snapshots_aave_v2,
        fetch_reserve_snapshots_aave_v3, ray_to_bps, u256_to_bigdecimal,
    },
    vectors::vec_addr_to_string,
};
use crate::parser::aave::data::abi::{
    AAVEv1Pool::AAVEv1PoolInstance,
    AAVEv2Pool::AAVEv2PoolInstance,
    AAVEv3Pool::AAVEv3PoolInstance
};
use crate::parser::aave::types::constants::{
    AAVE_V1_POOL, AAVE_V2_POOL, AAVE_V3_POOL, AAVE_V4_CORE_HUB, ETH_SENTINEL,
};
use crate::parser::erc_tokens::helpers::read::get_erc20_metadata;

pub async fn insert_aave_v1(
    conn: &DBEngine,
    chain_id: i64,
    parser: AAVEv1PoolInstance<DynProvider>,
    provider: &DynProvider,
    reserves_list: &Vec<Address>,
) {
    let protocol = &conn
        .insert_protocols(
            "AAVEv1",
            "AAVE-V1",
            "Lending",
            Some("src/crates/parser/aave/data/abi_aave_v1.json"),
        )
        .await
        .expect("Failed to insert protocol");

    let _ = &conn
        .insert_protocol_chains(protocol.id, chain_id)
        .await
        .expect("Failed to insert protocol_chain");

    let reserves_v1_csv: String = vec_addr_to_string(&reserves_list).await;

    let snapshots = fetch_reserve_snapshots_aave_v1(&parser, &reserves_list).await;

    let markets = &conn
        .insert_markets(
            protocol.id,
            chain_id,
            &AAVE_V1_POOL.to_string(),
            "lending",
            &reserves_v1_csv,
        )
        .await
        .expect("Failed to insert markets");

    let defillama = DefiLlamaApiConnector::build_connection()
        .await
        .expect("Failed to build DefiLlama connector");
    let mut prices = defillama
        .get_prices_current("ethereum", &reserves_list)
        .await
        .expect("Failed to fetch DefiLlama prices");

    if let Some(eth_quote) = defillama
        .get_price_by_coingecko_id("ethereum")
        .await
        .expect("Failed to fetch ETH price")
    {
        prices.insert(ETH_SENTINEL, eth_quote);
    }

    for (reserve, total_liquidity, liquidity_rate_ray) in snapshots {
        let (name_opt, symbol_opt, decimals_opt, total_supply_opt) = if reserve == ETH_SENTINEL {
            (
                Some("Ether".to_string()),
                Some("ETH".to_string()),
                Some(18_i16),
                None,
            )
        } else {
            get_erc20_metadata(reserve, provider.clone()).await
        };

        let tvl_usd_opt = match (decimals_opt, prices.get(&reserve)) {
            (Some(dec), Some(quote)) => {
                Some(compute_tvl_usd(total_liquidity, dec, quote.price).await)
            }
            _ => None,
        };

        let _ = conn
            .insert_market_metrics_ts(
                markets.id,
                Some(&reserve.to_string()),
                name_opt.as_deref(),
                symbol_opt.as_deref(),
                decimals_opt,
                total_supply_opt,
                tvl_usd_opt,
                u256_to_bigdecimal(total_liquidity).await,
                // TODO(volume): defer until USD-normalization lands; mixed-decimal sums are meaningless.
                BigDecimal::from(0),
                // apy_bps = apr_bps for now; revisit with continuous compounding (e^APR - 1) when V2/V3 land.
                ray_to_bps(liquidity_rate_ray).await,
                ray_to_bps(liquidity_rate_ray).await,
                "aave_v1:getReserveData:ethereum",
                "tier1",
            )
            .await
            .expect("Failed to insert market_metrics_ts");
    }
}


pub async fn insert_aave_v2(
    conn: &DBEngine,
    chain_id: i64,
    parser: AAVEv2PoolInstance<DynProvider>,
    provider: &DynProvider,
    reserves_list: &Vec<Address>,
) {
    let protocol = &conn
        .insert_protocols(
            "AAVEv2",
            "AAVE-V2",
            "Lending",
            Some("src/crates/parser/aave/data/abi_aave_v2.json"),
        )
        .await
        .expect("Failed to insert protocol");

    let _ = &conn
        .insert_protocol_chains(protocol.id, chain_id)
        .await
        .expect("Failed to insert protocol_chain");

    let reserves_v2_csv: String = vec_addr_to_string(&reserves_list).await;

    let snapshots = fetch_reserve_snapshots_aave_v2(&parser, provider.clone(), &reserves_list).await;

    let markets = &conn
        .insert_markets(
            protocol.id,
            chain_id,
            &AAVE_V2_POOL.to_string(),
            "lending",
            &reserves_v2_csv,
        )
        .await
        .expect("Failed to insert markets");

    let defillama = DefiLlamaApiConnector::build_connection()
        .await
        .expect("Failed to build DefiLlama connector");
    let mut prices = defillama
        .get_prices_current("ethereum", &reserves_list)
        .await
        .expect("Failed to fetch DefiLlama prices");

    if let Some(eth_quote) = defillama
        .get_price_by_coingecko_id("ethereum")
        .await
        .expect("Failed to fetch ETH price")
    {
        prices.insert(ETH_SENTINEL, eth_quote);
    }

    for (reserve, total_liquidity, liquidity_rate_ray) in snapshots {
        let (name_opt, symbol_opt, decimals_opt, total_supply_opt) = if reserve == ETH_SENTINEL {
            (
                Some("Ether".to_string()),
                Some("ETH".to_string()),
                Some(18_i16),
                None,
            )
        } else {
            get_erc20_metadata(reserve, provider.clone()).await
        };

        let tvl_usd_opt = match (decimals_opt, prices.get(&reserve)) {
            (Some(dec), Some(quote)) => {
                Some(compute_tvl_usd(total_liquidity, dec, quote.price).await)
            }
            _ => None,
        };

        let _ = conn
            .insert_market_metrics_ts(
                markets.id,
                Some(&reserve.to_string()),
                name_opt.as_deref(),
                symbol_opt.as_deref(),
                decimals_opt,
                total_supply_opt,
                tvl_usd_opt,
                u256_to_bigdecimal(total_liquidity).await,
                // TODO(volume): defer until USD-normalization lands; mixed-decimal sums are meaningless.
                BigDecimal::from(0),
                // apy_bps = apr_bps for now; revisit with continuous compounding (e^APR - 1) when V2/V3 land.
                ray_to_bps(liquidity_rate_ray).await,
                ray_to_bps(liquidity_rate_ray).await,
                "aave_v2:getReserveData:ethereum",
                "tier1",
            )
            .await
            .expect("Failed to insert market_metrics_ts");
    }
}

pub async fn insert_aave_v3(
    conn: &DBEngine,
    chain_id: i64,
    parser: AAVEv3PoolInstance<DynProvider>,
    provider: &DynProvider,
    reserves_list: &Vec<Address>,
) {
    let protocol = &conn
        .insert_protocols(
            "AAVEv3",
            "AAVE-V3",
            "Lending",
            Some("src/crates/parser/aave/data/abi_aave_v3.json"),
        )
        .await
        .expect("Failed to insert protocol");

    let _ = &conn
        .insert_protocol_chains(protocol.id, chain_id)
        .await
        .expect("Failed to insert protocol_chain");

    let reserves_v3_csv: String = vec_addr_to_string(&reserves_list).await;

    let snapshots = fetch_reserve_snapshots_aave_v3(&parser, provider.clone(), &reserves_list).await;

    let markets = &conn
        .insert_markets(
            protocol.id,
            chain_id,
            &AAVE_V3_POOL.to_string(),
            "lending",
            &reserves_v3_csv,
        )
        .await
        .expect("Failed to insert markets");

    let defillama = DefiLlamaApiConnector::build_connection()
        .await
        .expect("Failed to build DefiLlama connector");
    let mut prices = defillama
        .get_prices_current("ethereum", &reserves_list)
        .await
        .expect("Failed to fetch DefiLlama prices");

    if let Some(eth_quote) = defillama
        .get_price_by_coingecko_id("ethereum")
        .await
        .expect("Failed to fetch ETH price")
    {
        prices.insert(ETH_SENTINEL, eth_quote);
    }

    for (reserve, total_liquidity, liquidity_rate_ray) in snapshots {
        let (name_opt, symbol_opt, decimals_opt, total_supply_opt) = if reserve == ETH_SENTINEL {
            (
                Some("Ether".to_string()),
                Some("ETH".to_string()),
                Some(18_i16),
                None,
            )
        } else {
            get_erc20_metadata(reserve, provider.clone()).await
        };

        let tvl_usd_opt = match (decimals_opt, prices.get(&reserve)) {
            (Some(dec), Some(quote)) => {
                Some(compute_tvl_usd(total_liquidity, dec, quote.price).await)
            }
            _ => None,
        };

        let _ = conn
            .insert_market_metrics_ts(
                markets.id,
                Some(&reserve.to_string()),
                name_opt.as_deref(),
                symbol_opt.as_deref(),
                decimals_opt,
                total_supply_opt,
                tvl_usd_opt,
                u256_to_bigdecimal(total_liquidity).await,
                // TODO(volume): defer until USD-normalization lands; mixed-decimal sums are meaningless.
                BigDecimal::from(0),
                // apy_bps = apr_bps for now; revisit with continuous compounding (e^APR - 1) when V2/V3 land.
                ray_to_bps(liquidity_rate_ray).await,
                ray_to_bps(liquidity_rate_ray).await,
                "aave_v3:getReserveData:ethereum",
                "tier1",
            )
            .await
            .expect("Failed to insert market_metrics_ts");
    }
}