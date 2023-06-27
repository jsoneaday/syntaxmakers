use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::DbRepo;
use crate::common::repository::jobs::models::{NewJob, Job};
use crate::common::repository::base::EntityId;

mod internal {
    use super::*;    

    pub async fn create_job(conn: &Pool<Postgres>, new_job: NewJob) -> Result<EntityId, Error> {
        let result = query_as::<_, EntityId>(
            r"
            insert into job (
                employer_id, title, description, is_remote, headquarters_country_id, primary_lang_id, secondary_lang_id, industry_id, salary_id
            ) values (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            ) returning id
            ")
            .bind(new_job.employer_id)
            .bind(new_job.title)
            .bind(new_job.description)
            .bind(new_job.is_remote)
            .bind(new_job.headquarters_country_id)
            .bind(new_job.primary_lang_id)
            .bind(new_job.secondary_lang_id)
            .bind(new_job.industry_id)
            .bind(new_job.salary_id)
            .fetch_one(conn)
            .await;

        match result {
            Ok(row) => Ok(row),
            Err(e) => {
                println!("create job error: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn get_job(conn: &Pool<Postgres>, id: i64) -> Result<Option<Job>, Error> {
        query_as::<_, Job>("select * from job where id = $1")
            .bind(id)
            .fetch_optional(conn).await
    }

    pub async fn get_all_jobs(conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
        query_as::<_, Job>(
            r"
            select * from job 
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
pub trait CreateJobFn {
    async fn create_job(&self, conn: &Pool<Postgres>, new_developer: NewJob) -> Result<EntityId, Error>;
}

#[async_trait]
impl CreateJobFn for DbRepo {
    async fn create_job(&self, conn: &Pool<Postgres>, new_developer: NewJob) -> Result<EntityId, Error> {
        internal::create_job(conn, new_developer).await
    }
}

#[async_trait]
pub trait GetJobFn {
    async fn get_job(&self, conn: &Pool<Postgres>, id: i64) -> Result<Option<Job>, Error>;
}

#[async_trait]
impl GetJobFn for DbRepo {
    async fn get_job(&self, conn: &Pool<Postgres>, id: i64) -> Result<Option<Job>, Error> {
        internal::get_job(conn, id).await
    }
}

#[async_trait]
pub trait GetAllJobsFn {
    async fn get_all_jobs(&self, conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error>;
}

#[async_trait]
impl GetAllJobsFn for DbRepo {
    async fn get_all_jobs(&self, conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
        internal::get_all_jobs(conn, page_size, last_offset).await
    }
}