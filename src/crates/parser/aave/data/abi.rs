use alloy::sol;

sol! {
    #[sol(rpc)]
    contract AAVEv1Pool {
        function getReserves() external view returns (address[] memory);
        function getReserveData(
            address _reserve
        )
        external
        view
        returns (
            uint256 totalLiquidity,
            uint256 availableLiquidity,
            uint256 totalBorrowsStable,
            uint256 totalBorrowsVariable,
            uint256 liquidityRate,
            uint256 variableBorrowRate,
            uint256 stableBorrowRate,
            uint256 averageStableBorrowRate,
            uint256 utilizationRate,
            uint256 liquidityIndex,
            uint256 variableBorrowIndex,
            address aTokenAddress,
            uint40 lastUpdateTimestamp
        );
        function getReserveConfigurationData(
            address _reserve
        )
            external
            view
            returns (
            uint256 ltv,
            uint256 liquidationThreshold,
            uint256 liquidationBonus,
            address interestRateStrategyAddress,
            bool usageAsCollateralEnabled,
            bool borrowingEnabled,
            bool stableBorrowRateEnabled,
            bool isActive
            );
    }
}

sol! {
    #[sol(rpc)]
    contract AAVEv2Pool {
        function getReservesList() external view returns (address[] memory);
        function getReserveData(
            address _reserve
        )
        external
        view
        returns (
            uint256 configuration,
            uint128 liquidityIndex,
            uint128 variableBorrowIndex,
            uint128 currentLiquidityRate,
            uint128 currentVariableBorrowRate,
            uint128 currentStableBorrowRate,
            uint40 lastUpdateTimestamp,
            address aTokenAddress,
            address stableDebtTokenAddress,
            address variableDebtTokenAddress,
            address interestRateStrategyAddress,
            uint8 id
        );
        function getConfiguration(address asset) external view returns (uint256 data);
        function getReserveNormalizedIncome(address asset) external view returns (uint256);
        function getReserveNormalizedVariableDebt(address asset) external view returns (uint256);
    }
}

sol! {
    #[sol(rpc)]
    contract AAVEv3Pool {
        function getReservesList() external view returns (address[] memory);
        function getReserveData(
            address _reserve
        )
        external
        view
        returns (
            uint256 configuration,
            uint128 liquidityIndex,
            uint128 currentLiquidityRate,
            uint128 variableBorrowIndex,
            uint128 currentVariableBorrowRate,
            uint128 currentStableBorrowRate,
            uint40 lastUpdateTimestamp,
            uint16 id,
            address aTokenAddress,
            address stableDebtTokenAddress,
            address variableDebtTokenAddress,
            address interestRateStrategyAddress,
            uint128 accruedToTreasury,
            uint128 unbacked,
            uint128 isolationModeTotalDebt
        );
        function getConfiguration(address asset) external view returns (uint256 data);
        function getLiquidationGracePeriod(address asset) external view returns (uint40);
        function getReserveAToken(address asset) external view returns (address);
        function getReserveDeficit(address asset) external view returns (uint256);
        function getReserveNormalizedIncome(address asset) external view returns (uint256);
        function getReserveNormalizedVariableDebt(address asset) external view returns (uint256);
        function getVirtualUnderlyingBalance(address asset) external view returns (uint128);
    }
}

sol! {
    #[sol(rpc)]
    contract AAVEv4CoreHub {
        function getAddedAssets(uint256 assetId) external view returns (uint256);
        function getAddedShares(uint256 assetId) external view returns (uint256);
        function getAsset(uint256 assetId) external view returns (
            uint120 liquidity,
            uint120 realizedFees,
            uint8 decimals,
            uint120 addedShares,
            uint120 swept,
            int200 premiumOffsetRay,
            uint120 drawnShares,
            uint120 premiumShares,
            uint16 liquidityFee,
            uint120 drawnIndex,
            uint96 drawnRate,
            uint40 lastUpdateTimestamp,
            address underlying,
            address irStrategy,
            address reinvestmentController,
            address feeReceiver,
            uint200 deficitRay
        );
        function getSpoke(uint256 assetId, address spoke) external view returns (
            uint120 drawnShares,
            uint120 premiumShares,
            int200 premiumOffsetRay,
            uint120 addedShares,
            uint40 addCap,
            uint40 drawCap,
            uint24 riskPremiumThreshold,
            bool active,
            bool halted,
            uint200 deficitRay
        );
        function getAssetAccruedFees(uint256 assetId) external view returns (uint256);
        function getAssetCount() external view returns (uint256);
        function getAssetDeficitRay(uint256 assetId) external view returns (uint256);
        function getAssetDrawnIndex(uint256 assetId) external view returns (uint256);
        function getAssetDrawnRate(uint256 assetId) external view returns (uint256);
        function getAssetDrawnShares(uint256 assetId) external view returns (uint256);
        function getAssetId(address underlying) external view returns (uint256);
        function getAssetLiquidity(uint256 assetId) external view returns (uint256);
        function getAssetOwed(uint256 assetId) external view returns (uint256, uint256);
        function getAssetPremiumData(uint256 assetId) external view returns (uint256, uint256);
        function getAssetPremiumRay(uint256 assetId) external view returns (uint256);
        function getAssetSwept(uint256 assetId) external view returns (uint256);
        function getAssetTotalOwed(uint256 assetId) external view returns (uint256);
        function getAssetUnderlyingAndDecimals(uint256 assetId) external view returns (address, uint8);
        function getSpokeAddedAssets(uint256 assetId, address spoke) external view returns (uint256);
        function getSpokeAddedShares(uint256 assetId, address spoke) external view returns (uint256);
        function getSpokeAddress(uint256 assetId, uint256 index) external view returns (uint256);
        function getSpokeConfig(uint256 assetId, address spoke) external view returns (
            uint40 addCap,
            uint40 drawCap,
            uint24 riskPremiumThreshold,
            bool active,
            bool halted
        );
        function getSpokeCount(uint256 assetId) external view returns (uint256);
        function getSpokeDeficitRay(uint256 assetId, address spoke) external view returns (uint256);
        function getSpokeDrawnShares(uint256 assetId, address spoke) external view returns (uint256);
        function getSpokeOwed(uint256 assetId, address spoke) external view returns (uint256, uint256);
        function getSpokePremiumData(uint256 assetId, address spoke) external view returns (uint256, int256);
        function getSpokePremiumRay(uint256 assetId, address spoke) external view returns (uint256);
        function getSpokeTotalOwed(uint256 assetId, address spoke) external view returns (uint256);
        function isSpokeListed(uint256 assetId, address spoke) external view returns (bool);
        function isUnderlyingListed(address underlying) public view returns (bool);
        function previewAddByAssets(uint256 assetId, uint256 assets) external view returns (uint256);
        function previewAddByShares(uint256 assetId, uint256 shares) external view returns (uint256);
        function previewDrawByAssets(uint256 assetId, uint256 assets) external view returns (uint256);
        function previewRemoveByAssets(uint256 assetId, uint256 assets) external view returns (uint256);
        function previewRemoveByShares(uint256 assetId, uint256 shares) external view returns (uint256);
        function previewRestoreByAssets(uint256 assetId, uint256 assets) external view returns (uint256);
        function previewRestoreByShares(uint256 assetId, uint256 shares) external view returns (uint256);
    }
}
