use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginCredential {
    pub email: String,
    pub password: String
}