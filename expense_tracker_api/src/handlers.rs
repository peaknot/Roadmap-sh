
use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    errors::ApiError,
    schema::{AppState, Claims},
};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Expense {
    expense_desc: String,
    amount: i64,
    category: Category,
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Category {
    Food,
    Fare,
}
impl Category {
    fn as_str(&self) -> &'static str {
        match self {
            Category::Food => "Food",
            Category::Fare => "Fare",
        }
    }
}

pub async fn new_expense(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<Expense>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = claims.subject.parse::<i64>()
        .map_err(|_| ApiError::Unauthorized)?;

    // TODO set max capacity of description
    let description = payload.expense_desc.trim();
    if description.is_empty() {
        return Err(ApiError::BadRequest("Expense description must not be empty"));
    }
    // TODO set an upper bound for the amount
    if payload.amount <= 0 {
        return Err(ApiError::BadRequest("Amount must greater than 0"));
    }
    let dt_created = Utc::now().to_rfc3339();
    let category = payload.category.as_str();

    let result = sqlx::query(
        r#"
            INSERT INTO expenses 
                (expense_desc, amount, category, created_at, user_id)
            VALUES
                (?1, ?2, ?3, ?4, ?5)
        "#
    )
    .bind(description)
    .bind(payload.amount)
    .bind(category)
    .bind(dt_created)
    .bind(user_id)
    .execute(&state.pool)
    .await?;

    let expense_id = result.last_insert_rowid();
    Ok((StatusCode::CREATED, Json(json!({"msg": "Expense added successfully", "id": expense_id}))))
}
