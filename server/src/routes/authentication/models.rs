use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginCredential {
    pub is_dev_or_emp: DeveloperOrEmployer,
    pub email: String,
    pub password: String
}

#[derive(Deserialize, PartialEq)]
pub enum DeveloperOrEmployer {
    Developer = 0,
    Employer = 1
}