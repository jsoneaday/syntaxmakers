use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(PartialEq, Debug)]
pub enum AuthenticateResult {
    Success{ id: i64 },
    Failure
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum RepoDeveloperOrEmployer {
    Developer = 0,
    Employer = 1
}

/// Never never return this object!
#[derive(FromRow, Debug, Clone)]
pub struct Password {
    pub password: String
}

/// Used for both dev and employer
pub struct ChangePassword {
    /// dev or employer id
    pub id: i64,
    pub old_password: String,
    pub new_password: String,
    pub dev_or_emp: RepoDeveloperOrEmployer
}

#[derive(Debug)]
pub struct RepoResetPassword {
    pub user_id: i64,
    pub new_password: String,
    pub dev_or_emp: RepoDeveloperOrEmployer,
    pub unique_key: Uuid,
}