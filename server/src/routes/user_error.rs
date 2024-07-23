use actix_http::StatusCode;
use actix_web::{ResponseError, HttpResponse, http::header::ContentType};
use derive_more::Error;
use crate::common::{emailer::model::EmailError, repository::error::{EMAIL_ALREADY_CONFIRMED, EMAIL_CONFIRM_FAILED, EMAIL_CONFIRM_FAILED_NEWER_EXISTS, EMAIL_CONFIRM_FOR_PROFILE_UPDATE_FAILED, EMAIL_CONFIRM_INVALID_PARAMS, EMAIL_CONFIRM_INVALID_UNIQUE_KEY, EMAIL_CONFIRM_NOT_FOUND}};

#[derive(Debug, Error, PartialEq)]
pub enum UserError {
    InternalError,
    ValidationError { field: String },
    AuthenticationFailed,
    AuthorizationFailed,
    EmailSendFailed,
    EmailAlreadyInUse,
    EmailNeedsConfirmation,
    EmailAlreadyConfirmed,
    EmailConfirmationNotFound,
    EmailConfirmFailedNewerExists,
    EmailConfirmInvalidParams,
    EmailConfirmationFailed,
    EmailConfirmInvalidUniqueKey,
    EmailConfirmForProfileUpdateFailed,
    UsernameAlreadyInUse,
    CompanyNameAlreadyInUse,
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::InternalError => write!(f, "{}", "An internal error occurred. Please try again later."),
            UserError::ValidationError { field } => write!(f, "{}, {}", "Validation error on field: {}", field),
            UserError::AuthenticationFailed => write!(f, "{}", "Authentication Failed. Email or password is incorrect."),
            UserError::AuthorizationFailed => write!(f, "{}", "Authorization Failed."),
            UserError::EmailSendFailed => write!(f, "{}", "Sending email failed."),
            UserError::EmailAlreadyInUse => write!(f, "{}", "Email already in use."),
            UserError::EmailNeedsConfirmation => write!(f, "{}", "Email needs confirmation."),
            UserError::EmailAlreadyConfirmed => write!(f, "{}", EMAIL_ALREADY_CONFIRMED),
            UserError::EmailConfirmationNotFound => write!(f, "{}", EMAIL_CONFIRM_NOT_FOUND),
            UserError::EmailConfirmFailedNewerExists => write!(f, "{}", EMAIL_CONFIRM_FAILED_NEWER_EXISTS),
            UserError::EmailConfirmInvalidParams => write!(f, "{}", EMAIL_CONFIRM_INVALID_PARAMS),
            UserError::EmailConfirmationFailed => write!(f, "{}", EMAIL_CONFIRM_FAILED),
            UserError::EmailConfirmInvalidUniqueKey => write!(f, "{}", EMAIL_CONFIRM_INVALID_UNIQUE_KEY),
            UserError::EmailConfirmForProfileUpdateFailed => write!(f, "{}", EMAIL_CONFIRM_FOR_PROFILE_UPDATE_FAILED),
            UserError::UsernameAlreadyInUse => write!(f, "{}", "Username already in use."),
            UserError::CompanyNameAlreadyInUse => write!(f, "{}", "Company name already in use.")
        }
    }
}

impl UserError {
    pub fn from_sqlx_to_user_error(e: sqlx::Error) -> UserError {
        match e {
            sqlx::Error::RowNotFound => UserError::InternalError,
            sqlx::Error::ColumnDecode { .. } => UserError::InternalError,
            sqlx::Error::Decode(_) => UserError::InternalError,
            sqlx::Error::PoolTimedOut => UserError::InternalError,
            sqlx::Error::PoolClosed => UserError::InternalError,
            sqlx::Error::WorkerCrashed => UserError::InternalError,
            #[cfg(feature = "migrate")]
            sqlx::Error::Migrate(_) => UserError::InternalError,
            _ => UserError::InternalError,
        }
    }

    pub fn from_email_to_user_error(e: EmailError) -> UserError {
        match e {
            EmailError::EmailSendFailed => UserError::EmailSendFailed,
            EmailError::EmailBodyInvalidOnlyPlainTextAllowed => UserError::EmailSendFailed,
            EmailError::EmailConfirmationSendFailed => UserError::EmailSendFailed,
            EmailError::SqlxErrorDatabaseError => UserError::InternalError,
            EmailError::SqlxErrorEmailConfirmationNotFound => UserError::EmailConfirmationNotFound,
            EmailError::SqlxErrorEmailConfirmInvalidParams => UserError::EmailConfirmInvalidParams, 
            EmailError::SqlxErrorEmailAlreadyConfirmed => UserError::EmailAlreadyConfirmed,
            EmailError::SqlxErrorEmailConfirmFailedNewerExists => UserError::EmailConfirmFailedNewerExists,
            EmailError::SqlxErrorEmailConfirmForProfileUpdateFailed => UserError::EmailConfirmForProfileUpdateFailed,
            EmailError::SqlxErrorEmailConfirmInvalidUniqueKey => UserError::EmailConfirmInvalidUniqueKey,
            EmailError::SqlxErrorEmailConfirmationAttemptFailed => UserError::EmailConfirmationFailed,
            #[cfg(feature = "migrate")]
            sqlx::Error::Migrate(_) => UserError::InternalError,
        }
    }
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            UserError::AuthenticationFailed => StatusCode::UNAUTHORIZED,
            UserError::AuthorizationFailed => StatusCode::UNAUTHORIZED,
            UserError::EmailSendFailed => StatusCode::NOT_ACCEPTABLE,
            UserError::EmailAlreadyInUse => StatusCode::NOT_ACCEPTABLE,
            UserError::EmailConfirmInvalidParams => StatusCode::NOT_ACCEPTABLE,
            UserError::EmailNeedsConfirmation => StatusCode::NOT_ACCEPTABLE,
            UserError::EmailConfirmationNotFound => StatusCode::NOT_ACCEPTABLE,
            UserError::EmailAlreadyConfirmed => StatusCode::ALREADY_REPORTED,
            UserError::EmailConfirmationFailed => StatusCode::NOT_ACCEPTABLE,
            UserError::EmailConfirmInvalidUniqueKey => StatusCode::NOT_ACCEPTABLE,  
            UserError::EmailConfirmFailedNewerExists=> StatusCode::NOT_ACCEPTABLE,
            UserError::EmailConfirmForProfileUpdateFailed => StatusCode::NOT_ACCEPTABLE,          
            UserError::CompanyNameAlreadyInUse => StatusCode::NOT_ACCEPTABLE,
            UserError::UsernameAlreadyInUse => StatusCode::NOT_ACCEPTABLE
        }
    }
}

impl Into<UserError> for sqlx::Error {
    fn into(self) -> UserError {
        UserError::from_sqlx_to_user_error(self)
    }
}

impl From<EmailError> for UserError {
    fn from(err: EmailError) -> Self {
        UserError::from_email_to_user_error(err)
    }
}