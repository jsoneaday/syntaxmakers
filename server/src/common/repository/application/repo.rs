use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::{ConnGetter, DbRepo, EntityId};
use super::models::NewApplication;


mod internal {
    use super::*;

    pub async fn insert_application(conn: &Pool<Postgres>, new_application: NewApplication) -> Result<EntityId, Error> {
        query_as::<_, EntityId> (
            r"
                insert into application(
                    job_id, developer_id
                ) values (
                    $1, $2
                ) returning id
            "
        )
        .bind(new_application.job_id)
        .bind(new_application.developer_id)
        .fetch_one(conn)
        .await
    }
}

#[async_trait]
pub trait InsertApplicationFn {
    async fn insert_application(&self, new_application: NewApplication) -> Result<EntityId, Error>;
}

#[async_trait]
impl InsertApplicationFn for DbRepo  {
    async fn insert_application(&self, new_application: NewApplication) -> Result<EntityId, Error> {
        internal::insert_application(self.get_conn(), new_application).await
    }
}