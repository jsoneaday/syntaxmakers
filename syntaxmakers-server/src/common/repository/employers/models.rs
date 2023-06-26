use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Employer {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub company_id: i64
}

pub struct NewEmployer {
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub company_id: i64
}