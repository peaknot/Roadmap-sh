use thiserror::{self, Error};

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
