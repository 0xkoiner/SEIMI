use alloy::primitives::{Address, U256};
use alloy::providers::DynProvider;
use sqlx::types::BigDecimal;
use std::str::FromStr;

use crate::parser::aave::data::abi::{AAVEv1Pool, AAVEv2Pool, AAVEv3Pool, AAVEv4CoreHub};
use crate::parser::aave::types::structs::ReserveDataV1;

pub async fn u256_to_bigdecimal(u: U256) -> BigDecimal {
    BigDecimal::from_str(&u.to_string()).expect("U256 decimal string is valid")
}

// Aave RAY (1e27 = 100%) → basis points (1e4 = 100%). Saturates on overflow.
pub async fn ray_to_bps(ray: U256) -> i32 {
    let divisor = U256::from(10u64).pow(U256::from(23u64));
    i32::try_from(ray / divisor).unwrap_or(i32::MAX)
}

pub async fn calculate_average_apr_ray_aave_v1(
    aave_parser_v1: &AAVEv1Pool::AAVEv1PoolInstance<DynProvider>,
    reserves: &Vec<Address>,
) -> (U256, U256) {
    let mut total_liquidity_sum = U256::ZERO;
    let mut weighted_apr_ray_sum = U256::ZERO;

    for reserve in reserves.iter().copied() {
        let rd: ReserveDataV1 = aave_parser_v1
            .getReserveData(reserve)
            .call()
            .await
            .expect("Failed to call getReserveData")
            .into();
        total_liquidity_sum = total_liquidity_sum.saturating_add(rd.total_liquidity);

        weighted_apr_ray_sum = weighted_apr_ray_sum
            .saturating_add(rd.liquidity_rate.saturating_mul(rd.total_liquidity));
    }
    let avg_apr_ray = if total_liquidity_sum.is_zero() {
        U256::ZERO
    } else {
        weighted_apr_ray_sum / total_liquidity_sum
    };
    (avg_apr_ray, total_liquidity_sum)
}
