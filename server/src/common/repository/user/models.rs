
#[derive(PartialEq)]
pub enum AuthenticateResult {
    Success{ id: i64 },
    Failure
}

#[derive(PartialEq)]
pub enum DeveloperOrEmployer {
    Developer = 0,
    Employer = 1
}