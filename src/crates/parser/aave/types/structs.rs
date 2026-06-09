use alloy::primitives::{
    Address, U256,
    aliases::{I200, U24, U40, U96, U120, U200},
};

use crate::parser::aave::data::abi::AAVEv1Pool::{
    getReserveConfigurationDataReturn as ReserveConfigurationDataV1Return,
    getReserveDataReturn as ReserveDataV1Return,
};
use crate::parser::aave::data::abi::AAVEv2Pool::getReserveDataReturn as ReserveDataV2Return;
use crate::parser::aave::data::abi::AAVEv3Pool::getReserveDataReturn as ReserveDataV3Return;
use crate::parser::aave::data::abi::AAVEv4CoreHub::{
    getAssetReturn as AssetV4Return, getSpokeConfigReturn as SpokeConfigV4Return,
    getSpokeReturn as SpokeDataV4Return,
};

/**
 * 
 * 
 * AAVE V1
 * 
 * 
 */
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

/**
 * 
 * 
 * AAVE V2
 * 
 * 
 */
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

/**
 * 
 * 
 * AAVE V3
 * 
 * 
 */
#[derive(Debug)]
pub struct ReserveDataV3 {
    pub configuration: U256,
    pub liquidity_index: u128,
    pub current_liquidity_rate: u128,
    pub variable_borrow_index: u128,
    pub current_variable_borrow_rate: u128,
    pub current_stable_borrow_rate: u128,
    pub last_update_timestamp: U40,
    pub id: u16,
    pub a_token_address: Address,
    pub stable_debt_token_address: Address,
    pub variable_debt_token_address: Address,
    pub interest_rate_strategy_address: Address,
    pub accrued_to_treasury: u128,
    pub unbacked: u128,
    pub isolation_mode_total_debt: u128,
}

impl From<ReserveDataV3Return> for ReserveDataV3 {
    fn from(r: ReserveDataV3Return) -> Self {
        Self {
            configuration: r.configuration,
            liquidity_index: r.liquidityIndex,
            current_liquidity_rate: r.currentLiquidityRate,
            variable_borrow_index: r.variableBorrowIndex,
            current_variable_borrow_rate: r.currentVariableBorrowRate,
            current_stable_borrow_rate: r.currentStableBorrowRate,
            last_update_timestamp: r.lastUpdateTimestamp,
            id: r.id,
            a_token_address: r.aTokenAddress,
            stable_debt_token_address: r.stableDebtTokenAddress,
            variable_debt_token_address: r.variableDebtTokenAddress,
            interest_rate_strategy_address: r.interestRateStrategyAddress,
            accrued_to_treasury: r.accruedToTreasury,
            unbacked: r.unbacked,
            isolation_mode_total_debt: r.isolationModeTotalDebt,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReserveConfigurationDataV3 {
    pub ltv: u16,
    pub liquidation_threshold: u16,
    pub liquidation_bonus: u16,
    pub decimals: u8,
    pub is_active: bool,
    pub is_frozen: bool,
    pub borrowing_enabled: bool,
    pub is_paused: bool,
    pub flashloan_enabled: bool,
    pub reserve_factor: u16,
    pub borrow_cap: u64,
    pub supply_cap: u64,
    pub liquidation_protocol_fee: u16,
}

impl From<U256> for ReserveConfigurationDataV3 {
    fn from(data: U256) -> Self {
        // Aave V3 ReserveConfigurationMap bit layout (LSB-first):
        //   0..16    ltv
        //   16..32   liquidation_threshold
        //   32..48   liquidation_bonus
        //   48..56   decimals
        //   56       is_active
        //   57       is_frozen
        //   58       borrowing_enabled
        //   59       DEPRECATED stable-rate-borrowing
        //   60       is_paused
        //   61       DEPRECATED isolation-borrow
        //   62       DEPRECATED siloed-borrowing
        //   63       flashloan_enabled
        //   64..80   reserve_factor
        //   80..116  borrow_cap                (36 bits, whole tokens; 0 == no cap)
        //   116..152 supply_cap                (36 bits, whole tokens; 0 == no cap)
        //   152..168 liquidation_protocol_fee
        //   168..256 DEPRECATED/unused (eMode, unbacked, debt ceiling, virtual accounting)
        let bytes = data.to_le_bytes::<32>();
        let flags = bytes[7];
        // 36-bit borrow_cap occupies bytes 10..15; the top 4 bits of byte 14 belong to supply_cap.
        let borrow_cap = u64::from_le_bytes([
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], 0, 0, 0,
        ]) & 0xF_FFFF_FFFF;
        // 36-bit supply_cap starts at byte 14 bit 4; take 5 bytes and shift down.
        let supply_cap = (u64::from_le_bytes([
            bytes[14], bytes[15], bytes[16], bytes[17], bytes[18], 0, 0, 0,
        ]) >> 4)
            & 0xF_FFFF_FFFF;
        Self {
            ltv: u16::from_le_bytes([bytes[0], bytes[1]]),
            liquidation_threshold: u16::from_le_bytes([bytes[2], bytes[3]]),
            liquidation_bonus: u16::from_le_bytes([bytes[4], bytes[5]]),
            decimals: bytes[6],
            is_active: flags & 0b0000_0001 != 0,
            is_frozen: flags & 0b0000_0010 != 0,
            borrowing_enabled: flags & 0b0000_0100 != 0,
            is_paused: flags & 0b0001_0000 != 0,
            flashloan_enabled: flags & 0b1000_0000 != 0,
            reserve_factor: u16::from_le_bytes([bytes[8], bytes[9]]),
            borrow_cap,
            supply_cap,
            liquidation_protocol_fee: u16::from_le_bytes([bytes[19], bytes[20]]),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NormalizedIncomeV3(pub U256);

impl From<U256> for NormalizedIncomeV3 {
    fn from(ray: U256) -> Self {
        Self(ray)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NormalizedVariableDebtV3(pub U256);

impl From<U256> for NormalizedVariableDebtV3 {
    fn from(ray: U256) -> Self {
        Self(ray)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReserveDeficitV3(pub U256);

impl From<U256> for ReserveDeficitV3 {
    fn from(amount: U256) -> Self {
        Self(amount)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LiquidationGracePeriodV3(pub U40);

impl From<U40> for LiquidationGracePeriodV3 {
    fn from(deadline: U40) -> Self {
        Self(deadline)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VirtualUnderlyingBalanceV3(pub u128);

impl From<u128> for VirtualUnderlyingBalanceV3 {
    fn from(amount: u128) -> Self {
        Self(amount)
    }
}

/**
 * 
 * 
 * AAVE V4
 * 
 * 
 */
#[derive(Debug)]
pub struct AssetV4 {
    pub liquidity: U120,
    pub realized_fees: U120,
    pub decimals: u8,
    pub added_shares: U120,
    pub swept: U120,
    pub premium_offset_ray: I200,
    pub drawn_shares: U120,
    pub premium_shares: U120,
    pub liquidity_fee: u16,
    pub drawn_index: U120,
    pub drawn_rate: U96,
    pub last_update_timestamp: U40,
    pub underlying: Address,
    pub ir_strategy: Address,
    pub reinvestment_controller: Address,
    pub fee_receiver: Address,
    pub deficit_ray: U200,
}

impl From<AssetV4Return> for AssetV4 {
    fn from(r: AssetV4Return) -> Self {
        Self {
            liquidity: r.liquidity,
            realized_fees: r.realizedFees,
            decimals: r.decimals,
            added_shares: r.addedShares,
            swept: r.swept,
            premium_offset_ray: r.premiumOffsetRay,
            drawn_shares: r.drawnShares,
            premium_shares: r.premiumShares,
            liquidity_fee: r.liquidityFee,
            drawn_index: r.drawnIndex,
            drawn_rate: r.drawnRate,
            last_update_timestamp: r.lastUpdateTimestamp,
            underlying: r.underlying,
            ir_strategy: r.irStrategy,
            reinvestment_controller: r.reinvestmentController,
            fee_receiver: r.feeReceiver,
            deficit_ray: r.deficitRay,
        }
    }
}

#[derive(Debug)]
pub struct SpokeDataV4 {
    pub drawn_shares: U120,
    pub premium_shares: U120,
    pub premium_offset_ray: I200,
    pub added_shares: U120,
    pub add_cap: U40,
    pub draw_cap: U40,
    pub risk_premium_threshold: U24,
    pub active: bool,
    pub halted: bool,
    pub deficit_ray: U200,
}

impl From<SpokeDataV4Return> for SpokeDataV4 {
    fn from(r: SpokeDataV4Return) -> Self {
        Self {
            drawn_shares: r.drawnShares,
            premium_shares: r.premiumShares,
            premium_offset_ray: r.premiumOffsetRay,
            added_shares: r.addedShares,
            add_cap: r.addCap,
            draw_cap: r.drawCap,
            risk_premium_threshold: r.riskPremiumThreshold,
            active: r.active,
            halted: r.halted,
            deficit_ray: r.deficitRay,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpokeConfigV4 {
    pub add_cap: U40,
    pub draw_cap: U40,
    pub risk_premium_threshold: U24,
    pub active: bool,
    pub halted: bool,
}

impl From<SpokeConfigV4Return> for SpokeConfigV4 {
    fn from(r: SpokeConfigV4Return) -> Self {
        Self {
            add_cap: r.addCap,
            draw_cap: r.drawCap,
            risk_premium_threshold: r.riskPremiumThreshold,
            active: r.active,
            halted: r.halted,
        }
    }
}
