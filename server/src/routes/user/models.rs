use serde::Deserialize;
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