use crate::api::{
    AppState,
    dto::{ApiError, Claims, ExpenseDbRow, ExpenseRow, QueryExpense},
};
use crate::domain::Expense;
use axum::{
    Json,
    extract::{Extension, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

pub async fn list_expense(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Query(param): Query<QueryExpense>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = claims
        .sub
        .parse::<i64>()
        .map_err(|_| ApiError::Unauthorized)?;

    //let like = param.search.as_deref().and_then(|s| {
    //let s = s.trim();
    //(!s.is_empty()).then(|| format!("%{}%", s))
    // });
    let search: Option<&str> = param
        .search
        .as_deref()
        .map(|s| s.trim())
        .filter(|f| !f.is_empty());

    let rows: Vec<ExpenseDbRow> = match search {
        Some(text) => {
            let pattern: String = format!("%{}%", text);
            sqlx::query_as::<_, ExpenseDbRow>(
                r#"
                SELECT id, expense_desc, amount, category, created_at
                FROM expenses
                WHERE user_id = ?1 
                    AND (
                        expense_desc LIKE ?2 COLLATE NOCASE
                        OR category LIKE ?2 COLLATE NOCASE
                        OR CAST(amount AS TEXT) LIKE ?2
                    )
                ORDER BY created_at DESC
                LIMIT 10;
            "#,
            )
            .bind(user_id)
            .bind(&pattern)
            .fetch_all(&state.pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, ExpenseDbRow>(
                r#"
                SELECT id, expense_desc, amount, category, created_at
                FROM expenses
                WHERE user_id = ?1
                ORDER BY created_at DESC
                LIMIT 20
            "#,
            )
            .bind(user_id)
            .fetch_all(&state.pool)
            .await?
        }
    };

    let response_rows: Vec<ExpenseRow> = rows
        .into_iter()
        .map(Expense::try_from)
        .collect::<Result<Vec<_>, ApiError>>()?
        .into_iter()
        .map(ExpenseRow::try_from)
        .collect::<Result<Vec<_>, ApiError>>()?;

    Ok((StatusCode::OK, Json(json!({"Expenses": response_rows}))))
}
