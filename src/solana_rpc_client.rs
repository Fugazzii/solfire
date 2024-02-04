#[allow(unused_imports)]
use rand_core;

#[allow(unused_imports)]
use solana_program::hash::Hash;
#[allow(unused_imports)]
use solana_program::instruction::AccountMeta;
#[allow(unused_imports)]
use solana_program::native_token::sol_to_lamports;
#[allow(unused_imports)]
use solana_program::native_token::LAMPORTS_PER_SOL;
#[allow(unused_imports)]
use solana_program::pubkey::Pubkey;
#[allow(unused_imports)]
use solana_program::system_program;

#[allow(unused_imports)]
use solana_client::client_error::ClientError;
#[allow(unused_imports)]
use solana_client::rpc_client::RpcClient;
#[allow(unused_imports)]
use solana_client::rpc_config::RpcSendTransactionConfig;

#[allow(unused_imports)]
use solana_sdk::signature::Keypair;
#[allow(unused_imports)]
use solana_sdk::signature::Signature;
use solana_sdk::signature::{read_keypair_file, write_keypair_file};
#[allow(unused_imports)]
use solana_sdk::signer::Signer;
#[allow(unused_imports)]
use solana_sdk::system_instruction::create_account;
#[allow(unused_imports)]
use solana_sdk::transaction::Transaction;

pub struct SolanaClient {
    client: RpcClient,
    admin: Keypair
}

#[allow(dead_code)]
impl SolanaClient {
    pub fn connect(url: &str) -> Self {
        SolanaClient {
            client: RpcClient::new(url),
            admin: read_keypair_file("./wallets/w1.json").unwrap()
        }
    }

    pub fn read_keypair(&self, file_path: &str) -> Keypair {
        match read_keypair_file(file_path) {
            Ok(keypair) => keypair,
            Err(err) => {
                panic!("Failed to read keypair file. {:?}", err.to_string())
            }
        }
    }

    pub fn send_tx(&self, tx: &Transaction) -> Signature {
        match self.client.send_transaction(tx) {
            Ok(sig) => sig,
            Err(err) => {
                let e = err.get_transaction_error().unwrap();
                panic!("Failed to send tx. {:?}", e)
            }
        }
    }

    pub fn send_tx_without_preflight(&self, tx: &Transaction) -> Signature {
        let mut config: RpcSendTransactionConfig = RpcSendTransactionConfig::default();
        config.skip_preflight = true;

        match self.client.send_transaction_with_config(tx, config) {
            Ok(sig) => sig,
            Err(err) => {
                let e = err.get_transaction_error().unwrap();
                panic!("Failed to send tx. {:?}", e)
            }
        }
    }

    pub fn get_latest_hash(&self) -> Hash {
        let c: Hash = self.client.get_latest_blockhash().unwrap();
        println!("Recent blockhash: {:?}", c);
        c
    }

    pub fn airdrop(&self, pubkey: &Pubkey, sols: u32) -> Signature {
        let lamports = sol_to_lamports(sols as f64);
        match self.client.request_airdrop(pubkey, lamports) {
            Ok(sig) => {
                println!("Airdropped 1 SOL\nSignature: {}", sig);
                sig
            }
            Err(err) => panic!("Failed to airdrop. {:?}", err)
        }
    }

    pub fn describe_account(&self, pubkey: &Pubkey, is_signer: bool) -> AccountMeta {
        AccountMeta {
            pubkey: *pubkey,
            is_signer,
            is_writable: true
        }
    }

    pub fn create_account(&self, passphrase: &str, file_name: &str) {
        let keypair = Keypair::from_bytes(passphrase.as_bytes()).unwrap_or(
            // Default keypair
            Keypair::generate(&mut rand_core::OsRng)
        );
        let recent_blockhash = self.get_latest_hash();
        let lamports = sol_to_lamports(0.05);
        let space = 1024;
        write_keypair_file(&keypair, format!("./wallets/{}.json", file_name)).unwrap();

        let ix = create_account(
            &self.admin.pubkey(),
            &keypair.pubkey(),
            lamports,
            space,
            &system_program::id()
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.admin.pubkey()),
            &[&self.admin],
            recent_blockhash
        );

        let result = self.client.send_and_confirm_transaction(&tx);
        match result {
            Ok(signature) => {
                println!("Transaction confirmed. Signature: {:?}", signature);
            }
            Err(err) => {
                println!("Transaction failed. Error: {:?}", err);
            }
        }
    }
}
