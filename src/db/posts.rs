use serde::{Deserialize, Serialize};
use actix_web::web;
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}


pub async fn get_posts(db: web::Data<sqlx::PgPool>) -> Result<Vec<Post>, actix_web::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM ronskyblog.posts")
        .fetch_all(db.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })
}

pub async fn get_posts_by_id(db: web::Data<sqlx::PgPool>, id: i32) -> Result<Vec<Post>, actix_web::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM ronskyblog.posts where posts.id = $1")
        .bind(id)
        .fetch_all(db.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })
}

pub async fn get_sorted_posts(
    db: web::Data<sqlx::PgPool>, 
    sort_by: &str, 
    direction: &str, 
    limit: i32
) -> Result<Vec<Post>, actix_web::Error> {
    let query = match sort_by {
        "title" => "SELECT * FROM ronskyblog.posts ORDER BY title",
        "id" => "SELECT * FROM ronskyblog.posts ORDER BY id",
        _ => "SELECT * FROM ronskyblog.posts ORDER BY created_at"
    };

    let query = match direction.to_lowercase().as_str() {
        "asc" => format!("{} ASC LIMIT $1", query),
        _ => format!("{} DESC LIMIT $1", query),
    };

    sqlx::query_as::<_, Post>(&query)
        .bind(limit)
        .fetch_all(db.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })
}
