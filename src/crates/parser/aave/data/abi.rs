use alloy::sol;

sol! {
    #[sol(rpc)]
    contract AAVEv1Pool {
        function getReserves() external view returns (address[] memory);
        function getReserveDecimals(address _reserve) external view returns (uint256);
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
    }
}
