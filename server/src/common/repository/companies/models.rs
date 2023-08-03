use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Company {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub logo: Option<Vec<u8>>
}

#[derive(Debug)]
pub struct NewCompany {
    pub name: String
}