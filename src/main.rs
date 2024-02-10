use actix_web::{web::Data, App, HttpServer};

pub mod application;
use application::api::config;

pub mod infrastructure;
use infrastructure::{
    solana_rpc_client::SolanaClient,
    // database::Database
};

pub mod presentation;
use presentation::json_presenter::JsonPresenter;

pub mod common;

const ENV: &str = "./env/.env.dev"; 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    initialize();

    let factory = move || App::new()
        // .app_data(inject!(Database::new(env_var!("DATABASE_URL"))))
        .app_data(inject!(SolanaClient::connect(env_var!("JSON_RPC_URL"))))
        .app_data(inject!(JsonPresenter))
        .configure(config);

    HttpServer::new(factory)
        .bind(("0.0.0.0", 3000))
        .unwrap()
        .run()
        .await
}

fn initialize() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenvy::from_path(ENV).ok();
}