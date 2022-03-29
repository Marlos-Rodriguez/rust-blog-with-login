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

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use views::users::{get_users, login};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //ENV
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database URL is required");

    let connection = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(connection)
        .expect("Error in create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::scope("/users").service(get_users).service(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
