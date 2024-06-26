use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Developer {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>
}

/// Never never return this object!
#[derive(FromRow, Debug, Clone)]
pub struct DevPassword {
    pub password: String
}

pub struct NewDeveloper {
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>
}

pub struct UpdateDeveloper {
    pub id: i64,
    pub full_name: String,
    pub email: String,
    pub old_password: String,
    pub new_password: String,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>
}