use rand_core;
use solana_program::{
	hash::Hash, native_token::LAMPORTS_PER_SOL, system_program
};
use solana_client::{client_error::ClientError, rpc_client::RpcClient};
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

	pub fn perform_transaction(&self, tx: &Transaction) -> Result<Signature, ClientError>{
		self.client.send_and_confirm_transaction(tx)
	}

	pub fn get_latest_hash(&self) -> Hash {
		let c: Hash = self.client.get_latest_blockhash().unwrap();
		println!("Recent blockhash: {:?}", c);
		c
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