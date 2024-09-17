#[allow(unused)]

use lazy_static::lazy_static;
use diesel::{
    r2d2::{Pool, ConnectionManager},
    prelude::*,
};
use dotenv::dotenv;
use std::env;
use crate::repository::UserRepository;

type PgPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref POOL: PgPool = {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        PgPool::builder()
            .max_size(8)
            .build(ConnectionManager::new(db_url))
            .expect("Failed to create database connection pool")
    };

    pub static ref UserRepo: UserRepository = {
        UserRepository{}
    };
}
