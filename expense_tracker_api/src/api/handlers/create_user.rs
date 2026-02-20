use crate::api::dto::{ApiError, CreateUserRequestDTO, UserResponseDTO};
use crate::api::state::AppState;
use crate::domain::user::NewUser;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequestDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let new_user: NewUser = payload.try_into()?;

    let NewUser {
        username,
        email,
        password_hash,
    } = new_user;

    let user: UserResponseDTO = sqlx::query_as(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES (?1, ?2, ?3)
        RETURNING id, username, email, created_at
    "#,
    )
    .bind(username.into_inner())
    .bind(email.into_inner())
    .bind(password_hash.into_inner())
    .fetch_one(&state.pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "msg": "Account created successfully",
            "user": user,
        })),
    ))
}
