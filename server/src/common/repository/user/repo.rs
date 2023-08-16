use sqlx::{Postgres, query_as, Pool};
use crate::common::repository::{user::models::AuthenticateResult, base::{EntityId, DbRepo, ConnGetter}};
use async_trait::async_trait;
use crate::common::repository::user::models::DeveloperOrEmployer;

mod internal {  
    use super::*;    

    pub async fn authenticate_db(conn: &Pool<Postgres>, is_dev_or_emp: DeveloperOrEmployer, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error> {
        let query = if is_dev_or_emp == DeveloperOrEmployer::Developer {
            "select id from developer where email = $1 and password = $2"
        } else {
            "select id from employer where email = $1 and password = $2"
        };

        let result = query_as::<_, EntityId>(query)
            .bind(email)
            .bind(password)
            .fetch_optional(conn)
            .await;

        match result {
            Ok(opt_entity) => match opt_entity {
                Some(entity) => Ok(AuthenticateResult::Success { id: entity.id }),
                None => Ok(AuthenticateResult::Failure)
            },
            Err(e) => Err(e.into())
        }
    }
}

#[async_trait]
pub trait AuthenticateDbFn {
    async fn authenticate_db(&self, is_dev_or_emp: DeveloperOrEmployer, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error>;
}

#[async_trait]
impl AuthenticateDbFn for DbRepo {
    async fn authenticate_db(&self, is_dev_or_emp: DeveloperOrEmployer, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error> {
        internal::authenticate_db(self.get_conn(), is_dev_or_emp, email, password).await
    }
}