use crate::errors::AppError;
use anyhow::Result;
use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::str::FromStr;
use tokio::net::TcpListener;

use crate::api::{
    AppState, auth,
    handlers::{create_user, delete_expense, list_expense, login, new_expense, update_expense},
};

mod api;
mod domain;
mod errors;
#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();

    let db_url: String = std::env::var("DATABASE_URL")?;

    let connect_opts = SqliteConnectOptions::from_str(&db_url)?.foreign_keys(true);

    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await?;

    let jwt_secret = std::env::var("SECRET_KEY")?;
    let state = AppState { pool, jwt_secret };

    sqlx::migrate!("./migrations").run(&state.pool).await?;

    let app = build_app(state);

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("Listening to port 3000");
    axum::serve(listener, app).await?;

    Ok(())
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

    Router::new().merge(private).merge(public).with_state(state)
}
