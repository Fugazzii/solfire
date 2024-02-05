use actix_web::{App, HttpServer};
use application::api::ping;
use dotenv;

pub mod infrastructure;

mod application;

const ENV: &str = "./env/.env.dev"; 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path(ENV).ok();
    
    HttpServer::new(|| App::new().service(ping))
        .bind(("localhost", 3000))
        .unwrap()
        .run()
        .await
        .unwrap();

    Ok(())
}
