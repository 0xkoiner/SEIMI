use SEIMI::client::public_client::PublicClient;

#[tokio::main]
async fn main() {
    let public_client = PublicClient::new_public_provider("mainnet", "ethereum").expect("Failed to create public client");
    println!("Public client {:#?}", public_client);
}
