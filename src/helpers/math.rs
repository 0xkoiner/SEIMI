use alloy::primitives::{Address, U256};
use alloy::providers::DynProvider;
use sqlx::types::BigDecimal;
use std::str::FromStr;

use crate::parser::aave::data::abi::AAVEv1Pool;
use crate::parser::aave::types::structs::ReserveDataV1;

pub async fn u256_to_bigdecimal(u: U256) -> BigDecimal {
    BigDecimal::from_str(&u.to_string()).expect("U256 decimal string is valid")
}

pub async fn u256_to_u32(u: U256) -> u32 {
    u32::try_from(u).unwrap_or(u32::MAX)
}

pub async fn ray_to_bps(ray: U256) -> i32 {
    let divisor = U256::from(10u64).pow(U256::from(23u64));
    i32::try_from(ray / divisor).unwrap_or(i32::MAX)
}

pub async fn compute_tvl_usd(raw: U256, decimals: i16, price: f64) -> f64 {
    let raw_f64: f64 = raw.to_string().parse().unwrap_or(0.0);
    let scale = 10f64.powi(i32::from(decimals));
    (raw_f64 / scale) * price
}

pub async fn fetch_reserve_snapshots_aave_v1(
    aave_parser_v1: &AAVEv1Pool::AAVEv1PoolInstance<DynProvider>,
    reserves: &[Address],
) -> Vec<(Address, U256, U256)> {
    let mut out = Vec::with_capacity(reserves.len());
    for reserve in reserves.iter().copied() {
        let rd: ReserveDataV1 = aave_parser_v1
            .getReserveData(reserve)
            .call()
            .await
            .expect("Failed to call getReserveData")
            .into();
        out.push((reserve, rd.total_liquidity, rd.liquidity_rate));
    }
    out
}
