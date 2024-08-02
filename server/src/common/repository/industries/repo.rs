use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::industries::models::Industry;

mod internal { 
    use super::*;    

    pub async fn query_all_industries(conn: &Pool<Postgres>) -> Result<Vec<Industry>, Error> {
        query_as::<_, Industry>("select * from industry order by name").fetch_all(conn).await
    }
}

#[async_trait]
pub trait QueryAllIndustriesFn {
    async fn query_all_industries(&self) -> Result<Vec<Industry>, Error>;
}

#[async_trait]
impl QueryAllIndustriesFn for DbRepo {
    async fn query_all_industries(&self) -> Result<Vec<Industry>, Error> {
        internal::query_all_industries(self.get_conn()).await
    }
}