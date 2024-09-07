use diesel::prelude::*;
use diesel::sql_types::{Nullable, Bool, Integer, Timestamp, VarChar};
use serde::{Deserialize, Serialize};
use crate::schema::users::{all_columns, created_at, email, enabled, full_name, id, password};
use crate::schema::users::dsl::users;

#[derive(Serialize,Queryable,Deserialize,Insertable)]
#[diesel(table_name = "users")]
pub struct User {
    pub id: Integer,
    pub full_name: VarChar,
    pub email: VarChar,
    pub password: VarChar,
    pub enabled: Bool,
    pub created_at: Nullable<Timestamp>,
}

impl User {
    #[inline]
    pub fn get_user_by_id(conn: &mut PgConnection, param_id: i32) -> &User {
        let user = users.filter(id.eq(param_id))
            .load::<User>(conn);

        user.unwrap().first().unwrap()
    }
}
