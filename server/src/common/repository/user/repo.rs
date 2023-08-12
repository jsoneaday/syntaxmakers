use sqlx::{Postgres, query_as, Pool};
use crate::common::repository::{user::models::AuthenticateResult, base::{EntityId, DbRepo, ConnGetter}};
use async_trait::async_trait;

mod internal {
    use super::*;    

    pub async fn authenticate(conn: &Pool<Postgres>, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error> {
        let result = query_as::<_, EntityId>("select id from user where email = $1 and password = $2")
            .bind(email)
            .bind(password)
            .fetch_optional(conn)
            .await;

        match result {
            Ok(opt_entity) => match opt_entity {
                Some(_) => Ok(AuthenticateResult::Success),
                None => Ok(AuthenticateResult::Failure)
            },
            Err(e) => Err(e.into())
        }
    }
}

#[async_trait]
pub trait AuthenticateFn {
    async fn authenticate(&self, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error>;
}

#[async_trait]
impl AuthenticateFn for DbRepo {
    async fn authenticate(&self, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error> {
        internal::authenticate(self.get_conn(), email, password).await
    }
}