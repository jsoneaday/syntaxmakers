use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;
use crate::common::authentication::password_hash::verify_password;

#[derive(FromRow, Debug, Clone)]
pub struct Developer {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub description: String,
    /// hashed password from db
    password: String,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>
}

impl Developer {
    pub fn new(
        id: i64, 
        created_at: DateTime<Utc>, 
        updated_at: DateTime<Utc>, 
        user_name: String, 
        full_name: String, 
        password: String, 
        email: String, 
        description: String, 
        primary_lang_id: i64, 
        secondary_lang_id: Option<i64>
    ) -> Self {
        Developer { 
            id, 
            user_name, 
            created_at, 
            updated_at, 
            full_name, 
            email, 
            description,
            password,
            primary_lang_id,
            secondary_lang_id
        }
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, argon2::password_hash::Error> {
        verify_password(password, &self.password)
    }
}

/// When user profile is first created user must confirm email before profile can be used to login
/// When user updates their email their profile keeps the old email until they confirm the new email,
/// once they confirm new email, their profile is then updated with new email
#[derive(FromRow, Debug, Clone)]
pub struct DevEmailConfirm {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub developer_id: i64,
    pub is_confirmed: bool,
    pub is_valid: bool,
    pub new_email: String,
    pub unique_key: Uuid
}

pub struct NewDeveloper {
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub description: String,
    pub password: String,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>
}

pub struct UpdateDeveloper {
    pub id: i64,
    pub full_name: String,
    pub email: String,    
    pub description: String,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>
}
