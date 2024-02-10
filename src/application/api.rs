use std::fs;
use std::str::FromStr;

use actix_web::{
	http::header, 
	web::{self, Data, Json, Query}, 
	get,
	post,
	HttpResponse
};

use solana_program::pubkey::Pubkey;
use tokio::task::spawn_blocking;

use crate::{
	application::dtos::{
		AirdropSolDto, PublicKeyQuery, SendTxDto
	}, 
	infrastructure::solana_rpc_client::SolanaClient, 
	presentation::{json_presenter::JsonPresenter, presenter::Presenter}
};

#[get("/ping")]
pub async fn ping() -> HttpResponse {
	let ping_html_path: &str = "./static/ping.html";
	let html = fs::read_to_string(ping_html_path).unwrap();

	HttpResponse::Ok()
		.append_header(header::ContentType::html())
		.body(html)
}

#[get("/recent-hash")]
pub async fn recent_hash(
	rpc: Data<SolanaClient>
) -> HttpResponse {
	let result = spawn_blocking(move || rpc.get_latest_hash_base64()).await;

	JsonPresenter::present(
		result, 
		"Got recent hash",
		"Could not retrieve latest hash"
	)
}

#[get("/balance")]
pub async fn get_balance(
	rpc: Data<SolanaClient>,
	query: Query<PublicKeyQuery>
) -> HttpResponse {
	let pubkey = Pubkey::from_str(query.pubkey.as_str()).unwrap();
	let result = spawn_blocking(move || rpc.get_balance(&pubkey)).await;

	JsonPresenter::present(
		result, 
		"Successfully retrieved balance",
		"Could not retrieve balance"
	)
}

#[post("/tx")]
pub async fn send_tx(
	rpc: Data<SolanaClient>,
	send_tx_dto: Json<SendTxDto>
) -> HttpResponse {
	let SendTxDto { sender_keypair_path, recipient, sols } = send_tx_dto.0;
	let recipient_pk = Pubkey::from_str(&recipient).unwrap();
	let rpc_clone = rpc.clone();
	
	let keypair = spawn_blocking(move || {
		rpc_clone.read_keypair(sender_keypair_path.as_str())
	}).await.unwrap();

	let result = spawn_blocking(move || {
		rpc.send_tx(&keypair, &recipient_pk, sols)
	}).await;

	JsonPresenter::present(
		result,
		"Successfully performed transaction",
		"Failed to perform transaction"
	)
}

#[post("/airdrop")]
pub async fn airdrop(
	rpc: Data<SolanaClient>,
	airdrop_dto: Json<AirdropSolDto>
) -> HttpResponse {
	let AirdropSolDto { recipient, sols } = airdrop_dto.into_inner();
	let pubkey = Pubkey::from_str(&recipient).unwrap();
	let result = spawn_blocking(move || rpc.airdrop(&pubkey, sols)).await;

	JsonPresenter::present(
		result,
		format!("Airdropped {} SOL", sols).as_str(),
		"Could not retrieve balance"
	)
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

