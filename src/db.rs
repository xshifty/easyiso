use std::env;
use diesel::{
    r2d2::{Pool, ConnectionManager},
    pg::PgConnection
};
use lazy_static::lazy_static;

type PgPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref POOL: PgPool = {
        PgPool::builder()
            .max_size(8)
            .build(ConnectionManager::new(env::var("DATABASE_URL").expect("DATABASE_URL not set")))
            .expect("Failed to setup database connection pool")
    };
}
