use serde::Deserialize;

#[derive(Deserialize)]
pub struct SendTxDto {
	pub sender_keypair_path: String,
	pub recipient: String,
	pub sols: f64
}
