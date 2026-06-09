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