use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::countries::models::Country;

mod internal {
    use super::*;    

    pub async fn query_all_countries(conn: &Pool<Postgres>) -> Result<Vec<Country>, Error> {
        query_as::<_, Country>("select * from country").fetch_all(conn).await
    }
}

#[async_trait]
pub trait QueryAllCountriesFn {
    async fn query_all_countries(&self) -> Result<Vec<Country>, Error>;
}

#[async_trait]
impl QueryAllCountriesFn for DbRepo {
    async fn query_all_countries(&self) -> Result<Vec<Country>, Error> {
        internal::query_all_countries(self.get_conn()).await
    }
}