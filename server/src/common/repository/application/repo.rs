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

    pub async fn dev_has_applied(conn: &Pool<Postgres>, job_id: i64, dev_id: i64) -> Result<bool, Error> {
        let result = query_as::<_, EntityId>(r"
            select a.id
            from job j join application a on j.id = a.job_id
            where a.job_id = $1 and a.developer_id = $2
        ")
        .bind(job_id)
        .bind(dev_id)
        .fetch_one(conn)
        .await;

        match result {
            Ok(entity) => {
                println!("user {} already applied to job {}", dev_id, job_id);
                Ok(true)
            },
            Err(e) => {
                println!("error {:?}, user {} has not applied to {}", e, dev_id, job_id);
                Err(e)
            }
        }
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

#[async_trait]
pub trait DevHasAppliedFn {
    async fn dev_has_applied(&self, job_id: i64, dev_id: i64) -> Result<bool, Error>;
}

#[async_trait]
impl DevHasAppliedFn for DbRepo  {
    async fn dev_has_applied(&self, job_id: i64, dev_id: i64) -> Result<bool, Error> {
        internal::dev_has_applied(self.get_conn(), job_id, dev_id).await
    }
}