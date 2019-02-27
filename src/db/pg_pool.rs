extern crate r2d2;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

use std::env;

type ManagedPgConn = ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<ManagedPgConn>;

fn init(database_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("Failed to create pool.")
}

// Establishes a connection to the libellis postgres database on your machine, as specified by your
// DATABASE_URL environment variable. Returns a Pool
pub fn generate_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    init(&database_url)
}
