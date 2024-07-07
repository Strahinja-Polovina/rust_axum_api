use crate::messages::config_constants::{DATABASE_URL, DATABASE_URL_ERROR, POOL_CONNECTION_ERROR};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use lazy_static::lazy_static;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: PgPool = create_connection_pool().expect(POOL_CONNECTION_ERROR);
}

fn create_connection_pool() -> Result<PgPool, PoolError> {
    let database_url = env::var(DATABASE_URL).expect(DATABASE_URL_ERROR);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_connection() -> Result<PooledConnection<ConnectionManager<PgConnection>>, PoolError> {
    POOL.get()
}
