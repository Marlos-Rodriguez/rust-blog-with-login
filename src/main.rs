#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod views;

//ENV
use dotenv::dotenv;
use std::env;

//Diesel
use diesel::pg::PgConnection;
use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

//JWT
use jsonwebtoken::{DecodingKey, EncodingKey};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

//Views
use views::posts::{create_post, get_posts};
use views::users::{get_users, login, register};
use views::SecretKey;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //ENV
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database URL is required");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT Secret is required");
    let host = env::var("HOST").expect("HOST must be setted");
    let port = env::var("PORT").expect("PORT must be setted");

    let keys = SecretKey {
        enc_key: EncodingKey::from_secret(jwt_secret.as_ref()),
        dec_key: DecodingKey::from_secret(jwt_secret.as_ref()),
    };

    let connection = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(connection)
        .expect("Error in create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(keys.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/users")
                    .service(get_users)
                    .service(register)
                    .service(login),
            )
            .service(web::scope("/posts").service(get_posts).service(create_post))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
