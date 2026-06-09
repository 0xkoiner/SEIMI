use SEIMI::parser::aave::data::abi::{AAVEv1Pool, AAVEv2Pool, AAVEv3Pool};
use SEIMI::parser::aave::types::constants::{AAVE_V1_POOL, AAVE_V2_POOL, AAVE_V3_POOL};
use SEIMI::parser::aave::types::structs::{
    LiquidationGracePeriodV3, NormalizedIncomeV2, NormalizedIncomeV3, NormalizedVariableDebtV2,
    NormalizedVariableDebtV3, ReserveConfigurationDataV1, ReserveConfigurationDataV2,
    ReserveConfigurationDataV3, ReserveDataV1, ReserveDataV2, ReserveDataV3, ReserveDeficitV3,
    VirtualUnderlyingBalanceV3,
};
use SEIMI::public_client::client::public_client::PublicClient;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt()
    //     .with_env_filter(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "info,sqlx=warn".into()),
    //     )
    //     .init();

    let public_client = PublicClient::new_public_provider("mainnet", "ethereum")
        .expect("Failed to create public client");
    println!("Public client {:#?}", public_client);

    let aave_parser_v1 = AAVEv1Pool::new(AAVE_V1_POOL, public_client.provider.clone());
    let aave_parser_v2 = AAVEv2Pool::new(AAVE_V2_POOL, public_client.provider.clone());

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

    for reserve in reserves_v1 {
        let reserve_config_data: ReserveConfigurationDataV1 = aave_parser_v1
            .getReserveConfigurationData(reserve)
            .call()
            .await
            .expect("Failed to call getReserveConfigurationData")
            .into();
        println!(
            "Reserve: {:#?}, Configuration Data: {:#?}",
            reserve, reserve_config_data
        );
    }

    let reserves_v2 = aave_parser_v2
        .getReservesList()
        .call()
        .await
        .expect("Failed to call getReservesList");

    println!("Reserves: {:#?}", reserves_v2);

    for reserve in reserves_v2.clone() {
        let reserve_data: ReserveDataV2 = aave_parser_v2
            .getReserveData(reserve)
            .call()
            .await
            .expect("Failed to call getReserveData")
            .into();
        println!("Reserve: {:#?}, Data: {:#?}", reserve, reserve_data);
    }

    for reserve in reserves_v2.clone() {
        let reserve_config_data: ReserveConfigurationDataV2 = aave_parser_v2
            .getConfiguration(reserve)
            .call()
            .await
            .expect("Failed to call getReserveConfigurationData")
            .into();
        println!(
            "Reserve: {:#?}, Configuration Data: {:#?}",
            reserve, reserve_config_data
        );
    }

    for reserve in reserves_v2 {
        let income: NormalizedIncomeV2 = aave_parser_v2
            .getReserveNormalizedIncome(reserve)
            .call()
            .await
            .expect("Failed to call getReserveNormalizedIncome")
            .into();
        let variable_debt: NormalizedVariableDebtV2 = aave_parser_v2
            .getReserveNormalizedVariableDebt(reserve)
            .call()
            .await
            .expect("Failed to call getReserveNormalizedVariableDebt")
            .into();
        println!(
            "Reserve: {:#?}, NormalizedIncome: {:#?}, NormalizedVariableDebt: {:#?}",
            reserve, income, variable_debt
        );
    }

    let aave_parser_v3 = AAVEv3Pool::new(AAVE_V3_POOL, public_client.provider.clone());

    let reserves_v3 = aave_parser_v3
        .getReservesList()
        .call()
        .await
        .expect("Failed to call getReservesList");

    println!("Reserves V3: {:#?}", reserves_v3);

    for reserve in reserves_v3.clone() {
        let reserve_data: ReserveDataV3 = aave_parser_v3
            .getReserveData(reserve)
            .call()
            .await
            .expect("Failed to call getReserveData")
            .into();
        println!("Reserve: {:#?}, Data: {:#?}", reserve, reserve_data);
    }

    for reserve in reserves_v3.clone() {
        let reserve_config_data: ReserveConfigurationDataV3 = aave_parser_v3
            .getConfiguration(reserve)
            .call()
            .await
            .expect("Failed to call getConfiguration")
            .into();
        println!(
            "Reserve: {:#?}, Configuration Data: {:#?}",
            reserve, reserve_config_data
        );
    }

    for reserve in reserves_v3 {
        let income: NormalizedIncomeV3 = aave_parser_v3
            .getReserveNormalizedIncome(reserve)
            .call()
            .await
            .expect("Failed to call getReserveNormalizedIncome")
            .into();
        let variable_debt: NormalizedVariableDebtV3 = aave_parser_v3
            .getReserveNormalizedVariableDebt(reserve)
            .call()
            .await
            .expect("Failed to call getReserveNormalizedVariableDebt")
            .into();
        let a_token = aave_parser_v3
            .getReserveAToken(reserve)
            .call()
            .await
            .expect("Failed to call getReserveAToken");
        let deficit: ReserveDeficitV3 = aave_parser_v3
            .getReserveDeficit(reserve)
            .call()
            .await
            .expect("Failed to call getReserveDeficit")
            .into();
        let grace_period: LiquidationGracePeriodV3 = aave_parser_v3
            .getLiquidationGracePeriod(reserve)
            .call()
            .await
            .expect("Failed to call getLiquidationGracePeriod")
            .into();
        let virtual_balance: VirtualUnderlyingBalanceV3 = aave_parser_v3
            .getVirtualUnderlyingBalance(reserve)
            .call()
            .await
            .expect("Failed to call getVirtualUnderlyingBalance")
            .into();
        println!(
            "Reserve: {:#?}\n  NormalizedIncome: {:#?}\n  NormalizedVariableDebt: {:#?}\n  aToken: {:#?}\n  Deficit: {:#?}\n  GracePeriod: {:#?}\n  VirtualUnderlyingBalance: {:#?}",
            reserve, income, variable_debt, a_token, deficit, grace_period, virtual_balance
        );
    }

    // let conn = DBEngine::build_connection()
    //     .await
    //     .expect("Failed to connect to database");
    // println!("Database connection established: {:#?}", conn);

    // let _ = &conn.delete_all_tables().await.expect("Failed to delete all tables");

    // let _ = &conn.init_db().await.expect("Failed to initialize database");

    // let protocol = &conn
    //     .insert_protocols("AAVE4", "AAVE-V3", "Lending", None)
    //     .await
    //     .expect("Failed to insert protocol");
    // println!("Inserted protocol {:#?}", protocol);

    // let chain = &conn
    //     .insert_chains("Ethereum", 1)
    //     .await
    //     .expect("Failed to insert chain");
    // println!("Inserted chain {:#?}", chain);

    // let polygon = &conn
    //     .ensure_chain("Polygon", 137)
    //     .await
    //     .expect("Failed to ensure chain");
    // println!("Ensured chain {:#?}", polygon);

    // let protocol_chain = &conn
    //     .insert_protocol_chains(1, polygon.id)
    //     .await
    //     .expect("Failed to insert protocol_chain");
    // println!("Inserted protocol_chain {:#?}", protocol_chain);

    // let markets = &conn
    //     .insert_markets(
    //         protocol.id,
    //         chain.id,
    //         "0x000000000001",
    //         "lending",
    //         "0x000000000001,0x000000000002",
    //     )
    //     .await
    //     .expect("Failed to insert markets");
    // println!("Inserted markets {:#?}", markets);

    // let market_metrics_ts = &conn
    //     .insert_market_metrics_ts(markets.id, 20, 30, 40, 50, "my granny", "good-one")
    //     .await
    //     .expect("Failed to insert market_metrics_ts");
    // println!("Inserted market_metrics_ts {:#?}", market_metrics_ts);

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
