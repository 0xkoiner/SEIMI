use alloy::primitives::{Address, U256, aliases::U40};

use crate::parser::aave::data::abi::AAVEv1Pool::getReserveDataReturn;

#[derive(Debug)]
pub struct ReserveData {
    pub total_liquidity: U256,
    pub available_liquidity: U256,
    pub total_borrows_stable: U256,
    pub total_borrows_variable: U256,
    pub liquidity_rate: U256,
    pub variable_borrow_rate: U256,
    pub stable_borrow_rate: U256,
    pub average_stable_borrow_rate: U256,
    pub utilization_rate: U256,
    pub liquidity_index: U256,
    pub variable_borrow_index: U256,
    pub a_token_address: Address,
    pub last_update_timestamp: U40,
}

impl From<getReserveDataReturn> for ReserveData {
    fn from(r: getReserveDataReturn) -> Self {
        Self {
            total_liquidity: r.totalLiquidity,
            available_liquidity: r.availableLiquidity,
            total_borrows_stable: r.totalBorrowsStable,
            total_borrows_variable: r.totalBorrowsVariable,
            liquidity_rate: r.liquidityRate,
            variable_borrow_rate: r.variableBorrowRate,
            stable_borrow_rate: r.stableBorrowRate,
            average_stable_borrow_rate: r.averageStableBorrowRate,
            utilization_rate: r.utilizationRate,
            liquidity_index: r.liquidityIndex,
            variable_borrow_index: r.variableBorrowIndex,
            a_token_address: r.aTokenAddress,
            last_update_timestamp: r.lastUpdateTimestamp,
        }
    }
}
