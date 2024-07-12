use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub old_token: String,
    pub dev_or_emp: DeveloperOrEmployer
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ForgotPassword {
    pub email: String,
    pub dev_or_emp: DeveloperOrEmployer
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginCredential {
    pub dev_or_emp: DeveloperOrEmployer,
    pub email: String,
    pub password: String
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub enum DeveloperOrEmployer {
    Developer = 0,
    Employer = 1
}