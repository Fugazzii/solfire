use serde::Deserialize;

#[derive(Deserialize)]
pub struct SendTxDto {
	pub sender_keypair_path: String,
	pub recipient: String,
	pub sols: f64
}

#[derive(Deserialize)]
pub struct PublicKeyQuery {
	pub pubkey: String
}

#[derive(Deserialize)]
pub struct AirdropSolDto {
	pub recipient: String,
	pub sols: f64
}