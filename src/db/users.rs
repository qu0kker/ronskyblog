use serde::{Deserialize, Serialize};
use actix_web::web;
use chrono::{NaiveDateTime, NaiveDate};
use sqlx::FromRow;

use crate::posts::Post;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: Option<bool>,
    pub email_verified: Option<bool>,
    pub account_type: Option<String>,
    pub last_login_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub async fn get_posts_by_user_id(db: web::Data<sqlx::PgPool>, id: i32) -> Result<Vec<Post>, actix_web::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM ronskyblog.posts where posts.user_id = $1")
        .bind(id)
        .fetch_all(db.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })
}

pub async fn get_sorted_posts_by_user_id(
    db: web::Data<sqlx::PgPool>, 
    user_id: i32,
    sort_by: &str, 
    direction: &str, 
    limit: i32
) -> Result<Vec<Post>, actix_web::Error> {
    let query = match sort_by {
        "title" => "SELECT * FROM ronskyblog.posts WHERE posts.user_id = $1 ORDER BY title",
        "id" => "SELECT * FROM ronskyblog.posts WHERE posts.user_id = $1 ORDER BY id",
        _ => "SELECT * FROM ronskyblog.posts WHERE posts.user_id = $1 ORDER BY created_at"
    };

    let query = match direction.to_lowercase().as_str() {
        "asc" => format!("{} ASC LIMIT $2", query),
        _ => format!("{} DESC LIMIT $2", query),
    };

    sqlx::query_as::<_, Post>(&query)
        .bind(user_id)
        .bind(limit)
        .fetch_all(db.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })
}
