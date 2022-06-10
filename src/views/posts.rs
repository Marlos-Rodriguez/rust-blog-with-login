use super::super::schema::posts::dsl::*;
use diesel::prelude::*;

use actix_web::http::header::AUTHORIZATION;
use actix_web::{get, post, web, HttpResponse, Responder};

use jsonwebtoken::{decode, Algorithm, Validation};

use super::super::views::{ErrorResponse, SecretKey};

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

    return match req.headers().get(AUTHORIZATION) {
        None => {
            let new_error = ErrorResponse {
                error: "Error in get header".to_string(),
                description: "JWT needed for authorization".to_string(),
            };
            HttpResponse::InternalServerError().json(new_error)
        }
        Some(header) => {
            let jwt: &str = header.to_str().unwrap();

            let secret_key = &key.dec_key;

            return match decode::<Claims>(jwt, secret_key, &Validation::new(Algorithm::HS256)) {
                Ok(x) => {
                    let user_id = x.claims.id;
                    let results =
                        web::block(move || posts.filter(author_id.eq(user_id)).load::<Post>(&conn))
                            .await;
                    match results {
                        Ok(x) => HttpResponse::Ok().json(x),
                        Err(e) => {
                            let new_error = ErrorResponse {
                                error: "Error server".to_string(),
                                description: e.to_string(),
                            };
                            HttpResponse::InternalServerError().json(new_error)
                        }
                    }
                }
                Err(e) => {
                    let new_error = ErrorResponse {
                        error: "JWT fail to verify".to_string(),
                        description: e.to_string(),
                    };
                    HttpResponse::BadRequest().json(new_error)
                }
            };
        }
    };
}

#[post("/")]
pub async fn create_post(
    pool: web::Data<DbPool>,
    key: web::Data<SecretKey>,
    post_body: web::Json<NewPostHandler>,
    req: web::HttpRequest,
) -> impl Responder {
    let conn = pool.get().expect("Could not get DB connection");

    return match req.headers().get(AUTHORIZATION) {
        None => {
            let new_error = ErrorResponse {
                error: "Error in get header".to_string(),
                description: "JWT needed for authorization".to_string(),
            };
            HttpResponse::InternalServerError().json(new_error)
        }
        Some(header) => {
            println!("{:?}", header);
            let jwt: &str = header.to_str().unwrap();

            let secret_key = &key.dec_key;

            return match decode::<Claims>(jwt, secret_key, &Validation::new(Algorithm::HS256)) {
                Ok(x) => {
                    let user_id = x.claims.id;
                    let body_clone = post_body.clone();
                    let results =
                        web::block(move || Post::create_post(&conn, body_clone, &user_id)).await;
                    match results {
                        Ok(x) => HttpResponse::Ok().json(x),
                        Err(e) => {
                            let new_error = ErrorResponse {
                                error: "Error server".to_string(),
                                description: e.to_string(),
                            };
                            HttpResponse::InternalServerError().json(new_error)
                        }
                    }
                }
                Err(e) => {
                    let new_error = ErrorResponse {
                        error: "JWT fail to verify".to_string(),
                        description: e.to_string(),
                    };
                    HttpResponse::BadRequest().json(new_error)
                }
            };
        }
    };
}
