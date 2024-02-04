use solana_program::{instruction::Instruction, pubkey::Pubkey};

use solana_sdk::{signature::read_keypair_file, signer::Signer, transaction::Transaction};

mod solana_rpc_client;
use solana_rpc_client::SolanaClient;

const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com/";
// const TESTNET_RPC_URL: &str = "https://api.testnet.solana.com/";
// const MAINNET_RPC_URL: &str = "https://api.mainnet.solana.com/";

#[tokio::main]
async fn main() {
    let system_id: Pubkey = Pubkey::default();

    let client = SolanaClient::connect(DEVNET_RPC_URL);

    client.create_account("iliamagaria", "w4");
}
