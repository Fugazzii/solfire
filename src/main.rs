/** External imports */
use dotenv;
use solana_sdk::signer::Signer;

/** Local imports */
mod infrastructure;
use infrastructure::{
    database::Database,
    solana_rpc_client::SolanaClient
};

const ENV: &str = "./env/.env.dev"; 

#[tokio::main]
async fn main() {
    dotenv::from_path(ENV).ok();
    
    let db = Database::new();

    let rpc_url = dotenv::var("JSON_RPC_URL");

    let client = SolanaClient::connect(rpc_url.unwrap().as_str());

    let txs = client.get_all_txs(&client.admin.pubkey());

    println!("{:?}", txs);
}
