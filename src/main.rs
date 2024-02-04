mod solana_rpc_client;
use solana_rpc_client::SolanaClient;
use solana_sdk::signer::Signer;

const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com/";

#[tokio::main]
async fn main() {
    let client = SolanaClient::connect(DEVNET_RPC_URL);

    println!("Balance: {} SOL", client.get_balance(&client.admin.pubkey()));
}
