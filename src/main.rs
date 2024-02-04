use solana_program::{
    instruction::Instruction,
    pubkey::Pubkey
};

use solana_sdk::{
    signature::read_keypair_file, signer::Signer, transaction::Transaction
};

mod solana_rpc_client;
use solana_rpc_client::SolanaClient;

const RPC_DEVNET_URL: &str = "https://api.devnet.solana.com/";

#[tokio::main]
async fn main() {    

    let system_id: Pubkey = Pubkey::default();

    let client = SolanaClient::new(RPC_DEVNET_URL);

    let keypair1 = read_keypair_file("./wallets/w1.json").unwrap();
    let keypair2 = read_keypair_file("./wallets/w2.json").unwrap();

    let (pubkey1, pubkey2) = (keypair1.pubkey(), keypair2.pubkey());
    let (sender, recipient) = (
        client.describe_account(&pubkey1, true),
        client.describe_account(&pubkey2, false)
    );

    // 42 lamports
    let instruction_data = &[2,0,0,0,42,0,0,0,0,0,0,0];

    let ix = Instruction::new_with_bytes(
        system_id,
        instruction_data,
        vec![sender, recipient]
    );
    
    let signers = [&keypair1];

    let tx = Transaction::new_signed_with_payer(
        &[ix], 
        Some(&pubkey1), 
        &signers, 
        client.get_latest_hash()
    );

    let sig = client.send_tx(&tx);

    println!("Signature: {}", sig);

}
