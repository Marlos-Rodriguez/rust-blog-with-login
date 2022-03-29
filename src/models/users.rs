use super::super::schema::users;

use chrono::NaiveDateTime;
use pwhash::bcrypt;
use uuid::Uuid;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Queryable)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub is_admin: &'a bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewUserHandler {
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

impl User {
    pub fn validate_password(&self, password: &String) -> bool {
        return bcrypt::verify(password, self.password.as_str());
    }

    pub fn hash_password(password: &String) -> String {
        return bcrypt::hash(password).unwrap();
    }

    pub fn create_user<'a>(
        conn: &PgConnection,
        user: &mut NewUserHandler,
    ) -> Result<User, diesel::result::Error> {
        let password_hash = User::hash_password(&user.password);
        let id: String = Uuid::new_v4().to_string();

        let new_user = NewUser {
            id: &id,
            username: &user.username,
            email: &user.email,
            password: &password_hash,
            is_admin: &user.is_admin,
        };

        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(conn)
    }
}
