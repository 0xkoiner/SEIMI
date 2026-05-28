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

    let _ = &conn.init_db().await.expect("Failed to initialize database");

    let user = &conn
        .insert_user("neko", "neko6@gmail.com")
        .await
        .expect("Failed to insert user");
    println!("Inserted user {:#?}", user);

    let db_all = &conn.read_all().await.expect("Failed to read DB");

    for user in db_all.iter() {
        println!("User: {:#?}", user);
    }

    let db_signle = &conn
        .read_by_id(1)
        .await
        .expect("Failed to read single user");
    println!("Single user: {:#?}", db_signle);

    let updated = conn
        .update_user(2, "neo", "neo@example.com")
        .await
        .expect("Failed to update user");
    println!("Updated user {:#?}", updated);

    // let deleted = conn.delete_by_id(9).await.expect("Failed to delete user");
    // println!("Deleted {deleted} row(s) with id 1");

    //  let db_all_after_delete = &conn.read_all().await.expect("Failed to read DB after delete");
    //  println!("All users after delete: {:#?}", db_all_after_delete);
}
