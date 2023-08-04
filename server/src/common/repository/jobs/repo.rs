use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::jobs::models::{NewJob, Job};
use crate::common::repository::base::EntityId;

mod internal {
    use crate::common::repository::error::SqlxError;
    use super::*;    

    pub async fn insert_job(conn: &Pool<Postgres>, new_job: NewJob) -> Result<EntityId, Error> {
        let mut tx = conn.begin().await.unwrap();

        let insert_result = query_as::<_, EntityId>(
            r"
            insert into job (
                employer_id, title, description, is_remote, primary_lang_id, secondary_lang_id, industry_id, salary_id
            ) values (
                $1, $2, $3, $4, $5, $6, $7, $8
            ) returning id
            ")
            .bind(new_job.employer_id)
            .bind(new_job.title)
            .bind(new_job.description)
            .bind(new_job.is_remote)
            .bind(new_job.primary_lang_id)
            .bind(new_job.secondary_lang_id)
            .bind(new_job.industry_id)
            .bind(new_job.salary_id)
            .fetch_one(&mut *tx)
            .await;
        
        let inserted_entity = match insert_result {
            Ok(row) => Ok(row.clone()),
            Err(e) => {
                println!("insert job error: {:?}", e);
                Err(e)
            }
        };
        if let Err(e) = inserted_entity {
            return Err(e);
        }

        let job_id = inserted_entity.unwrap().id;
        if new_job.is_remote {
            if let Some(_) = new_job.country_id {
                return Err(sqlx::Error::Database(Box::new(SqlxError::IsRemoteContstraintError)));
            }
        } else {
            if let Some(country_id) = new_job.country_id {
                _ = query_as::<_, EntityId>(
                    r"
                    insert into jobs_countries (
                        job_id, country_id
                    ) values (
                        $1, $2
                    ) returning id
                    ")
                .bind(job_id)
                .bind(country_id)
                .fetch_one(&mut *tx)
                .await;
            } else {
                return Err(sqlx::Error::Database(Box::new(SqlxError::IsRemoteContstraintError)));
            }
        }

        _ = tx.commit().await;  

        Ok(EntityId { id: job_id })
    }

    pub async fn query_job(conn: &Pool<Postgres>, id: i64) -> Result<Option<Job>, Error> {
        query_as::<_, Job>(
            r"
            select 
                j.id, 
                j.created_at, 
                j.updated_at, 
                j.employer_id, 
                j.title, 
                j.description, 
                j.is_remote, 
                jc.country_id,
                j.primary_lang_id,
                j.secondary_lang_id,
                j.industry_id,
                j.salary_id
            from job j left join jobs_countries jc on j.id = jc.job_id 
            where j.id = $1
            "
        )
        .bind(id)
        .fetch_optional(conn).await
    }

    pub async fn query_all_jobs(conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
        query_as::<_, Job>(
            r"
            select 
                j.id, 
                j.created_at, 
                j.updated_at, 
                j.employer_id, 
                j.title, 
                j.description, 
                j.is_remote, 
                jc.country_id,
                j.primary_lang_id,
                j.secondary_lang_id,
                j.industry_id,
                j.salary_id
            from job j left join jobs_countries jc on j.id = jc.job_id 
            order by updated_at desc 
            limit $1
            offset $2
            ")
            .bind(page_size)
            .bind(last_offset)
            .fetch_all(conn).await
    }
}

#[async_trait]
pub trait InsertJobFn {
    async fn insert_job(&self, new_job: NewJob) -> Result<EntityId, Error>;
}

#[async_trait]
impl InsertJobFn for DbRepo {
    async fn insert_job(&self, new_job: NewJob) -> Result<EntityId, Error> {
        internal::insert_job(self.get_conn(), new_job).await
    }
}

#[async_trait]
pub trait QueryJobFn {
    async fn query_job(&self, id: i64) -> Result<Option<Job>, Error>;
}

#[async_trait]
impl QueryJobFn for DbRepo {
    async fn query_job(&self, id: i64) -> Result<Option<Job>, Error> {
        internal::query_job(self.get_conn(), id).await
    }
}

#[async_trait]
pub trait QueryAllJobsFn {
    async fn query_all_jobs(&self, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error>;
}

#[async_trait]
impl QueryAllJobsFn for DbRepo {
    async fn query_all_jobs(&self, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
        internal::query_all_jobs(self.get_conn(), page_size, last_offset).await
    }
}