use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::salaries::models::Salary;

mod internal {
    use super::*;    

    pub async fn query_all_salaries(conn: &Pool<Postgres>) -> Result<Vec<Salary>, Error> {
        query_as::<_, Salary>("select * from salary").fetch_all(conn).await
    }
}

#[async_trait]
pub trait QueryAllSalariesFn {
    async fn query_all_salaries(&self) -> Result<Vec<Salary>, Error>;
}

#[async_trait]
impl QueryAllSalariesFn for DbRepo {
    async fn query_all_salaries(&self) -> Result<Vec<Salary>, Error> {
        internal::query_all_salaries(self.get_conn()).await
    }
}