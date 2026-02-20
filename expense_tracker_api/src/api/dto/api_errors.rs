use crate::domain::errors::{DomainError, ValidationError};
use argon2::password_hash;
use axum::{
    Json,
    http::{self, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::{self, Error};
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(&'static str),
    #[error("{0}")]
    NotFound(&'static str),
    #[error("{0}")]
    Conflict(&'static str),
    #[error("internal server error")]
    Internal,
    #[error("Invalid credentials")]
    Unauthorized,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
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

impl From<ValidationError> for ApiError {
    fn from(err: ValidationError) -> Self {
        match err {
            ValidationError::InvalidLength => ApiError::BadRequest("Invalid length"),
            ValidationError::FieldEmpty => ApiError::BadRequest("Field must not be empty"),
            ValidationError::InvalidCharacter => {
                ApiError::BadRequest("Field contains invalid characters")
            }
            ValidationError::InvalidStartCharacter => {
                ApiError::BadRequest("Field must start with alphanumeric character")
            }
            ValidationError::InvalidFormat => ApiError::BadRequest("Invalid format"),
            ValidationError::InvalidAmount => ApiError::BadRequest("Invalid amount"),
            ValidationError::InvalidCategory => ApiError::BadRequest("Invalid Category"),
        }
    }
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::Validation(v) => v.into(),
            DomainError::Conflict => ApiError::Conflict("user already exists"),
        }
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
impl From<std::env::VarError> for ApiError {
    fn from(_: std::env::VarError) -> Self {
        ApiError::Internal
    }
}
impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        ApiError::Internal
    }
}
impl From<http::header::ToStrError> for ApiError {
    fn from(_: http::header::ToStrError) -> Self {
        ApiError::Internal
    }
}
impl From<std::num::ParseIntError> for ApiError {
    fn from(_: std::num::ParseIntError) -> Self {
        ApiError::Internal
    }
}
