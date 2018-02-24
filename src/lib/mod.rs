extern crate r2d2;
#[cfg(not(release))]
extern crate dotenv;
#[cfg(not(release))]
use std::env;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

pub mod schema;
pub mod models;
pub mod db_conn;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// #[cfg(buid="production")]
#[cfg(release)]
static DATABASE_URL: &'static str = env!("DATABASE_URL");

pub fn init_pool() -> Pool {
    #[cfg(not(release))]
    dotenv::dotenv().expect("Failed to read .env file");
    #[cfg(not(release))]
    let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    r2d2::Pool::new(manager).expect("db pool")
}
