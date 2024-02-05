// use solana_sdk::signer::Signer;
use actix_web::{get, web, HttpResponse};

use crate::infrastructure::{
	// database::Database,
	solana_rpc_client::SolanaClient
};

#[get("/ping")]
pub async fn ping() -> HttpResponse {
	HttpResponse::Ok().body("pong!\n")
}

#[get("/recent-hash")]
pub async fn recent_hash(rpc: web::Data<SolanaClient>) -> HttpResponse {
	let latest_hash = tokio::task::spawn_blocking(move || rpc.get_latest_hash()).await.unwrap();
	HttpResponse::Ok().body(latest_hash.to_string())
}

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/api")
			.service(ping)
			.service(recent_hash)
	);
}

