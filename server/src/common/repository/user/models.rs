use serde::{Serialize, Deserialize};


#[derive(PartialEq, Debug)]
pub enum AuthenticateResult {
    Success{ id: i64 },
    Failure
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum DeveloperOrEmployer {
    Developer = 0,
    Employer = 1
}