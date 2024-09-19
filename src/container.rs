#[allow(unused)]

use lazy_static::lazy_static;
use diesel::{
    r2d2::{Pool, ConnectionManager},
    prelude::*,
};
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;
use crate::models::User;
use crate::repositories::{CertificationRepository, UserRepository};

type PgPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug,Clone)]
pub struct Auth {
    pub is_logged_in: bool,
    pub user: Option<User>
}

impl Auth {
    pub fn update(&mut self, is_logged_in: bool, user: Option<User>) {
        self.is_logged_in = is_logged_in;
        self.user = user;
    }
}

lazy_static! {
    pub static ref POOL: PgPool = {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        PgPool::builder()
            .max_size(8)
            .build(ConnectionManager::new(db_url))
            .expect("Failed to create database connection pool")
    };

    pub static ref AUTH: Mutex<Auth> = Mutex::new(Auth{
        is_logged_in: false,
        user: None,
    });

    pub static ref UserRepo: UserRepository = {
        UserRepository{}
    };

    pub static ref CertificationRepo: CertificationRepository = {
        CertificationRepository{}
    };
}
