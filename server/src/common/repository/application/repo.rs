use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::{ConnGetter, DbRepo, EntityId};
use super::models::NewApplication;


mod internal {
    use crate::common::repository::error::SqlxError;

    use super::*;

    pub async fn insert_application(conn: &Pool<Postgres>, new_application: NewApplication) -> Result<EntityId, Error> {        
        let applied = query_as::<_, EntityId>(r"
            select a.id
            from job j join application a on j.id = a.job_id
            where a.job_id = $1 and a.developer_id = $2
        ")
        .bind(new_application.job_id)
        .bind(new_application.developer_id)
        .fetch_optional(conn)
        .await;

        match applied {
            Ok(entity) => {
                if let None = entity {
                    return query_as::<_, EntityId> (
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
                    .await;
                }
            },
            Err(e) => {
                return Err(e);
            }
        }
    
        Err(SqlxError::DatabaseQueryFailed.into())
    }

    pub async fn dev_has_applied(conn: &Pool<Postgres>, job_id: i64, dev_id: i64) -> Result<bool, Error> {
        let result = query_as::<_, EntityId>(r"
            select a.id
            from job j join application a on j.id = a.job_id
            where a.job_id = $1 and a.developer_id = $2
        ")
        .bind(job_id)
        .bind(dev_id)
        .fetch_optional(conn)
        .await;

        match result {
            Ok(has_applied) => {
                if let None = has_applied {
                    Ok(false)
                } else {
                    Ok(true)
                }                
            },
            Err(e) => {
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