use super::super::schema::users::dsl::*;
use diesel::prelude::*;

use actix_web::{get, post, web, HttpResponse, Responder};

use super::super::models::users::*;
use super::super::views::{LoginResponse, SecretKey};
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
pub async fn register(pool: web::Data<DbPool>, body: web::Json<NewUserHandler>) -> impl Responder {
    let conn = pool.get().expect("Could no get DB connection");

    let request = body.clone();

    let results = web::block(move || User::create_user(&conn, request)).await;

    match results {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    key: web::Data<SecretKey>,
    body: web::Json<NewLoginHandler>,
) -> impl Responder {
    let conn = pool.get().expect("Could no get DB connection");

    let username_item = body.username.clone();
    let password_item = body.password.clone();
    let secret_key = &key.enc_key;

    let results = web::block(move || {
        users
            .filter(username.eq(username_item))
            .limit(1)
            .load::<User>(&conn)
    })
    .await;

    let response = match results {
        Ok(x) => {
            if x.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let user = &x[0];

            match user.validate_password(&password_item) {
                true => {
                    let jwt = user.create_token(&secret_key);
                    let jwt_response = LoginResponse { jwt: jwt };
                    HttpResponse::Ok().json(jwt_response)
                }
                false => {
                    println!("Bad password");
                    HttpResponse::BadRequest().finish()
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    };
    return response;
}
