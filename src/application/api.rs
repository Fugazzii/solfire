// use solana_sdk::signer::Signer;
use actix_web::{get, web, App, HttpResponse, HttpServer};

use crate::infrastructure;
use infrastructure::{
    database::Database,
    solana_rpc_client::SolanaClient
};

#[get("/ping")]
pub async fn ping() -> HttpResponse {
	// let db = Database::new();
	// let rpc_url = dotenv::var("JSON_RPC_URL");
	// let client = SolanaClient::connect(rpc_url.unwrap().as_str());

	HttpResponse::Ok().body("pong!")
}
