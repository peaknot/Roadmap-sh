
use axum::{Extension, Json, extract::{Path, Query, State}, http::StatusCode, response::IntoResponse};
use chrono::{Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::prelude::FromRow;

use crate::{
    errors::ApiError,
    schema::{AppState, Claims},
};
#[derive(Serialize, FromRow)]
pub struct Expense {
    id: i64,
    expense_desc: String,
    amount: i64,
    category: String,
    created_at: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExpenseRequestDTO {
    expense_desc: String,
    amount: i64,
    category: Category,
}
#[derive(Deserialize)]
pub struct QueryExpense {
    search: Option<String>,
}
#[derive(Deserialize)]
pub struct UpdateRequestDTO {
    expense_desc: Option<String>,
    amount: Option<i64>,
}


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Category {
    Food,
    Fare,
}
impl Category {
    fn as_str(&self) -> &'static str {
        //TODO make case insensitive
        match self {
            Category::Food => "Food",
            Category::Fare => "Fare",
        }
    }
}

pub async fn new_expense(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<ExpenseRequestDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id: i64 = claims.sub.parse::<i64>()
        .map_err(|_| ApiError::Unauthorized)?;

    // TODO set max capacity of description
    let description: &str = payload.expense_desc.trim();
    if description.is_empty() {
        return Err(ApiError::BadRequest("Expense description must not be empty"));
    }
    // TODO set an upper bound for the amount
    if payload.amount <= 0 {
        return Err(ApiError::BadRequest("Amount must greater than 0"));
    }
    let dt_created: String = Utc::now().to_rfc3339();
    let category: &str = payload.category.as_str();
    
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

pub async fn list_expense(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>, 
    Query(param): Query<QueryExpense>
) -> Result<impl IntoResponse, ApiError> {
    let user_id = claims.sub.parse::<i64>()
        .map_err(|_| ApiError::Unauthorized)?;

    //let like = param.search.as_deref().and_then(|s| {
    //let s = s.trim();
    //(!s.is_empty()).then(|| format!("%{}%", s))
    // });
    let search: Option<&str> = param.search
        .as_deref()
        .map(|s| s.trim())
        .filter(|f| !f.is_empty());

    let rows: Vec<Expense> = match search {
        Some(text) => {
            let pattern: String = format!("%{}%", text);
            sqlx::query_as::<_,Expense>(r#"
                SELECT id, expense_desc, amount, category, created_at
                FROM expenses
                WHERE user_id = ?1 
                    AND (
                        expense_desc LIKE ?2 COLLATE NOCASE
                        OR category LIKE ?2 COLLATE NOCASE
                        OR CAST(amount AS TEXT) LIKE ?2
                    )
                ORDER BY created_at DESC;
            "#)
            .bind(user_id)
            .bind(&pattern)
            .fetch_all(&state.pool)
            .await?
        },
        None => {
            sqlx::query_as::<_,Expense>(r#"
                SELECT id, expense_desc, amount, category, created_at
                FROM expenses
                WHERE user_id = ?1
                ORDER BY created_at DESC
                LIMIT 20
            "#)
            .bind(user_id)
            .fetch_all(&state.pool)
            .await?
        }
    };
    Ok((StatusCode::OK, Json(json!({"Expenses": rows}))))
}

pub async fn update_expense(
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateRequestDTO>
) -> Result<impl IntoResponse, ApiError> {
    //TODO sanitize and validate UpdateRequestDTO inputs
    let user_id: i64 = claims.sub.parse()
        .map_err(|_| ApiError::Unauthorized)?;
    //TODO also create updated_at for created_at if update is success
    let update = sqlx::query_as::<_, Expense>(
        r#"
            UPDATE expenses
            SET expense_desc = COALESCE(?3, expense_desc), amount = COALESCE(?4, amount)
            WHERE id = ?2 AND user_id = ?1 
            RETURNING id, expense_desc, amount, category, created_at
        "#
    )
    .bind(user_id)
    .bind(id)
    .bind(payload.expense_desc)
    .bind(payload.amount)
    .fetch_optional(&state.pool)
    .await?;
    
    match update {
        Some(u) => {
            Ok((
                StatusCode::OK,
                Json(json!({"msg": "expense updated successfully", "expense": u}))
            ))
        },
        None => {
            Ok((
                StatusCode::NOT_FOUND,
                Json(json!({"msg": "expense not found"}))
            ))
        }
    }
}

pub async fn delete_expense(
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    State(state): State<AppState>,

) -> Result<StatusCode, ApiError> {
    
    //todo validate if such expense exist
    let user_id: i64 = claims.sub.parse()
        .map_err(|_| ApiError::Unauthorized)?;

    let delete = sqlx::query(
        r#"
            DELETE FROM expenses
            WHERE user_id = ?1 AND id = ?2
        "#
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