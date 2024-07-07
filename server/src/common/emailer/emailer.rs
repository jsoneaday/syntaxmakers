use async_trait::async_trait;
use uuid::Uuid;
use super::model::EmailError;

pub struct Emailer;

#[async_trait]
pub trait EmailerService {
    async fn send_email_confirm_requirement(&self, profile_id: i64, new_email: String, unique_key: Uuid) -> Result<(), EmailError>;

    async fn receive_email_confirm(&self, profile_id: i64, new_email: String, unique_key: Uuid) -> Result<(), EmailError>;
}

#[allow(unused)]
#[async_trait]
impl EmailerService for Emailer {
    async fn send_email_confirm_requirement(&self, profile_id: i64, new_email: String, unique_key: Uuid) -> Result<(), EmailError> {
        Ok(())
    }

    async fn receive_email_confirm(&self, profile_id: i64, new_email: String, unique_key: Uuid) -> Result<(), EmailError> {
        Ok(())
    }
}
