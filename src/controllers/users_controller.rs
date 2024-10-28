use actix_web::{get, web, HttpResponse, Error};
use crate::db::*;
use crate::SortParams;

pub async fn handler_get_posts_by_user_id(db: web::Data<sqlx::PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let posts = get_posts_by_user_id(db, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(posts))
}

pub async fn handler_get_sorted_posts_by_user_id(query_string: web::Query<SortParams>, db: web::Data<sqlx::PgPool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let posts = get_sorted_posts_by_user_id(db, user_id.into_inner(), &query_string.sort_by, &query_string.direction, query_string.limit).await?;
    Ok(HttpResponse::Ok().json(posts))
}
