use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid legnth")]
    InvalidLength,
    #[error("Field must not be empty")]
    FieldEmpty,
    #[error("Field contains invalid characters")]
    InvalidCharacter,
    #[error("Field must start with alphanumeric charcter")]
    InvalidStartCharacter,
    #[error("Invalid format")]
    InvalidFormat,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Invalid category")]
    InvalidCategory,
}
#[derive(Debug, Error)]
pub enum DomainError {
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error("username already taken")]
    Conflict,
}
