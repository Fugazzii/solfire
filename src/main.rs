use env_logger;
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
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenvy::from_path(ENV).ok();

    println!("{}", env_var!("JSON_RPC_UR"));

    // let db_url = dotenvy::var("DATABASE_URL").expect("Provide DATABASE_URL");
    let factory = move || App::new()
        // .app_data(
        //     Data::new(
        //         Database::new(&db_url).pool
        //     )
        // )
        .app_data(
            Data::new(
                SolanaClient::connect(dotenvy::var("JSON_RPC_URL").unwrap().as_str())
            )
        )
        .app_data(Data::new(JsonPresenter))
        .configure(config);

    HttpServer::new(factory)
        .bind(("localhost", 3000))
        .unwrap()
        .run()
        .await
}
