use crate::common::{repository::base::Repository, authentication::auth_service::{AuthKeys, Authenticator}};

pub struct AppState<T: Repository, U: Authenticator> {
    pub repo: T,
    pub auth_service: U,
    pub auth_keys: AuthKeys
}