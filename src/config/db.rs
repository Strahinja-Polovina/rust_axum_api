use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use lazy_static::lazy_static;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: PgPool = create_connection_pool().expect("Failed to create DB Pool");
}

fn create_connection_pool() -> Result<PgPool, PoolError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL doesnt exist in .env");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_connection() -> Result<PooledConnection<ConnectionManager<PgConnection>>, PoolError> {
    POOL.get()
}

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    get_connection().expect("Failed to get a database connection from the pool")
}
