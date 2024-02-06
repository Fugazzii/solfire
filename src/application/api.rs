use std::str::FromStr;

use actix_web::{get, post, web::{self, Data, Json}, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use solana_program::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use tokio::task::spawn_blocking;

use crate::infrastructure::{
	// database::Database,
	solana_rpc_client::SolanaClient
};

#[get("/ping")]
pub async fn ping() -> HttpResponse {
	HttpResponse::Ok().body("pong!\n")
}

#[get("/recent-hash")]
pub async fn recent_hash(rpc: Data<SolanaClient>) -> HttpResponse {
	let latest_hash = tokio::task::spawn_blocking(move || rpc.get_latest_hash()).await.unwrap();
	HttpResponse::Ok().body(latest_hash.to_string())
}

#[derive(Deserialize)]
struct SendTxDto {
	sender_keypair_path: String,
	recipient: String,
	sols: f64
}

#[post("/tx")]
pub async fn send_tx(
	rpc: Data<SolanaClient>,
	send_tx_dto: Json<SendTxDto>
) -> HttpResponse {
	println!("Sending...");
	let SendTxDto { sender_keypair_path, recipient, sols } = send_tx_dto.0;
	let recipient_pk = Pubkey::from_str(&recipient).unwrap();
	let rpc_clone = rpc.clone();
	
	let keypair = spawn_blocking(move || {
		rpc_clone.read_keypair(sender_keypair_path.as_str())
	}).await.unwrap();

	let signature = spawn_blocking(move || {
		rpc.send_tx(&keypair, &recipient_pk, sols)
	}).await.unwrap();

	HttpResponse::Ok().json(json!({
		"signature": signature
	}))
}

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/api")
			.service(ping)
			.service(recent_hash)
			.service(send_tx)
	);
}

