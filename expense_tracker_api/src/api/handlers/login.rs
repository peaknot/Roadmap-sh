use crate::api::{
    dto::{ApiError, Claims, LoginRequestDTO},
    state::AppState,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::json;
use sqlx::prelude::FromRow;

#[derive(FromRow)]
struct AuthUser {
    id: i64,
    password_hash: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequestDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let row = sqlx::query_as::<_, AuthUser>(
        r#"
        SELECT * FROM users WHERE username = ?1
    "#,
    )
    .bind(&payload.username)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::Unauthorized)?;

    let parsed = match PasswordHash::new(&row.password_hash) {
        Ok(hash) => hash,
        Err(_) => {
            return Err(ApiError::Internal);
        }
    };
    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed)
        .is_err()
    {
        return Err(ApiError::Unauthorized);
    }
    let issued_at = Utc::now();
    let expiration = issued_at + Duration::hours(1);

    let claims: Claims = Claims {
        sub: row.id.to_string(),
        exp: expiration.timestamp(),
        iat: issued_at.timestamp(),
    };

    let key = &state.jwt_secret;
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "msg": "Login successful",
            "token": token,
            "type": "Bearer"
        })),
    ))
}
