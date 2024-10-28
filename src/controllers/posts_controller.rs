use actix_web::{get, web, HttpResponse, Error};
use serde::Deserialize;
use crate::db::*;

#[derive(Deserialize)]
pub struct SortParams {
    #[serde(rename = "sortBy", default = "default_sort_by")]
    pub sort_by: String,
    #[serde(default = "default_direction")]
    pub direction: String,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

fn default_sort_by() -> String { "createdAt".into() }
fn default_direction() -> String { "desc".into() }
fn default_limit() -> i32 { 5 }


#[get("/posts")]
pub async fn handler_get_posts(db: web::Data<sqlx::PgPool>) -> Result<HttpResponse, Error> {
    let posts = get_posts(db).await?;
    Ok(HttpResponse::Ok().json(posts))
}

#[get("/posts/{id}")]
pub async fn handler_get_posts_by_id(db: web::Data<sqlx::PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let posts = get_posts_by_id(db, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(posts))
}

#[get("/posts/sorted")]
pub async fn handler_get_sorted_posts(query_string: web::Query<SortParams>, db: web::Data<sqlx::PgPool>) -> Result<HttpResponse, Error> {
    let posts = get_sorted_posts(db, &query_string.sort_by, &query_string.direction, query_string.limit).await?;
    Ok(HttpResponse::Ok().json(posts))
}
