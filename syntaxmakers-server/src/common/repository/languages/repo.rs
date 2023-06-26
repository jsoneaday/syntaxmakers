use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::DbRepo;
use crate::common::repository::languages::models::Language;

mod internal {
    use super::*;    

    pub async fn get_all_languages(conn: &Pool<Postgres>) -> Result<Vec<Language>, Error> {
        query_as::<_, Language>("select * from prog_language").fetch_all(conn).await
    }
}

#[async_trait]
pub trait GetAllLanguagesFn {
    async fn get_all_languages(&self, conn: &Pool<Postgres>) -> Result<Vec<Language>, Error>;
}

#[async_trait]
impl GetAllLanguagesFn for DbRepo {
    async fn get_all_languages(&self, conn: &Pool<Postgres>) -> Result<Vec<Language>, Error> {
        internal::get_all_languages(conn).await
    }
}