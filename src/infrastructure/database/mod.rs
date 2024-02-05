use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
	pub pool: DBPool
}

impl Database {
	pub fn new(database_url: &String) -> Self {
		let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
	}
}