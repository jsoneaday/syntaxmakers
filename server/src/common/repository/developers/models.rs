use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::common::authentication::password_hash::verify_password;

#[derive(FromRow, Debug, Clone)]
pub struct Developer {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    /// hashed password from db
    password: String,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>
}

impl Developer {
    pub fn new(id: i64, created_at: DateTime<Utc>, updated_at: DateTime<Utc>, user_name: String, full_name: String, password: String, email: String, primary_lang_id: i64, secondary_lang_id: Option<i64>) -> Self {
        Developer { 
            id, 
            user_name, 
            created_at, 
            updated_at, 
            full_name, 
            email, 
            password,
            primary_lang_id,
            secondary_lang_id
        }
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, argon2::password_hash::Error> {
        verify_password(password, &self.password)
    }
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