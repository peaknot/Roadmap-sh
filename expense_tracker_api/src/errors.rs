use argon2::{password_hash};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(&'static str),
    #[error("resource not found")]
    _NotFound,
    #[error("{0}")]
    Conflict(&'static str),
    #[error("internal server error")]
    Internal,
}
#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::_NotFound => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (
            status,
            Json(ErrorBody {
                error: self.to_string(),
            }),
        )
            .into_response()
    }
}

impl From<password_hash::Error> for ApiError {
    fn from(_: password_hash::Error) -> Self {
        ApiError::Internal
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db_err) = &err {
            if db_err.message().contains("UNIQUE constraint failed") {
                return ApiError::Conflict("Username already exists");
            }
        }
        ApiError::Internal
    }
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("env var error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("migration error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

}
