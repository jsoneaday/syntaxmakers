use derive_more::Error;
use std::marker::Send;

pub const IS_REMOTE_CONSTRAINT_ERROR: &str = "When is_remote is true country_id must be None";
pub const DATABASE_QUERY_FAILED: &str = "Database query has failed";
pub const PASSWORD_CHANGE_FAILED: &str = "Password change failed";
pub const EMAIL_CONFIRM_INVALID_PARAMS: &str = "Email confirm invalid parameters";
pub const EMAIL_CONFIRM_FAILED: &str = "Email confirm failed";
pub const EMAIL_CONFIRM_NOT_FOUND: &str = "Email confirm not found";
pub const EMAIL_ALREADY_CONFIRMED: &str = "Email already confirmed";
pub const EMAIL_CONFIRM_INVALID_UNIQUE_KEY: &str = "Email confirm failed, invalid unique key";
pub const EMAIL_CONFIRM_FAILED_NEWER_EXISTS: &str = "Newer email confirmations exist than the one presented";
pub const EMAIL_CONFIRM_FOR_PROFILE_UPDATE_FAILED: &str = "Email confirm for profile update has failed";
pub const USER_NOT_FOUND_BY_EMAIL: &str = "User not found by email";

#[derive(Error, Debug, PartialEq)]
pub enum SqlxError {
    IsRemoteContstraintError,
    DatabaseQueryFailed,
    PasswordChangeFailed,
    EmailConfirmFailed,
    EmailConfirmInvalidParams,
    EmailConfirmNotFound,
    EmailAlreadyConfirmed,
    EmailConfirmInvalidUniqueKey,
    EmailConfirmFailedNewerExists,
    EmailConfirmForProfileUpdateFailed,
    UserNotFoundByEmail
}

impl std::fmt::Display for SqlxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlxError::IsRemoteContstraintError => write!(f, "{}", IS_REMOTE_CONSTRAINT_ERROR),   
            SqlxError::DatabaseQueryFailed => write!(f, "{}", DATABASE_QUERY_FAILED),         
            SqlxError::PasswordChangeFailed => write!(f, "{}", PASSWORD_CHANGE_FAILED),
            SqlxError::EmailConfirmFailed => write!(f, "{}", EMAIL_CONFIRM_FAILED),
            SqlxError::EmailConfirmInvalidParams => write!(f, "{}", EMAIL_CONFIRM_INVALID_PARAMS),
            SqlxError::EmailConfirmNotFound => write!(f, "{}", EMAIL_CONFIRM_NOT_FOUND),
            SqlxError::EmailAlreadyConfirmed => write!(f, "{}", EMAIL_ALREADY_CONFIRMED),
            SqlxError::EmailConfirmInvalidUniqueKey => write!(f, "{}", EMAIL_CONFIRM_INVALID_UNIQUE_KEY),
            SqlxError::EmailConfirmFailedNewerExists => write!(f, "{}", EMAIL_CONFIRM_FAILED_NEWER_EXISTS),
            SqlxError::EmailConfirmForProfileUpdateFailed => write!(f, "{}", EMAIL_CONFIRM_FOR_PROFILE_UPDATE_FAILED),
            SqlxError::UserNotFoundByEmail => write!(f, "{}", USER_NOT_FOUND_BY_EMAIL)
        }
    }
}

impl sqlx::error::DatabaseError for SqlxError {
    fn message(&self) -> &str {
        println!("sqlxerror message {}", self.to_string());
        match self {
            SqlxError::IsRemoteContstraintError => IS_REMOTE_CONSTRAINT_ERROR,   
            SqlxError::DatabaseQueryFailed => DATABASE_QUERY_FAILED,         
            SqlxError::PasswordChangeFailed => PASSWORD_CHANGE_FAILED,
            SqlxError::EmailConfirmFailed => EMAIL_CONFIRM_FAILED,
            SqlxError::EmailConfirmInvalidParams => EMAIL_CONFIRM_INVALID_PARAMS,
            SqlxError::EmailConfirmNotFound => EMAIL_CONFIRM_NOT_FOUND,
            SqlxError::EmailAlreadyConfirmed => EMAIL_ALREADY_CONFIRMED,
            SqlxError::EmailConfirmInvalidUniqueKey => EMAIL_CONFIRM_INVALID_UNIQUE_KEY,
            SqlxError::EmailConfirmFailedNewerExists => EMAIL_CONFIRM_FAILED_NEWER_EXISTS,
            SqlxError::EmailConfirmForProfileUpdateFailed => EMAIL_CONFIRM_FOR_PROFILE_UPDATE_FAILED,
            SqlxError::UserNotFoundByEmail => USER_NOT_FOUND_BY_EMAIL,
        }
    }

    fn kind(&self) -> sqlx::error::ErrorKind {
        sqlx::error::ErrorKind::ForeignKeyViolation
    }

    fn as_error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        self
    }

    fn as_error_mut(&mut self) -> &mut (dyn std::error::Error + Send + Sync + 'static) {
        self
    }

    fn into_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync + 'static> {
        self
    }
}
