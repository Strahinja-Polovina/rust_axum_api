use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use lazy_static::lazy_static;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: PgPool = create_connection_pool_test().expect("Failed to create DB Pool");
}

fn create_connection_pool_test() -> Result<PgPool, PoolError> {
    let database_url =
        env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST doesnt exist in .env");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_connection_test() -> Result<PooledConnection<ConnectionManager<PgConnection>>, PoolError>
{
    POOL.get()
}

pub fn establish_connection_test() -> PooledConnection<ConnectionManager<PgConnection>> {
    get_connection_test().expect("Failed to get a database connection from the pool")
}
