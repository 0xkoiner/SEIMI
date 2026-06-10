use alloy::primitives::{Address, B256, U256};
use sqlx::types::BigDecimal;

use SEIMI::db::db_engine::sqlx_conn::DBEngine;
use SEIMI::helpers::{
    math::{calculate_average_apr_ray_aave_v1, ray_to_bps, u256_to_bigdecimal},
    vectors::vec_addr_to_string,
};
use SEIMI::parser::aave::data::abi::{AAVEv1Pool, AAVEv2Pool, AAVEv3Pool, AAVEv4CoreHub};
use SEIMI::parser::aave::types::constants::{
    AAVE_V1_POOL, AAVE_V2_POOL, AAVE_V3_POOL, AAVE_V4_CORE_HUB,
};
use SEIMI::parser::aave::types::structs::{
    AssetV4, LiquidationGracePeriodV3, NormalizedIncomeV2, NormalizedIncomeV3,
    NormalizedVariableDebtV2, NormalizedVariableDebtV3, ReserveConfigurationDataV1,
    ReserveConfigurationDataV2, ReserveConfigurationDataV3, ReserveDataV1, ReserveDataV2,
    ReserveDataV3, ReserveDeficitV3, SpokeConfigV4, SpokeDataV4, VirtualUnderlyingBalanceV3,
};
use SEIMI::public_client::client::public_client::PublicClient;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn".into()),
        )
        .init();

    let public_client = PublicClient::new_public_provider("mainnet", "ethereum")
        .expect("Failed to create public client");
    println!("Public client {:#?}", public_client);

    let aave_parser_v1 = AAVEv1Pool::new(AAVE_V1_POOL, public_client.provider.clone());
    // // let aave_parser_v2 = AAVEv2Pool::new(AAVE_V2_POOL, public_client.provider.clone());
    // // let aave_parser_v3: AAVEv3Pool::AAVEv3PoolInstance<alloy::providers::DynProvider> = AAVEv3Pool::new(AAVE_V3_POOL, public_client.provider.clone());
    // // let aave_hub_v4 = AAVEv4CoreHub::new(AAVE_V4_CORE_HUB, public_client.provider.clone());

    let reserves_v1 = aave_parser_v1
        .getReserves()
        .call()
        .await
        .expect("Failed to call getReserves");

    println!("Reserves: {:#?}", reserves_v1);

    for reserve in reserves_v1.clone() {
        let reserve_data: ReserveDataV1 = aave_parser_v1
            .getReserveData(reserve)
            .call()
            .await
            .expect("Failed to call getReserveData")
            .into();
        println!("Reserve: {:#?}, Data: {:#?}", reserve, reserve_data);
    }

    // for reserve in reserves_v1 {
    //     let reserve_config_data: ReserveConfigurationDataV1 = aave_parser_v1
    //         .getReserveConfigurationData(reserve)
    //         .call()
    //         .await
    //         .expect("Failed to call getReserveConfigurationData")
    //         .into();
    //     println!(
    //         "Reserve: {:#?}, Configuration Data: {:#?}",
    //         reserve, reserve_config_data
    //     );
    // }

    // // let reserves_v2 = aave_parser_v2
    // //     .getReservesList()
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getReservesList");

    // // println!("Reserves: {:#?}", reserves_v2);

    // // for reserve in reserves_v2.clone() {
    // //     let reserve_data: ReserveDataV2 = aave_parser_v2
    // //         .getReserveData(reserve)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getReserveData")
    // //         .into();
    // //     println!("Reserve: {:#?}, Data: {:#?}", reserve, reserve_data);
    // // }

    // // for reserve in reserves_v2.clone() {
    // //     let reserve_config_data: ReserveConfigurationDataV2 = aave_parser_v2
    // //         .getConfiguration(reserve)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getReserveConfigurationData")
    // //         .into();
    // //     println!(
    // //         "Reserve: {:#?}, Configuration Data: {:#?}",
    // //         reserve, reserve_config_data
    // //     );
    // // }

    // // for reserve in reserves_v2 {
    // //     let income: NormalizedIncomeV2 = aave_parser_v2
    // //         .getReserveNormalizedIncome(reserve)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getReserveNormalizedIncome")
    // //         .into();
    // //     let variable_debt: NormalizedVariableDebtV2 = aave_parser_v2
    // //         .getReserveNormalizedVariableDebt(reserve)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getReserveNormalizedVariableDebt")
    // //         .into();
    // //     println!(
    // //         "Reserve: {:#?}, NormalizedIncome: {:#?}, NormalizedVariableDebt: {:#?}",
    // //         reserve, income, variable_debt
    // //     );
    // // }

    // // let reserves_v3 = aave_parser_v3
    // //     .getReservesList()
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getReservesList");

    // // println!("Reserves V3: {:#?}", reserves_v3);

    // // for reserve in reserves_v3.clone() {
    // //     let reserve_data: ReserveDataV3 = aave_parser_v3
    // //         .getReserveData(reserve)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getReserveData")
    // //         .into();
    // //     println!("Reserve: {:#?}, Data: {:#?}", reserve, reserve_data);
    // // }

    // // for reserve in reserves_v3.clone() {
    // //     let reserve_config_data: ReserveConfigurationDataV3 = aave_parser_v3
    // //         .getConfiguration(reserve)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getConfiguration")
    // //         .into();
    // //     println!(
    // //         "Reserve: {:#?}, Configuration Data: {:#?}",
    // //         reserve, reserve_config_data
    // //     );
    // // }

    // // for reserve in reserves_v3 {
    // //     let income: NormalizedIncomeV3 = aave_parser_v3
    // //         .getReserveNormalizedIncome(reserve)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getReserveNormalizedIncome")
    // //         .into();
    // //     let variable_debt: NormalizedVariableDebtV3 = aave_parser_v3
    // //         .getReserveNormalizedVariableDebt(reserve)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getReserveNormalizedVariableDebt")
    // //         .into();
    // //     let a_token = aave_parser_v3
    // //         .getReserveAToken(reserve)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getReserveAToken");
    // //     let deficit: ReserveDeficitV3 = aave_parser_v3
    // //         .getReserveDeficit(reserve)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getReserveDeficit")
    // //         .into();
    // //     let grace_period: LiquidationGracePeriodV3 = aave_parser_v3
    // //         .getLiquidationGracePeriod(reserve)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getLiquidationGracePeriod")
    // //         .into();
    // //     let virtual_balance: VirtualUnderlyingBalanceV3 = aave_parser_v3
    // //         .getVirtualUnderlyingBalance(reserve)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getVirtualUnderlyingBalance")
    // //         .into();
    // //     println!(
    // //         "Reserve: {:#?}\n  NormalizedIncome: {:#?}\n  NormalizedVariableDebt: {:#?}\n  aToken: {:#?}\n  Deficit: {:#?}\n  GracePeriod: {:#?}\n  VirtualUnderlyingBalance: {:#?}",
    // //         reserve, income, variable_debt, a_token, deficit, grace_period, virtual_balance
    // //     );
    // // }

    // // let asset_id_v4 = U256::ZERO;

    // // let asset_v4: AssetV4 = aave_hub_v4
    // //     .getAsset(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAsset")
    // //     .into();
    // // println!("V4 asset {:#?}: {:#?}", asset_id_v4, asset_v4);

    // // let underlying_listed_v4 = aave_hub_v4
    // //     .isUnderlyingListed(asset_v4.underlying)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call isUnderlyingListed");
    // // let resolved_asset_id_v4 = aave_hub_v4
    // //     .getAssetId(asset_v4.underlying)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetId");
    // // println!(
    // //     "V4 underlying: {:#?}, listed: {underlying_listed_v4}, resolved assetId: {resolved_asset_id_v4}",
    // //     asset_v4.underlying
    // // );

    // // let added_assets_v4 = aave_hub_v4
    // //     .getAddedAssets(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAddedAssets");
    // // let added_shares_v4 = aave_hub_v4
    // //     .getAddedShares(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAddedShares");
    // // let accrued_fees_v4 = aave_hub_v4
    // //     .getAssetAccruedFees(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetAccruedFees");
    // // let asset_count_v4 = aave_hub_v4
    // //     .getAssetCount()
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetCount");
    // // let asset_deficit_ray_v4 = aave_hub_v4
    // //     .getAssetDeficitRay(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetDeficitRay");
    // // let asset_drawn_index_v4 = aave_hub_v4
    // //     .getAssetDrawnIndex(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetDrawnIndex");
    // // let asset_drawn_rate_v4 = aave_hub_v4
    // //     .getAssetDrawnRate(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetDrawnRate");
    // // let asset_drawn_shares_v4 = aave_hub_v4
    // //     .getAssetDrawnShares(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetDrawnShares");
    // // let asset_liquidity_v4 = aave_hub_v4
    // //     .getAssetLiquidity(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetLiquidity");
    // // let asset_swept_v4 = aave_hub_v4
    // //     .getAssetSwept(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetSwept");
    // // let asset_total_owed_v4 = aave_hub_v4
    // //     .getAssetTotalOwed(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetTotalOwed");
    // // let asset_owed_v4 = aave_hub_v4
    // //     .getAssetOwed(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetOwed");
    // // let asset_premium_data_v4 = aave_hub_v4
    // //     .getAssetPremiumData(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetPremiumData");
    // // let asset_premium_ray_v4 = aave_hub_v4
    // //     .getAssetPremiumRay(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetPremiumRay");
    // // let asset_underlying_and_decimals_v4 = aave_hub_v4
    // //     .getAssetUnderlyingAndDecimals(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getAssetUnderlyingAndDecimals");
    // // println!(
    // //     "V4 asset {asset_id_v4}:\n  added_assets={added_assets_v4}\n  added_shares={added_shares_v4}\n  accrued_fees={accrued_fees_v4}\n  asset_count={asset_count_v4}\n  deficit_ray={asset_deficit_ray_v4}\n  drawn_index={asset_drawn_index_v4}\n  drawn_rate={asset_drawn_rate_v4}\n  drawn_shares={asset_drawn_shares_v4}\n  liquidity={asset_liquidity_v4}\n  swept={asset_swept_v4}\n  total_owed={asset_total_owed_v4}\n  owed=({}, {})\n  premium_data=({}, {})\n  premium_ray={asset_premium_ray_v4}\n  underlying_and_decimals=({}, {})",
    // //     asset_owed_v4._0,
    // //     asset_owed_v4._1,
    // //     asset_premium_data_v4._0,
    // //     asset_premium_data_v4._1,
    // //     asset_underlying_and_decimals_v4._0,
    // //     asset_underlying_and_decimals_v4._1,
    // // );

    // // let spoke_count_v4 = aave_hub_v4
    // //     .getSpokeCount(asset_id_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call getSpokeCount");
    // // println!("V4 spoke count for asset {asset_id_v4}: {spoke_count_v4}");

    // // let mut spoke_idx_v4 = U256::ZERO;
    // // let one_v4 = U256::from(1u64);
    // // while spoke_idx_v4 < spoke_count_v4 {
    // //     // `getSpokeAddress` returns `uint256` per the contract stub; convert to `Address` (low 20 bytes).
    // //     let spoke_addr_raw_v4 = aave_hub_v4
    // //         .getSpokeAddress(asset_id_v4, spoke_idx_v4)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getSpokeAddress");
    // //     let spoke_addr_v4 = Address::from_word(B256::from(spoke_addr_raw_v4));

    // //     let spoke_listed_v4 = aave_hub_v4
    // //         .isSpokeListed(asset_id_v4, spoke_addr_v4)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call isSpokeListed");

    // //     let spoke_data_v4: SpokeDataV4 = aave_hub_v4
    // //         .getSpoke(asset_id_v4, spoke_addr_v4)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getSpoke")
    // //         .into();
    // //     let spoke_config_v4: SpokeConfigV4 = aave_hub_v4
    // //         .getSpokeConfig(asset_id_v4, spoke_addr_v4)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getSpokeConfig")
    // //         .into();

    // //     let spoke_added_assets_v4 = aave_hub_v4
    // //         .getSpokeAddedAssets(asset_id_v4, spoke_addr_v4)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getSpokeAddedAssets");
    // //     let spoke_added_shares_v4 = aave_hub_v4
    // //         .getSpokeAddedShares(asset_id_v4, spoke_addr_v4)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getSpokeAddedShares");
    // //     let spoke_deficit_ray_v4 = aave_hub_v4
    // //         .getSpokeDeficitRay(asset_id_v4, spoke_addr_v4)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getSpokeDeficitRay");
    // //     let spoke_drawn_shares_v4 = aave_hub_v4
    // //         .getSpokeDrawnShares(asset_id_v4, spoke_addr_v4)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getSpokeDrawnShares");
    // //     let spoke_owed_v4 = aave_hub_v4
    // //         .getSpokeOwed(asset_id_v4, spoke_addr_v4)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getSpokeOwed");
    // //     let spoke_premium_data_v4 = aave_hub_v4
    // //         .getSpokePremiumData(asset_id_v4, spoke_addr_v4)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getSpokePremiumData");
    // //     let spoke_premium_ray_v4 = aave_hub_v4
    // //         .getSpokePremiumRay(asset_id_v4, spoke_addr_v4)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getSpokePremiumRay");
    // //     let spoke_total_owed_v4 = aave_hub_v4
    // //         .getSpokeTotalOwed(asset_id_v4, spoke_addr_v4)
    // //         .call()
    // //         .await
    // //         .expect("Failed to call getSpokeTotalOwed");

    // //     println!(
    // //         "V4 spoke[{spoke_idx_v4}] addr={spoke_addr_v4} listed={spoke_listed_v4}\n  data={spoke_data_v4:#?}\n  config={spoke_config_v4:#?}\n  added_assets={spoke_added_assets_v4}\n  added_shares={spoke_added_shares_v4}\n  deficit_ray={spoke_deficit_ray_v4}\n  drawn_shares={spoke_drawn_shares_v4}\n  owed=({}, {})\n  premium_data=({}, {})\n  premium_ray={spoke_premium_ray_v4}\n  total_owed={spoke_total_owed_v4}",
    // //         spoke_owed_v4._0,
    // //         spoke_owed_v4._1,
    // //         spoke_premium_data_v4._0,
    // //         spoke_premium_data_v4._1,
    // //     );

    // //     spoke_idx_v4 += one_v4;
    // // }

    // // let preview_amount_v4 = U256::from(1_000_000u64);
    // // let preview_shares_v4 = U256::from(1_000_000u64);
    // // let preview_add_by_assets_v4 = aave_hub_v4
    // //     .previewAddByAssets(asset_id_v4, preview_amount_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call previewAddByAssets");
    // // let preview_add_by_shares_v4 = aave_hub_v4
    // //     .previewAddByShares(asset_id_v4, preview_shares_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call previewAddByShares");
    // // let preview_draw_by_assets_v4 = aave_hub_v4
    // //     .previewDrawByAssets(asset_id_v4, preview_amount_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call previewDrawByAssets");
    // // let preview_remove_by_assets_v4 = aave_hub_v4
    // //     .previewRemoveByAssets(asset_id_v4, preview_amount_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call previewRemoveByAssets");
    // // let preview_remove_by_shares_v4 = aave_hub_v4
    // //     .previewRemoveByShares(asset_id_v4, preview_shares_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call previewRemoveByShares");
    // // let preview_restore_by_assets_v4 = aave_hub_v4
    // //     .previewRestoreByAssets(asset_id_v4, preview_amount_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call previewRestoreByAssets");
    // // let preview_restore_by_shares_v4 = aave_hub_v4
    // //     .previewRestoreByShares(asset_id_v4, preview_shares_v4)
    // //     .call()
    // //     .await
    // //     .expect("Failed to call previewRestoreByShares");
    // // println!(
    // //     "V4 previews for asset {asset_id_v4} (sample amount={preview_amount_v4}, shares={preview_shares_v4}):\n  add_by_assets={preview_add_by_assets_v4}\n  add_by_shares={preview_add_by_shares_v4}\n  draw_by_assets={preview_draw_by_assets_v4}\n  remove_by_assets={preview_remove_by_assets_v4}\n  remove_by_shares={preview_remove_by_shares_v4}\n  restore_by_assets={preview_restore_by_assets_v4}\n  restore_by_shares={preview_restore_by_shares_v4}"
    // // );

    let conn = DBEngine::build_connection()
        .await
        .expect("Failed to connect to database");
    println!("Database connection established: {:#?}", conn);

    let _ = &conn
        .delete_all_tables()
        .await
        .expect("Failed to delete all tables");

    let _ = &conn.init_db().await.expect("Failed to initialize database");

    let protocol = &conn
        .insert_protocols(
            "AAVEv1",
            "AAVE-V1",
            "Lending",
            Some("src/crates/parser/aave/data/abi_aave_v1.json"),
        )
        .await
        .expect("Failed to insert protocol");
    println!("Inserted protocol {:#?}", protocol);

    let chain_ethereum = &conn
        .insert_chains("Ethereum", 1)
        .await
        .expect("Failed to insert chain");
    println!("Inserted chain {:#?}", chain_ethereum);

    let protocol_chain = &conn
        .insert_protocol_chains(1, chain_ethereum.id)
        .await
        .expect("Failed to insert protocol_chain");
    println!("Inserted protocol_chain {:#?}", protocol_chain);

    let reserves_v1_csv: String = vec_addr_to_string(&reserves_v1).await;

    let (avg_apr_ray, total_liquidity_sum) =
        calculate_average_apr_ray_aave_v1(&aave_parser_v1, &reserves_v1).await;

    let markets = &conn
        .insert_markets(
            protocol.id,
            chain_ethereum.id,
            &AAVE_V1_POOL.to_string(),
            "lending",
            &reserves_v1_csv,
        )
        .await
        .expect("Failed to insert markets");
    println!("Inserted markets {:#?}", markets);

    let market_metrics_ts = &conn
        .insert_market_metrics_ts(
            markets.id,
            u256_to_bigdecimal(total_liquidity_sum).await,
            BigDecimal::from(0), // TODO(volume): scan Deposit/Withdraw/Borrow/Repay events
            ray_to_bps(avg_apr_ray).await, // apy_bps (TODO: compound)
            ray_to_bps(avg_apr_ray).await, // apr_bps (supply, TVL-weighted)
            "aave_v1:getReserveData:ethereum",
            "tier1",
        )
        .await
        .expect("Failed to insert market_metrics_ts");
    println!("Inserted market_metrics_ts {:#?}", market_metrics_ts);

    // let protocol_metrics_ts = &conn
    //     .insert_protocol_metrics_ts(protocol.id, 20, 30, "my granny", "good-one")
    //     .await
    //     .expect("Failed to insert protocol_metrics_ts");
    // println!("Inserted protocol_metrics_ts {:#?}", protocol_metrics_ts);

    // let aggregate_metrics_ts = &conn
    //     .insert_aggregate_metrics_ts(20, 30, 40, "my granny", "good-one")
    //     .await
    //     .expect("Failed to insert aggregate_metrics_ts");
    // println!("Inserted aggregate_metrics_ts {:#?}", aggregate_metrics_ts);

    // let volume_rollups = &conn
    //     .insert_volume_rollups("scope", 1, "window", 60)
    //     .await
    //     .expect("Failed to insert volume_rollups");
    // println!("Inserted volume_rollups {:#?}", volume_rollups);
}
