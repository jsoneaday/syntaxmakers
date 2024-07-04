use chrono::{DateTime, Utc};
use sqlx::FromRow;
use crate::common::authentication::password_hash::verify_password;

#[derive(FromRow, Debug, Clone)]
pub struct Employer {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
    password: String,
    pub email: String,
    pub company_id: i64
}

impl Employer {
    pub fn new(id: i64, created_at: DateTime<Utc>, updated_at: DateTime<Utc>, user_name: String, full_name: String, password: String, email: String, company_id: i64) -> Self {
        Employer { 
            id, 
            user_name, 
            created_at, 
            updated_at, 
            full_name, 
            email, 
            password,
            company_id
        }
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, argon2::password_hash::Error> {
        verify_password(password, &self.password)
    }
}

pub struct NewEmployer {
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub company_id: i64
}

/// does not include password!
pub struct UpdateEmployer {
    pub id: i64,
    pub full_name: String,
    pub email: String,    
    pub company_id: i64
}