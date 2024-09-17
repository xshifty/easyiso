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

        users::table
            .filter(users::id.eq(id_param))
            .first::<User>(POOL.clone().get().as_mut().unwrap())
            .expect("could not fetch user")
            .clone()
    }
}
