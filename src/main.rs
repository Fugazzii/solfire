mod database;
mod json_rpc_client;

use database::{Database, Event};
use json_rpc_client::SolanaClient;

const JSON_RPC_URL: &str = "https://api.devnet.solana.com/";

#[tokio::main]
async fn main() {
    let _db: Database<Event> = Database::new();

    let client = SolanaClient::new(JSON_RPC_URL);

    client.get_latest_hash();
}
