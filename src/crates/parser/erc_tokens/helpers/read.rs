use sqlx::types::BigDecimal;
use alloy::primitives::Address;
use alloy::providers::DynProvider;
use crate::helpers::math::u256_to_bigdecimal;
use crate::parser::erc_tokens::data::abi::Erc20;
use crate::public_client::client::public_client::PublicClient;

pub async fn get_erc20_metadata(
    token_address: Address,
    provider: DynProvider,
) -> (Option<String>, Option<String>, Option<i16>, Option<BigDecimal>) {
    let erc20_parser = Erc20::new(token_address, provider);
    // .ok() so non-ERC-20 addresses (Aave's ETH sentinel 0xEEEE...) and
    // bytes32-style legacy tokens (MKR) store NULL instead of panicking.
    let name_opt = erc20_parser.name().call().await.ok();
    let symbol_opt = erc20_parser.symbol().call().await.ok();
    let decimals_opt = erc20_parser
        .decimals()
        .call()
        .await
        .ok()
        .map(|d| d as i16);
    let total_supply_opt = match erc20_parser.totalSupply().call().await.ok() {
        Some(ts) => Some(u256_to_bigdecimal(ts).await),
        None => None,
    };

    (name_opt, symbol_opt, decimals_opt, total_supply_opt)
}