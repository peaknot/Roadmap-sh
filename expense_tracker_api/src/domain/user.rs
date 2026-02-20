use crate::domain::user_types::{Email, Password, UserName};
use crate::domain::{Amount, Category, Description};
#[derive(Clone)]
pub struct _User {
    pub id: i64,
    pub username: UserName,
    pub email: Email,
    pub password_hash: Password,
    pub created_at: i64,
}
pub struct NewUser {
    pub username: UserName,
    pub email: Email,
    pub password_hash: Password,
}
#[derive(Clone)]
pub struct Expense {
    pub id: i64,
    pub expense_desc: Description,
    pub amount: Amount,
    pub category: Category,
    pub created_at: String,
}

#[derive(Clone)]
pub struct NewExpense {
    pub expense_desc: Description,
    pub amount: Amount,
    pub category: Category,
    pub created_at: String,
}
