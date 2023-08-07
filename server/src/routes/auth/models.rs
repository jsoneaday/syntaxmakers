use actix_jwt_auth_middleware::FromRequest;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, FromRequest)]
pub struct User {
    pub id: i32
}