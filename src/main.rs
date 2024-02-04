use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey
};
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
    transaction::Transaction
};

mod solana_rpc_client;
use solana_rpc_client::SolanaClient;

const RPC_DEVNET_URL: &str = "https://api.devnet.solana.com/";

#[tokio::main]
async fn main() {    

    let client = SolanaClient::new(RPC_DEVNET_URL);

    let keypair = Keypair::new();

    let pubkey = keypair.pubkey();
    let address_pk = Pubkey::new_unique();
    let system_id = Pubkey::default();


    // 42 lamports
    let instruction_data = &[2,0,0,0,42,0,0,0,0,0,0,0];
    let instruction_accounts = vec![
        AccountMeta {
            pubkey,
            is_signer: true,
            is_writable: true
        },
        AccountMeta {
            pubkey: address_pk,
            is_signer: false,
            is_writable: true
        }
    ];

    let ix = Instruction::new_with_bytes(system_id, instruction_data, instruction_accounts);
    
    let signers = [&keypair];

    let tx = Transaction::new_signed_with_payer(
        &[ix], 
        Some(&pubkey), 
        &signers, 
        client.get_latest_hash()
    );

    let sig = client.send_tx(&tx);

    println!("Signature: {}", sig);

}
