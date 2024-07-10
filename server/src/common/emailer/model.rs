use derive_more::Error;
use crate::common::repository::error::{DATABASE_QUERY_FAILED, EMAIL_ALREADY_CONFIRMED, EMAIL_CONFIRM_FAILED, EMAIL_CONFIRM_FAILED_NEWER_EXISTS, EMAIL_CONFIRM_FOR_PROFILE_UPDATE_FAILED, EMAIL_CONFIRM_INVALID_PARAMS, EMAIL_CONFIRM_INVALID_UNIQUE_KEY, EMAIL_CONFIRM_NOT_FOUND};

const EMAIL_CONFIRMATION_SEND_FAILED: &str = "Confirmation email failed to send";
const SENDING_EMAIL_FAILED: &str = "Sending email failed";

#[derive(Error, Debug, PartialEq)]
pub enum EmailError {
    EmailConfirmationSendFailed,
    EmailSendFailed,
    SqlxErrorDatabaseError,
    SqlxErrorEmailConfirmInvalidParams,
    SqlxErrorEmailConfirmationAttemptFailed,
    SqlxErrorEmailConfirmationNotFound, 
    SqlxErrorEmailAlreadyConfirmed,   
    SqlxErrorEmailConfirmInvalidUniqueKey,
    SqlxErrorEmailConfirmFailedNewerExists,
    SqlxErrorEmailConfirmForProfileUpdateFailed,
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &EmailError::EmailConfirmationSendFailed => write!(f, "{}", EMAIL_CONFIRMATION_SEND_FAILED),
            &EmailError::EmailSendFailed => write!(f, "{}", SENDING_EMAIL_FAILED),
            &EmailError::SqlxErrorDatabaseError => write!(f, "{}", DATABASE_QUERY_FAILED),
            &EmailError::SqlxErrorEmailConfirmInvalidParams => write!(f, "{}", EMAIL_CONFIRM_INVALID_PARAMS),
            &EmailError::SqlxErrorEmailConfirmationAttemptFailed => write!(f, "{}", EMAIL_CONFIRM_FAILED),
            &EmailError::SqlxErrorEmailConfirmationNotFound => write!(f, "{}", EMAIL_CONFIRM_NOT_FOUND),
            &EmailError::SqlxErrorEmailAlreadyConfirmed => write!(f, "{}", EMAIL_ALREADY_CONFIRMED),
            &EmailError::SqlxErrorEmailConfirmInvalidUniqueKey => write!(f, "{}", EMAIL_CONFIRM_INVALID_UNIQUE_KEY),
            &EmailError::SqlxErrorEmailConfirmFailedNewerExists => write!(f, "{}", EMAIL_CONFIRM_FAILED_NEWER_EXISTS),
            &EmailError::SqlxErrorEmailConfirmForProfileUpdateFailed => write!(f, "{}", EMAIL_CONFIRM_FOR_PROFILE_UPDATE_FAILED),
        }
    }
}

impl From<sqlx::Error> for EmailError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(db_err) => {
                println!("EmailError db_err {}", db_err.message());
                if db_err.message() == DATABASE_QUERY_FAILED {
                    EmailError::SqlxErrorDatabaseError
                } else if db_err.message() == EMAIL_CONFIRM_INVALID_PARAMS {
                    EmailError::SqlxErrorEmailConfirmInvalidParams
                } else if db_err.message() == EMAIL_CONFIRM_FAILED {
                    EmailError::SqlxErrorEmailConfirmationAttemptFailed
                } else if db_err.message() == EMAIL_CONFIRM_NOT_FOUND {
                    EmailError::SqlxErrorEmailConfirmationNotFound
                } else if db_err.message() == EMAIL_ALREADY_CONFIRMED{
                    EmailError::SqlxErrorEmailAlreadyConfirmed
                } else if db_err.message() == EMAIL_CONFIRM_INVALID_UNIQUE_KEY {
                    EmailError::SqlxErrorEmailConfirmInvalidUniqueKey
                } else if db_err.message() == EMAIL_CONFIRM_FAILED_NEWER_EXISTS {
                    EmailError::SqlxErrorEmailConfirmFailedNewerExists
                } else if db_err.message() == EMAIL_CONFIRM_FOR_PROFILE_UPDATE_FAILED {
                    EmailError::SqlxErrorEmailConfirmForProfileUpdateFailed
                } else {
                    EmailError::SqlxErrorDatabaseError
                }
            },
            _ => EmailError::SqlxErrorDatabaseError
        }
    }
}

impl sqlx::error::DatabaseError for EmailError {
    fn message(&self) -> &str {
        match self {
            EmailError::SqlxErrorDatabaseError => DATABASE_QUERY_FAILED,
            EmailError::SqlxErrorEmailConfirmInvalidParams => EMAIL_CONFIRM_INVALID_PARAMS,
            EmailError::SqlxErrorEmailConfirmationAttemptFailed => EMAIL_CONFIRM_FAILED,
            EmailError::SqlxErrorEmailConfirmationNotFound => EMAIL_CONFIRM_NOT_FOUND,
            EmailError::SqlxErrorEmailAlreadyConfirmed => EMAIL_ALREADY_CONFIRMED,
            EmailError::SqlxErrorEmailConfirmInvalidUniqueKey => EMAIL_CONFIRM_INVALID_UNIQUE_KEY,
            EmailError::SqlxErrorEmailConfirmFailedNewerExists => EMAIL_CONFIRM_FAILED_NEWER_EXISTS,
            EmailError::SqlxErrorEmailConfirmForProfileUpdateFailed => EMAIL_CONFIRM_FOR_PROFILE_UPDATE_FAILED,
            _ => "Email confirmation failed"
        }
    }

    fn kind(&self) -> sqlx::error::ErrorKind {
        sqlx::error::ErrorKind::Other
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