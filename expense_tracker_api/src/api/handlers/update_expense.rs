use crate::api::{
    AppState,
    dto::{ApiError, Claims, ExpenseResponseDTO, UpdateRequestDTO},
};
use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

pub async fn update_expense(
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateRequestDTO>,
) -> Result<impl IntoResponse, ApiError> {
    //TODO sanitize and validate UpdateRequestDTO inputs
    let user_id: i64 = claims.sub.parse().map_err(|_| ApiError::Unauthorized)?;
    //TODO also create updated_at for created_at if update is success
    let update: Option<ExpenseResponseDTO> = sqlx::query_as(
        r#"
            UPDATE expenses
            SET expense_desc = COALESCE(?3, expense_desc), amount = COALESCE(?4, amount)
            WHERE id = ?2 AND user_id = ?1 
            RETURNING id, expense_desc, amount, category, created_at
        "#,
    )
    .bind(user_id)
    .bind(id)
    .bind(payload.expense_desc)
    .bind(payload.amount)
    .fetch_optional(&state.pool)
    .await?;

    match update {
        Some(u) => Ok((
            StatusCode::OK,
            Json(json!({"msg": "expense updated successfully", "expense": u})),
        )),
        None => Ok((
            StatusCode::NOT_FOUND,
            Json(json!({"msg": "expense not found"})),
        )),
    }
}
