use SEIMI::db::db_engine::sqlx_conn::DBEngine;
use SEIMI::public_client::client::public_client::PublicClient;

#[tokio::main]
async fn main() {
    let public_client = PublicClient::new_public_provider("mainnet", "ethereum")
        .expect("Failed to create public client");
    println!("Public client {:#?}", public_client);

    let conn = DBEngine::build_connection()
        .await
        .expect("Failed to connect to database");
    println!("Database connection established: {:#?}", conn);
   
    DBEngine::init_db(&conn)
        .await
        .expect("Failed to initialize database");

    let user = DBEngine::insert_user(&conn, "neko", "neko@gmail.com")
        .await
        .expect("Failed to insert user");
    println!("Inserted user {:#?}", user);
}
