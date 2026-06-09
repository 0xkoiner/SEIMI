use alloy::primitives::{Address, U256, aliases::U40};

use crate::parser::aave::data::abi::AAVEv1Pool::{
    getReserveConfigurationDataReturn as ReserveConfigurationDataV1Return,
    getReserveDataReturn as ReserveDataV1Return,
};
use crate::parser::aave::data::abi::AAVEv2Pool::getReserveDataReturn as ReserveDataV2Return;

#[derive(Debug)]
pub struct ReserveDataV1 {
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
pub struct ReserveConfigurationDataV1 {
    pub ltv: U256,
    pub liquidation_threshold: U256,
    pub liquidation_bonus: U256,
    pub interest_rate_strategy_address: Address,
    pub usage_as_collateral_enabled: bool,
    pub borrowing_enabled: bool,
    pub stable_borrow_rate_enabled: bool,
    pub is_active: bool,
}

impl From<ReserveDataV1Return> for ReserveDataV1 {
    fn from(r: ReserveDataV1Return) -> Self {
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

impl From<ReserveConfigurationDataV1Return> for ReserveConfigurationDataV1 {
    fn from(r: ReserveConfigurationDataV1Return) -> Self {
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

#[derive(Debug)]
pub struct ReserveDataV2 {
    pub configuration: U256,
    pub liquidity_index: u128,
    pub variable_borrow_index: u128,
    pub current_liquidity_rate: u128,
    pub current_variable_borrow_rate: u128,
    pub current_stable_borrow_rate: u128,
    pub last_update_timestamp: U40,
    pub a_token_address: Address,
    pub stable_debt_token_address: Address,
    pub variable_debt_token_address: Address,
    pub interest_rate_strategy_address: Address,
    pub id: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct ReserveConfigurationDataV2 {
    pub ltv: u16,
    pub liquidation_threshold: u16,
    pub liquidation_bonus: u16,
    pub decimals: u8,
    pub is_active: bool,
    pub is_frozen: bool,
    pub borrowing_enabled: bool,
    pub stable_borrow_rate_enabled: bool,
    pub reserve_factor: u16,
}

impl From<ReserveDataV2Return> for ReserveDataV2 {
    fn from(r: ReserveDataV2Return) -> Self {
        Self {
            configuration: r.configuration,
            liquidity_index: r.liquidityIndex,
            variable_borrow_index: r.variableBorrowIndex,
            current_liquidity_rate: r.currentLiquidityRate,
            current_variable_borrow_rate: r.currentVariableBorrowRate,
            current_stable_borrow_rate: r.currentStableBorrowRate,
            last_update_timestamp: r.lastUpdateTimestamp,
            a_token_address: r.aTokenAddress,
            stable_debt_token_address: r.stableDebtTokenAddress,
            variable_debt_token_address: r.variableDebtTokenAddress,
            interest_rate_strategy_address: r.interestRateStrategyAddress,
            id: r.id,
        }
    }
}

impl From<U256> for ReserveConfigurationDataV2 {
    fn from(data: U256) -> Self {
        // Aave V2 ReserveConfigurationMap bit layout (LSB-first):
        //   0..16 ltv | 16..32 liq.threshold | 32..48 liq.bonus | 48..56 decimals |
        //   56 active | 57 frozen | 58 borrowing | 59 stable_borrow | 60..64 reserved |
        //   64..80 reserve_factor
        let bytes = data.to_le_bytes::<32>();
        let flags = bytes[7];
        Self {
            ltv: u16::from_le_bytes([bytes[0], bytes[1]]),
            liquidation_threshold: u16::from_le_bytes([bytes[2], bytes[3]]),
            liquidation_bonus: u16::from_le_bytes([bytes[4], bytes[5]]),
            decimals: bytes[6],
            is_active: flags & 0b0001 != 0,
            is_frozen: flags & 0b0010 != 0,
            borrowing_enabled: flags & 0b0100 != 0,
            stable_borrow_rate_enabled: flags & 0b1000 != 0,
            reserve_factor: u16::from_le_bytes([bytes[8], bytes[9]]),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NormalizedIncomeV2(pub U256);

impl From<U256> for NormalizedIncomeV2 {
    fn from(ray: U256) -> Self {
        Self(ray)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NormalizedVariableDebtV2(pub U256);

impl From<U256> for NormalizedVariableDebtV2 {
    fn from(ray: U256) -> Self {
        Self(ray)
    }
}