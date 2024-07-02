use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Job {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub employer_id: i64,
    pub employer_name: String,
    pub company_id: i64,
    pub company_name: String,
    pub company_logo: Option<Vec<u8>>,
    pub title: String,
    pub description: String,
    pub is_remote: bool,
    pub country_id: Option<i64>,
    pub country_name: Option<String>,
    pub primary_lang_id: i64,
    pub primary_lang_name: String,
    pub secondary_lang_id: Option<i64>,
    pub secondary_lang_name: Option<String>,
    pub industry_id: i64,
    pub industry_name: String,
    pub salary_id: i64,
    pub salary: i32
}

#[derive(FromRow, Debug, Clone)]
pub struct JobApplied {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Application table's created_at value
    pub dev_applied_at: DateTime<Utc>,
    pub employer_id: i64,
    pub employer_name: String,
    pub company_id: i64,
    pub company_name: String,
    pub company_logo: Option<Vec<u8>>,
    pub title: String,
    pub description: String,
    pub is_remote: bool,
    pub country_id: Option<i64>,
    pub country_name: Option<String>,
    pub primary_lang_id: i64,
    pub primary_lang_name: String,
    pub secondary_lang_id: Option<i64>,
    pub secondary_lang_name: Option<String>,
    pub industry_id: i64,
    pub industry_name: String,
    pub salary_id: i64,
    pub salary: i32
}

#[derive(FromRow, Debug, Clone)]
pub struct JobCountry {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub job_id: i64,
    pub country_id: i64,
}

#[derive(Clone)]
pub struct NewJob {
    pub employer_id: i64,
    pub title: String,
    pub description: String,
    pub is_remote: bool,
    pub country_id: Option<i64>,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>,
    pub industry_id: i64,
    pub salary_id: i64
}

pub struct UpdateJob {
    pub id: i64,
    pub employer_id: i64,
    pub title: String,
    pub description: String,
    pub is_remote: bool,
    pub country_id: Option<i64>,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>,
    pub industry_id: i64,
    pub salary_id: i64
}