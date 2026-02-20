use crate::api::{
    AppState,
    dto::{ApiError, Claims},
};
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
};

pub async fn delete_expense(
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {
    //todo validate if such expense exist
    let user_id: i64 = claims.sub.parse().map_err(|_| ApiError::Unauthorized)?;

    let delete = sqlx::query(
        r#"
            DELETE FROM expenses
            WHERE user_id = ?1 AND id = ?2
        "#,
    )
    .bind(user_id)
    .bind(id)
    .execute(&state.pool)
    .await?;

    if delete.rows_affected() == 0 {
        return Err(ApiError::NotFound("expense not found"));
    }
    Ok(StatusCode::NO_CONTENT)
}
