use dotenv;

mod solana_rpc_client;
use solana_rpc_client::SolanaClient;

const ENV: &str = "./env/.env.dev"; 

#[tokio::main]
async fn main() {
    dotenv::from_path(ENV).ok();
    
    let rpc_url = dotenv::var("JSON_RPC_URL");

    let client = SolanaClient::connect(rpc_url.unwrap().as_str());
}
