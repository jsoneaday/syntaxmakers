#[derive(PartialEq)]
pub enum AuthenticateResult {
    Success,
    Failure
}

#[derive(PartialEq)]
pub enum DeveloperOrEmployer {
    Developer = 0,
    Employer = 1
}