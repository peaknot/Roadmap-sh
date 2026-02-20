use crate::domain::{Amount, Category, Description};
use crate::{api::dto::ApiError, domain::Expense};
use serde::Serialize;
use sqlx::prelude::FromRow;
#[derive(FromRow, Serialize)]
pub struct ExpenseRow {
    id: i64,
    expense_desc: String,
    amount: i64,
    category: String,
    created_at: String,
}

impl TryFrom<Expense> for ExpenseRow {
    type Error = ApiError;
    fn try_from(value: Expense) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            expense_desc: value.expense_desc.into_inner(),
            amount: value.amount.as_i64(),
            category: value.category.into_inner(),
            created_at: value.created_at,
        })
    }
}
#[derive(FromRow)]
pub struct ExpenseDbRow {
    id: i64,
    expense_desc: String,
    amount: i64,
    category: String,
    created_at: String,
}
impl TryFrom<ExpenseDbRow> for Expense {
    type Error = ApiError;

    fn try_from(row: ExpenseDbRow) -> Result<Self, Self::Error> {
        Ok(Expense {
            id: row.id,
            expense_desc: Description::try_from(row.expense_desc.as_str())?,
            amount: Amount::try_from(row.amount)?,
            category: Category::try_from(row.category)?,
            created_at: row.created_at,
        })
    }
}
