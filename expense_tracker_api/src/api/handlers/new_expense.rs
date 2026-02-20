use crate::api::{
    AppState,
    dto::{ApiError, Claims, ExpenseResponseDTO, NewExpenseRequest},
};
use crate::domain::NewExpense;
use axum::{self, Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn new_expense(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<NewExpenseRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id: i64 = claims
        .sub
        .parse::<i64>()
        .map_err(|_| ApiError::Unauthorized)?;

    let new_expense: NewExpense = payload.try_into()?;

    let NewExpense {
        expense_desc,
        amount,
        category,
        created_at,
    } = new_expense;

    let expense: ExpenseResponseDTO = sqlx::query_as(
        r#"
            INSERT INTO expenses 
                (expense_desc, amount, category, created_at, user_id)
            VALUES
                (?1, ?2, ?3, ?4, ?5)
            RETURNING id, expense_desc, amount, category, created_at
        "#,
    )
    .bind(expense_desc.into_inner())
    .bind(amount.as_i64())
    .bind(category.into_inner())
    .bind(created_at)
    .bind(user_id)
    .fetch_one(&state.pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
        "msg": "Expense added successfully",
        "expense": expense
        })),
    ))
}
