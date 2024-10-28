use actix_web::{web, App, HttpServer};
use actix_web::Error;
use actix_web::{HttpResponse, HttpRequest};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

mod db;
mod controllers;
mod auth;
use auth::*;
use controllers::*;
use db::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create db pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handler_get_posts)
            .service(handler_get_sorted_posts)
            .service(handler_get_posts_by_id)
            .route("/login", web::post().to(login))
            .route("/users/{user_id}/posts", web::get().to(protected_handler_get_posts_by_user_id))
            .route("/users/{user_id}/posts/sorted", web::get().to(protected_handler_get_sorted_posts_by_user_id))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
/*
To create these kinds of protected routes
pub async fn protected_handler_get_posts(
    req: HttpRequest,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    require_auth(&req)?;
    handler_get_posts(db).await
}

pub async fn protected_handler_get_sorted_posts(
    req: HttpRequest,
    query: web::Query<SortParams>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    require_auth(&req)?;
    handler_get_sorted_posts(query, db).await
}

pub async fn protected_handler_get_posts_by_id(
    req: HttpRequest,
    db: web::Data<PgPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    require_auth(&req)?;
    handler_get_posts_by_id(db, id).await
}
*/

pub async fn protected_handler_get_posts_by_user_id(
    req: HttpRequest,
    db: web::Data<PgPool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    require_auth(&req)?;
    handler_get_posts_by_user_id(db, user_id).await
}

pub async fn protected_handler_get_sorted_posts_by_user_id(
    req: HttpRequest,
    query: web::Query<SortParams>,
    db: web::Data<PgPool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    require_auth(&req)?;
    handler_get_sorted_posts_by_user_id(query, db, user_id).await
}
