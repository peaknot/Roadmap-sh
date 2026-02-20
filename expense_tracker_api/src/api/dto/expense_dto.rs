use crate::api::dto::ApiError;
use crate::domain::{Amount, Category, Description, NewExpense};
use chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewExpenseRequest {
    expense_desc: String,
    amount: i64,
    category: String,
}

impl TryFrom<NewExpenseRequest> for NewExpense {
    type Error = ApiError;
    fn try_from(input: NewExpenseRequest) -> Result<Self, Self::Error> {
        let valid_description = Description::try_from(input.expense_desc.as_str())?;
        let valid_amount = Amount::try_from(input.amount)?;
        let category = Category::try_from(input.category)?;

        let created_at = Utc::now().to_rfc3339();

        Ok(Self {
            expense_desc: valid_description,
            amount: valid_amount,
            category,
            created_at,
        })
    }
}
