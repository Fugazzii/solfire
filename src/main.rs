use env_logger;
use actix_web::{web::Data, App, HttpServer};

pub mod application;
use application::api::config;

pub mod infrastructure;
use infrastructure::{
    solana_rpc_client::SolanaClient,
    // database::Database
};

const ENV: &str = "./env/.env.dev"; 


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    dotenvy::from_path(ENV).ok();

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
        .configure(config);

    HttpServer::new(factory)
        .bind(("localhost", 3000))
        .unwrap()
        .run()
        .await
}
