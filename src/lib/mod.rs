extern crate r2d2;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

pub mod schema;
pub mod models;
pub mod db_conn;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

static DATABASE_URL: &'static str = env!("DATABASE_URL");

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    r2d2::Pool::new(manager).expect("db pool")
}
