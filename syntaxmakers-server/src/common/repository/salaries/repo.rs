use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::DbRepo;
use crate::common::repository::salaries::models::Salary;

mod internal {
    use super::*;    

    pub async fn get_all_salaries(conn: &Pool<Postgres>) -> Result<Vec<Salary>, Error> {
        query_as::<_, Salary>("select * from salary").fetch_all(conn).await
    }
}

#[async_trait]
pub trait GetAllSalariesFn {
    async fn get_all_salaries(&self, conn: &Pool<Postgres>) -> Result<Vec<Salary>, Error>;
}

#[async_trait]
impl GetAllSalariesFn for DbRepo {
    async fn get_all_salaries(&self, conn: &Pool<Postgres>) -> Result<Vec<Salary>, Error> {
        internal::get_all_salaries(conn).await
    }
}