use alloy::primitives::{Address, U256, aliases::U40};

use crate::parser::aave::data::abi::AAVEv1Pool::{
    getReserveConfigurationDataReturn, getReserveDataReturn,
};

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

#[derive(Debug)]
pub struct ReserveConfigurationData {
    pub ltv: U256,
    pub liquidation_threshold: U256,
    pub liquidation_bonus: U256,
    pub interest_rate_strategy_address: Address,
    pub usage_as_collateral_enabled: bool,
    pub borrowing_enabled: bool,
    pub stable_borrow_rate_enabled: bool,
    pub is_active: bool,
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

impl From<getReserveConfigurationDataReturn> for ReserveConfigurationData {
    fn from(r: getReserveConfigurationDataReturn) -> Self {
        Self {
            ltv: r.ltv,
            liquidation_threshold: r.liquidationThreshold,
            liquidation_bonus: r.liquidationBonus,
            interest_rate_strategy_address: r.interestRateStrategyAddress,
            usage_as_collateral_enabled: r.usageAsCollateralEnabled,
            borrowing_enabled: r.borrowingEnabled,
            stable_borrow_rate_enabled: r.stableBorrowRateEnabled,
            is_active: r.isActive,
        }
    }
}
