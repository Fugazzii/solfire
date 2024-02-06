use std::str::FromStr;

use actix_web::{
	http::header, 
	web::{self, Data, Json, Query}, 
	get,
	post,
	HttpResponse
};

use serde_json::json;
use solana_program::pubkey::Pubkey;
use tokio::task::spawn_blocking;

use crate::{
	application::dtos::{
		AirdropSolDto, PublicKeyQuery, SendTxDto
	}, 
	infrastructure::solana_rpc_client::SolanaClient
};

#[get("/ping")]
pub async fn ping() -> HttpResponse {
	HttpResponse::Ok()
		.append_header(header::ContentType::html())
		.body("<h1>Pong! Server is running 🚀 </h1>")
}

#[get("/recent-hash")]
pub async fn recent_hash(
	rpc: Data<SolanaClient>
) -> HttpResponse {
	let result = spawn_blocking(move || rpc.get_latest_hash()).await;

	match result {
		Ok(recent_hash) => {
			HttpResponse::Ok()
			.append_header(header::ContentType::json())
			.json(json!({
				"success": true,
				"message": "Got recent hash",
				"data": {
					"latest_hash": recent_hash.to_string()
				}
			}))	
		},
		Err(err) => {
			HttpResponse::NotFound()
				.append_header(header::ContentType::json())
				.json(json!({
					"success": false,
					"message": "Could not retrieve latest hash",
					"error": err.to_string()
				}))
		}
	}

}

#[get("/balance")]
pub async fn get_balance(
	rpc: Data<SolanaClient>,
	query: Query<PublicKeyQuery>
) -> HttpResponse {
	let pubkey = Pubkey::from_str(query.pubkey.as_str()).unwrap();
	let result = spawn_blocking(move || rpc.get_balance(&pubkey)).await;

	match result {
		Ok(balance) => {
			HttpResponse::Ok()
				.append_header(header::ContentType::json())
				.json(json!({
					"success": true,
					"message": "Successfully retrieved balance",
					"data": balance
				}))
		},
		Err(err) => {
			HttpResponse::NotFound()
				.append_header(header::ContentType::json())
				.json(json!({
					"success": false,
					"message": "Could not retrieve balance",
					"error": err.to_string()
				}))
		}
	}

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

#[post("/airdrop")]
pub async fn airdrop(
	rpc: Data<SolanaClient>,
	airdrop_dto: Json<AirdropSolDto>
) -> HttpResponse {
	let AirdropSolDto { recipient, sols } = airdrop_dto.into_inner();
	let pubkey = Pubkey::from_str(&recipient).unwrap();
	let result = spawn_blocking(move || rpc.airdrop(&pubkey, sols)).await;

	match result {
		Ok(signature) => {
			HttpResponse::Ok()
				.append_header(header::ContentType::json())
				.json(json!({
					"success": true,
					"message": format!("Airdropped {} SOL", sols),
					"data": {
						"signature": signature
					}
				}))
		},
		Err(err) => {
			HttpResponse::NotFound()
				.append_header(header::ContentType::json())
				.json(json!({
					"success": false,
					"message": "Could not retrieve balance",
					"error": err.to_string()
				}))
		}
	}
}

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/api")
			.service(ping)
			.service(recent_hash)
			.service(send_tx)
			.service(get_balance)
			.service(airdrop)
	);
}

