use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::DbRepo;
use crate::common::repository::countries::models::Country;

mod internal {
    use super::*;    

    pub async fn get_all_countries(conn: &Pool<Postgres>) -> Result<Vec<Country>, Error> {
        query_as::<_, Country>("select * from country").fetch_all(conn).await
    }
}

#[async_trait]
pub trait GetAllCountriesFn {
    async fn get_all_countrie(&self, conn: &Pool<Postgres>) -> Result<Vec<Country>, Error>;
}

#[async_trait]
impl GetAllCountriesFn for DbRepo {
    async fn get_all_countrie(&self, conn: &Pool<Postgres>) -> Result<Vec<Country>, Error> {
        internal::get_all_countries(conn).await
    }
}