use super::super::schema::users;

use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
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

#[derive(Clone, Serialize, Deserialize)]
pub struct NewLoginHandler {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub jwt: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    id: String,
    iat: i64,
    exp: i64,
    email: String,
}

impl User {
    pub fn validate_password(&self, password: &String) -> bool {
        return bcrypt::verify(password, self.password.as_str());
    }

    pub fn hash_password(password: &String) -> String {
        return bcrypt::hash(password).unwrap();
    }

    pub fn create_token(&self, secret: &EncodingKey) -> String {
        let iat_token = Utc::now().timestamp();
        let exp_token = Utc::now()
            .checked_add_signed(Duration::minutes(60))
            .expect("Invalid timestamp")
            .timestamp();

        let new_claims = Claims {
            id: self.id.to_string(),
            iat: iat_token,
            exp: exp_token,
            email: self.email.to_string(),
        };
        let token = match encode(&Header::default(), &new_claims, &secret) {
            Ok(t) => t,
            Err(e) => panic!("Error encoding the token: {}", e),
        };

        return token;
    }

    pub fn create_user<'a>(
        conn: &PgConnection,
        user: NewUserHandler,
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
