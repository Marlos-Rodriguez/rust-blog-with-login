use super::super::schema::users::dsl::*;
use diesel::prelude::*;

use actix_web::{get, post, web, HttpResponse, Responder};

use super::super::models::users::*;
use super::super::DbPool;

#[get("/")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Could not get DB connection");

    let results = web::block(move || users.load::<User>(&conn)).await;

    match results {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/register")]
pub async fn login(pool: web::Data<DbPool>, body: web::Json<NewUserHandler>) -> impl Responder {
    let conn = pool.get().expect("Could no get DB connection");

    let mut request = body.clone();

    let results = web::block(move || User::create_user(&conn, &mut request)).await;

    match results {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
