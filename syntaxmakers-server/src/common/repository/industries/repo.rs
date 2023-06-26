use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::DbRepo;
use crate::common::repository::industries::models::Industry;

mod internal { 
    use super::*;    

    pub async fn get_all_industries(conn: &Pool<Postgres>) -> Result<Vec<Industry>, Error> {
        query_as::<_, Industry>("select * from industry").fetch_all(conn).await
    }
}

#[async_trait]
pub trait GetAllIndustriesFn {
    async fn get_all_industries(&self, conn: &Pool<Postgres>) -> Result<Vec<Industry>, Error>;
}

#[async_trait]
impl GetAllIndustriesFn for DbRepo {
    async fn get_all_industries(&self, conn: &Pool<Postgres>) -> Result<Vec<Industry>, Error> {
        internal::get_all_industries(conn).await
    }
}