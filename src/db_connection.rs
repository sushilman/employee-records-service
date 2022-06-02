extern crate dotenv;


use diesel::r2d2::{ Pool, PoolError, PooledConnection, ConnectionManager };
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn pg_pool_handler(pool: &PgPool) -> Result<PgPooledConnection, PoolError> {
    Ok(pool.get().unwrap())
}
