use alloy::sol;

sol! {
    #[sol(rpc)]
    contract Erc20 {
        function name() public view virtual returns (string memory);
        function symbol() public view virtual returns (string memory);
        function decimals() public view virtual returns (uint8);
        function totalSupply() public view virtual returns (uint256);
        function balanceOf(address account) public view virtual returns (uint256);
        function allowance(address owner, address spender) public view virtual returns (uint256);
    }
}
