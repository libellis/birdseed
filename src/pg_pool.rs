extern crate r2d2;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

type ManagedPgConn = ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<ManagedPgConn>;

use std::ops::Deref;

/// Db Connection request guard type: wrapper around r2d2 pooled connection
pub struct DbConn(pub r2d2::PooledConnection<ManagedPgConn>);

// For the convenience of using an &DbConn as an &PgConnection.
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn init(database_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("Failed to create pool.")
}
