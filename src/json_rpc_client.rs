use solana_client::rpc_client::RpcClient;
use solana_program::hash::Hash;

pub struct SolanaClient {
	client: RpcClient
}

impl SolanaClient {
	pub fn new(url: &str) -> Self {
		SolanaClient {
			client: RpcClient::new(url) 
		}
	}

	pub fn get_latest_hash(&self) -> Hash {
		let c: Hash = self.client.get_latest_blockhash().unwrap();
		println!("Recent blockhash: {:?}", c);
		c
	}
}