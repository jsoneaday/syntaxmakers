use crate::common::{repository::base::Repository, authentication::auth_service::AuthKeys};

pub struct AppState<T: Repository> {
    pub repo: T,
    pub auth_keys: AuthKeys
}