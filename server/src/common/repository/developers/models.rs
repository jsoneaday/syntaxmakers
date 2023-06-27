use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Developer {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
    pub primary_lang_id: i64
}

pub struct NewDeveloper {
    pub user_name: String,
    pub full_name: String,
    pub primary_lang_id: i64
}