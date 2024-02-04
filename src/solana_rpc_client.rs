use solana_program::{
    hash::Hash,
    instruction::AccountMeta,
    native_token::{lamports_to_sol, sol_to_lamports},
    pubkey::Pubkey,
    system_program,
};

use solana_client::{
    rpc_client::RpcClient, 
    rpc_config::RpcSendTransactionConfig
};

use solana_sdk::{
    signature::{read_keypair_file, write_keypair_file, Keypair, Signature},
    signer::Signer,
    system_instruction::create_account,
    transaction::Transaction
};

pub struct SolanaClient {
    client: RpcClient,
    pub admin: Keypair
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

    pub fn get_balance(&self, pubkey: &Pubkey) -> f64 {
        match self.client.get_balance(pubkey) {
            Ok(lamports) => {
                let sols = lamports_to_sol(lamports);
                sols
            }
            Err(err) => panic!("Failed to retrieve balance. {:?}", err)
        }
    }

    pub fn get_all_txs(&self, pubkey: &Pubkey) -> Vec<String> {
        match self.client.get_signatures_for_address(pubkey) {
            Ok(txs) => {
                let mut result: Vec<String> = vec![];

                for tx in txs {
                    result.push(tx.signature);
                }

                result
            },
            Err(err) => panic!("Failed to retrieve txs {:?}", err)
        }
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
        let keypair = Keypair::from_bytes(passphrase.as_bytes()).unwrap_or(Keypair::new());
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