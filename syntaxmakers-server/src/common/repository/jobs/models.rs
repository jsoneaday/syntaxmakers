use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Job {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub employer_id: i64,
    pub title: String,
    pub description: String,
    pub is_remote: bool,
    pub headquarters_country_id: i64,
    pub primary_lang_id: i64,
    pub secondary_lang_id: i64,
    pub industry_id: i64,
    pub salary_id: i64
}

pub struct NewJob {
    pub employer_id: i64,
    pub title: String,
    pub description: String,
    pub is_remote: bool,
    pub headquarters_country_id: Option<i64>,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>,
    pub industry_id: i64,
    pub salary_id: i64
}