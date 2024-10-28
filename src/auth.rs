use actix_web::{error::ErrorUnauthorized, web, Error, HttpRequest, HttpResponse};
use sqlx::Row;
use bcrypt::verify;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};

const JWT_SECRET: &[u8] = b"spaghettios";

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,
}

pub async fn login(
    db: web::Data<PgPool>,
    creds: web::Json<LoginCredentials>,
) -> Result<HttpResponse, Error> {
    let user = sqlx::query(
        "SELECT id, password_hash FROM ronskyblog.users WHERE username = $1",
    )
    .bind(&creds.username)
    .fetch_optional(db.get_ref())
    .await
    .map_err(|e| ErrorUnauthorized(e.to_string()))?
    .ok_or_else(|| ErrorUnauthorized("Invalid credentials"))?;

    let password_hash: String = user.get("password_hash");
    let id: i32 = user.get("id");

    if !verify(&creds.password, &password_hash)
        .map_err(|e| ErrorUnauthorized(e.to_string()))? 
    {
        return Err(ErrorUnauthorized("Invalid credentials"));
    }

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + 24 * 3600; // 24 hours

    let claims = Claims {
        user_id: id,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .map_err(|e| ErrorUnauthorized(e.to_string()))?;

    Ok(HttpResponse::Ok().json(token))
}

// Simple function to check if a request is authenticated
pub fn require_auth(req: &HttpRequest) -> Result<i32, Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ErrorUnauthorized("Missing authorization header"))?
        .to_str()
        .map_err(|_| ErrorUnauthorized("Invalid authorization header"))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ErrorUnauthorized("Invalid token format"))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map_err(|_| ErrorUnauthorized("Invalid token"))?;

    Ok(token_data.claims.user_id)
}
