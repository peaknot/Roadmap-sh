use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
#[derive(Deserialize)]
pub struct LoginRequestDTO {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, FromRow)]
pub struct UserResponseDTO {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: String,
}
#[derive(Serialize, FromRow)]
pub struct ExpenseResponseDTO {
    pub id: i64,
    pub expense_desc: String,
    pub amount: i64,
    pub category: String,
    pub created_at: String,
}
#[derive(Deserialize)]
pub struct UpdateRequestDTO {
    pub expense_desc: Option<String>,
    pub amount: Option<i64>,
}

#[derive(Deserialize)]
pub struct QueryExpense {
    pub search: Option<String>,
}
