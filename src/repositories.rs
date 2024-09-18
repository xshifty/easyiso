use std::fmt::format;
use diesel::prelude::*;
use crate::container::*;
use crate::models::User;
use crate::schema;

#[derive(Debug,Clone,Copy)]
pub struct UserRepository {
}

impl UserRepository {
    pub fn get_user_by_id(self, id_param: i32) -> User {
        use schema::users;

        let mut binding = POOL.get();
        let conn = binding.as_mut().unwrap();

        users::table
            .filter(users::id.eq(id_param.clone()))
            .first::<User>(conn)
            .expect(format!("User #{} not found", id_param.clone()).as_str())
    }

    pub fn get_user_by_login(self, email_param: String, passwd_param: String) -> User {
        use schema::users;

        let mut binding = POOL.get();
        let conn = binding.as_mut().unwrap();

        users::table
            .filter(users::email.eq(email_param.clone()))
            .filter(users::password.eq(passwd_param.clone()))
            .first::<User>(conn)
            .expect(format!("User {} not found", email_param.clone()).as_str())
    }
}
