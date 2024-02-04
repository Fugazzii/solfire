use solana_client::rpc_client;

mod database;
mod json_rpc_client;

use database::{Database, Event};


const JSON_RPC_URL: &str = "https://api.devnet.solana.com/";

#[tokio::main]
async fn main() {
    let _db: Database<Event> = Database::new();

    let client = rpc_client::RpcClient::new(JSON_RPC_URL);

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    println!("Recent blockhash: {:?}", recent_blockhash);
}
