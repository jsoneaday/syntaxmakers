use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub old_token: String,
    pub dev_or_emp: RouteDeveloperOrEmployer
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ForgotPassword {
    pub email: String,
    pub dev_or_emp: RouteDeveloperOrEmployer
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RouteResetPassword {    
    pub user_id: i64,
    pub new_password: String,
    pub dev_or_emp: RouteDeveloperOrEmployer,
    pub unique_key: Uuid
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginCredential {
    pub dev_or_emp: RouteDeveloperOrEmployer,
    pub email: String,
    pub password: String
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub enum RouteDeveloperOrEmployer {
    Developer = 0,
    Employer = 1
}