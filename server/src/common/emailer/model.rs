use derive_more::{Display, Error};

#[derive(Error, Display, Debug)]
pub enum EmailError {
    #[display(fmt = "Confirmation email failed to send")]
    EmailConfirmationSendFailed,
    #[display(fmt = "Confirmation email receive processing failed")]
    EmailConfirmationReceiveFailed,
    #[display(fmt = "Sending email failed")]
    EmailSendFailed
}

impl sqlx::error::DatabaseError for EmailError {
    fn message(&self) -> &str {
        "Email error has occurred"
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