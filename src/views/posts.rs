use super::super::schema::posts::dsl::*;
use diesel::prelude::*;

use actix_web::http::header::AUTHORIZATION;
use actix_web::{get, web, HttpResponse, Responder};

use jsonwebtoken::{decode, Algorithm, Validation};

use super::super::views::SecretKey;

use super::super::models::posts::*;
use super::super::models::users::Claims;
use super::super::DbPool;

#[get("/")]
pub async fn get_posts(
    pool: web::Data<DbPool>,
    key: web::Data<SecretKey>,
    req: web::HttpRequest,
) -> impl Responder {
    let conn = pool.get().expect("Could not get DB connection");

    let jwt: &str = req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();

    let secret_key = &key.dec_key;

    return match decode::<Claims>(jwt, secret_key, &Validation::new(Algorithm::HS256)) {
        Ok(x) => {
            let user_id = x.claims.id;
            let results =
                web::block(move || posts.filter(author_id.eq(user_id)).load::<Post>(&conn)).await;
            match results {
                Ok(x) => HttpResponse::Ok().json(x),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    };
}
