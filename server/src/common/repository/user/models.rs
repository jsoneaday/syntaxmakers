use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(PartialEq, Debug)]
pub enum AuthenticateResult {
    Success{ id: i64 },
    Failure
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum DeveloperOrEmployer {
    Developer = 0,
    Employer = 1
}

/// Never never return this object!
#[derive(FromRow, Debug, Clone)]
pub struct Password {
    pub password: String
}

pub struct ChangePassword {
    pub id: i64,
    pub old_password: String,
    pub new_password: String,
}