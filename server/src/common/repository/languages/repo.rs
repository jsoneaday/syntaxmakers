use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::languages::models::Language;

mod internal {
    use super::*;    

    pub async fn query_all_languages(conn: &Pool<Postgres>) -> Result<Vec<Language>, Error> {
        query_as::<_, Language>("select * from prog_language order by name").fetch_all(conn).await
    }
}

#[async_trait]
pub trait QueryAllLanguagesFn {
    async fn query_all_languages(&self) -> Result<Vec<Language>, Error>;
}

#[async_trait]
impl QueryAllLanguagesFn for DbRepo {
    async fn query_all_languages(&self) -> Result<Vec<Language>, Error> {
        internal::query_all_languages(self.get_conn()).await
    }
}