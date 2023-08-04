use derive_more::{Display, Error};
use std::marker::Send;

#[derive(Error, Display, Debug)]
pub enum SqlxError {
    #[display(fmt = "When is_remote is true country_id must be None")]
    IsRemoteContstraintError
}

impl sqlx::error::DatabaseError for SqlxError {
    fn message(&self) -> &str {
        "A database error has occurred"
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