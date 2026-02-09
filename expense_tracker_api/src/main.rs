use crate::{
    errors::{ApiError, AppError},
    schema::{Claims, CreateUser, LoginPayload, User},
};
use anyhow::Result;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::json;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tokio::net::TcpListener;

mod errors;
mod schema;
#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();

    let db_url: String = std::env::var("DATABASE_URL")?;
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app: Router = Router::new()
        .route("/users", post(create_user))
        .route("/login", post(login))
        .with_state(pool);

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("Listening to port 3000");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_user(
    State(pool): State<SqlitePool>,
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
    .execute(&pool)
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
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE username = ?
    "#,
    )
    .bind(&payload.username)
    .fetch_optional(&pool)
    .await?
    .ok_or(ApiError::NotFound("Username does not exist"))?;

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
        subject: user.id,
        expiry: expiration.timestamp() as usize,
        issued_at: issued_at.timestamp() as usize,
    };

    let key = std::env::var("SECRET_KEY")?;
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
