use std::str::FromStr;

use crate::{
    auth::auth, errors::{ApiError, AppError}, handlers::{list_expense, new_expense, update_expense, delete_expense}, schema::{AppState, Claims, CreateUser, LoginPayload, User}
};
use anyhow::Result;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, Router, extract::State, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get, patch, post}};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::json;
use sqlx::{SqlitePool, sqlite::{SqliteConnectOptions, SqlitePoolOptions}};
use tokio::net::TcpListener;

mod auth;
mod handlers;
mod errors;
mod schema;
#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();

    let db_url: String = std::env::var("DATABASE_URL")?;

    let connect_opts = SqliteConnectOptions::from_str(&db_url)?
        .foreign_keys(true);
    
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await?;

    let jwt_secret = std::env::var("SECRET_KEY")?;
    let state = AppState {pool, jwt_secret};

    sqlx::migrate!("./migrations").run(&state.pool).await?;

    let app = build_app(state);

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("Listening to port 3000");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, ApiError> {
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash: String = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|err| {
            eprintln!("Password hashing fialed: {err}");
            ApiError::Internal
        })?
        .to_string();

    let user_name: &str = payload.username.trim();

    if user_name.is_empty() {
        eprintln!("Username is required");
        return Err(ApiError::BadRequest("Username is required".into()));
    }
    //TODO validate email address first

    let time_created: i64 = Utc::now().timestamp();

    let result = sqlx::query(
        r#"
        INSERT INTO users (username, email, password_hash, created_at)
        VALUES (?1, ?2, ?3, ?4)
    "#,
    )
    .bind(user_name)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(time_created)
    .execute(&state.pool)
    .await;

    match result {
        Ok(id) => {
            let user_id: i64 = id.last_insert_rowid();
            Ok((
                StatusCode::CREATED,
                Json(json!({"msg": "Account created successfully", "id": user_id})),
            ))
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("UNIQUE") {
                return Err(ApiError::Conflict("Username already exists"));
            } else {
                eprintln!("DB insert failed: {e}");
                Err(ApiError::Internal)
            }
        }
    }
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, ApiError> {

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE username = ?1
    "#,
    )
    .bind(&payload.username)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::Unauthorized)?;

    let parsed = match PasswordHash::new(&user.password_hash) {
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
        sub: user.id.to_string(),
        exp: expiration.timestamp(),
        iat: issued_at.timestamp(),
    };

    let key = &state.jwt_secret;
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )?;


    Ok((StatusCode::OK, Json(json!({ 
        "msg": "Login successful",
        "token": token,
        "type": "Bearer"
    }))))
}

fn build_app(state: AppState) -> Router {
    let public = Router::new()
        .route("/users", post(create_user))
        .route("/login", post(login));

    let private = Router::new()
        .route("/home/expense/delete/{id}", delete(delete_expense))
        .route("/home/expense/update/{id}", patch(update_expense))
        .route("/home/expense/add", post(new_expense))
        .route("/home/expense/list", get(list_expense))
        .route_layer(middleware::from_fn(auth));

    Router::new()
        .merge(private)
        .merge(public)
        .with_state(state)
}
