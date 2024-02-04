mod solana_rpc_client;
use solana_rpc_client::SolanaClient;

const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com/";

#[tokio::main]
async fn main() {
    let client = SolanaClient::connect(DEVNET_RPC_URL);

    client.create_account("iliamagaria", "w4");
}
