use chrono::prelude::*;
use diesel::{Queryable};

#[derive(Debug,Clone,Queryable)]
pub struct User {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
}
