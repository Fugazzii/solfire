use rand_core;
use solana_program::{
	hash::Hash, native_token::{sol_to_lamports, LAMPORTS_PER_SOL}, pubkey::Pubkey, system_program
};
use solana_client::{client_error::ClientError, rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{
	signature::{Keypair, Signature}, signer::Signer, system_instruction::create_account, transaction::Transaction
};

pub struct SolanaClient {
	client: RpcClient
}

impl SolanaClient {

	pub fn new(url: &str) -> Self {
		SolanaClient {
			client: RpcClient::new(url) 
		}
	}

	pub fn send_tx(&self, tx: &Transaction) -> Signature {
		match self.client.send_transaction(	tx) {
			Ok(sig) => sig,
			Err(err) => self.handle_err(&err)
		}
	}

	pub fn send_tx_without_preflight(&self, tx: &Transaction) -> Signature {
	    let mut config: RpcSendTransactionConfig = RpcSendTransactionConfig::default();
		config.skip_preflight = true;
		
		match self.client.send_transaction_with_config(	tx, config) {
			Ok(sig) => sig,
			Err(err) => self.handle_err(&err)
		}
	}

	fn handle_err(&self, err: &ClientError) -> Signature {
		let e = err.get_transaction_error().unwrap();
		panic!("Transaction failed. {:?}", e)
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

	// pub fn create_wallet(&self, passphrase: &str) {
	// 	let keypair = Keypair::generate(&mut rand_core::OsRng);
	// 	let recent_blockhash = self.get_latest_hash();
	// 	let instructions = create_account(
	// 		&system_program::id(),
	// 		&keypair.pubkey(),
    //         LAMPORTS_PER_SOL,
	// 		1024,
	// 		&keypair.pubkey()
	// 	);

	// }


}