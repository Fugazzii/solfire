mod database;
use database::{Database, Event};

fn main() {

    let db: Database<Event> = Database::new();


    println!("Hello, world!");
}
