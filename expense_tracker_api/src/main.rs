use crate::schema::CreateUser;
use anyhow::Result;
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use chrono::Utc;
use serde_json::{Value, json};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

mod schema;
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    let db_url: String = std::env::var("DATABASE_URL")?;
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app: Router = Router::new()
        .route("/users", post(create_user))
        .with_state(pool);

    Ok(())
}

async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash: String = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|err| {
            eprintln!("Password hashing fialed: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Password hashing failed"})),
            )
        })?
        .to_string();

    let user_name: &str = payload.username.trim();

    if user_name.is_empty() {
        eprintln!("Username is required");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": " Username is required"})),
        ));
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
                return Err((
                    StatusCode::CONFLICT,
                    Json(json!({
                        "msg": "Username already exists"
                    })),
                ));
            } else {
                eprintln!("DB insert failed: {e}");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                    "error": "Internal server error"
                    })),
                ))
            }
        }
    }
}
