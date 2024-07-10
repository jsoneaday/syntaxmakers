use serde::Deserialize;
use uuid::Uuid;
use crate::routes::authentication::models::DeveloperOrEmployer;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRoute {
    /// developer id
    pub id: i64,
    pub old_password: String,
    pub new_password: String,
    pub dev_or_emp: DeveloperOrEmployer
}
#[derive(Deserialize)]
pub struct ConfirmEmailQuery {
    pub is_dev: bool,
    pub profile_id: i64,
    pub new_email: String,
    pub unique_key: Option<Uuid>
}