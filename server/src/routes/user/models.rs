use serde::Deserialize;
use uuid::Uuid;
use crate::routes::authentication::models::RouteDeveloperOrEmployer;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRoute {
    /// developer id
    pub id: i64,
    pub old_password: String,
    pub new_password: String,
    pub dev_or_emp: RouteDeveloperOrEmployer
}

#[derive(Deserialize)]
pub struct ConfirmEmailQuery {
    pub is_dev: bool,
    pub profile_id: i64,
    pub new_email: String,
    pub unique_key: Option<Uuid>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendEmail {
    pub sender_emp_id: i64,
    pub receiver_dev_id: i64,
    pub subject: String,
    pub body: String
}