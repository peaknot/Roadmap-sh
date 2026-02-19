use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, prelude::FromRow};

#[derive(Clone)]
pub struct AppState{
    pub pool: SqlitePool,
    pub jwt_secret: String,
}
#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: i64,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}
