use crate::common::{authentication::auth_keys_service::{AuthKeys, Authenticator}, emailer::emailer::EmailerSendService, repository::base::Repository};

pub struct AppState<T: Repository, E: EmailerSendService, U: Authenticator> {
    pub repo: T,
    pub emailer: E,
    pub auth_service: U,
    pub auth_keys: AuthKeys
}