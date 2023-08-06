use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::jobs::models::{NewJob, Job};
use crate::common::repository::base::EntityId;

mod internal {
    use crate::common::repository::{error::SqlxError, developers::models::Developer};
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
                e.full_name as employer_name,
                co.id as company_id,
                co.name as company_name,
                co.logo as company_logo,
                j.title, 
                j.description, 
                j.is_remote, 
                jc.country_id,
                cy.name as country_name,
                j.primary_lang_id,
                ppl.name as primary_lang_name,
                j.secondary_lang_id,
                spl.name as secondary_lang_name,
                j.industry_id,
                i.name as industry_name,
                j.salary_id,
                s.base as salary
            from 
                job j 
                    join employer e on j.employer_id = e.id
                    join company co on e.company_id = co.id
                    left join jobs_countries jc on j.id = jc.job_id 
                    full outer join country cy on jc.country_id = cy.id
                    join prog_language ppl on j.primary_lang_id = ppl.id
                    join prog_language spl on j.secondary_lang_id = spl.id
                    join industry i on j.industry_id = i.id
                    join salary s on j.salary_id = s.id
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
                e.full_name as employer_name,
                co.id as company_id,
                co.name as company_name,
                co.logo as company_logo,
                j.title, 
                j.description, 
                j.is_remote, 
                jc.country_id,
                cy.name as country_name,
                j.primary_lang_id,
                ppl.name as primary_lang_name,
                j.secondary_lang_id,
                spl.name as secondary_lang_name,
                j.industry_id,
                i.name as industry_name,
                j.salary_id,
                s.base as salary
            from 
                job j 
                    join employer e on j.employer_id = e.id
                    join company co on e.company_id = co.id
                    left join jobs_countries jc on j.id = jc.job_id 
                    full outer join country cy on jc.country_id = cy.id
                    join prog_language ppl on j.primary_lang_id = ppl.id
                    join prog_language spl on j.secondary_lang_id = spl.id
                    join industry i on j.industry_id = i.id
                    join salary s on j.salary_id = s.id
            order by updated_at desc 
            limit $1
            offset $2
            ")
            .bind(page_size)
            .bind(last_offset)
            .fetch_all(conn).await
    }

    pub async fn query_jobs_by_search(conn: &Pool<Postgres>, search_str: String, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
        query_as::<_, Job>(
            r"
            select 
                j.id, 
                j.created_at, 
                j.updated_at, 
                j.employer_id, 
                e.full_name as employer_name,
                co.id as company_id,
                co.name as company_name,
                co.logo as company_logo,
                j.title, 
                j.description, 
                j.is_remote, 
                jc.country_id,
                cy.name as country_name,
                j.primary_lang_id,
                ppl.name as primary_lang_name,
                j.secondary_lang_id,
                spl.name as secondary_lang_name,
                j.industry_id,
                i.name as industry_name,
                j.salary_id,
                s.base as salary
            from 
                job j 
                    join employer e on j.employer_id = e.id
                    join company co on e.company_id = co.id
                    left join jobs_countries jc on j.id = jc.job_id 
                    full outer join country cy on jc.country_id = cy.id
                    join prog_language ppl on j.primary_lang_id = ppl.id
                    join prog_language spl on j.secondary_lang_id = spl.id
                    join industry i on j.industry_id = i.id
                    join salary s on j.salary_id = s.id
            order by updated_at desc 
            limit $1
            offset $2
            ")
            .bind(page_size)
            .bind(last_offset)
            .fetch_all(conn).await
    }

    pub async fn query_jobs_by_dev_profile(conn: &Pool<Postgres>, dev_id: i64, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
        let developer_result = query_as::<_, Developer>(
            r"
            select d.id, d.created_at, d.updated_at, d.user_name, d.full_name, d.email, d.primary_lang_id, dsl.secondary_lang_id
            from developer d left join developers_secondary_langs dsl on d.id = dsl.developer_id
            where d.id = $1
            "
        )
        .bind(dev_id)
        .fetch_one(conn)
        .await;

        #[allow(unused)]
        let mut developer: Option<Developer> = None;
        if let Ok(dev) = developer_result {
            developer = Some(dev);
        } else {
            return Err(developer_result.err().unwrap());
        }
        let developer = developer.unwrap();

        query_as::<_, Job>(
            r"
            select 
                j.id, 
                j.created_at, 
                j.updated_at, 
                j.employer_id, 
                e.full_name as employer_name,
                co.id as company_id,
                co.name as company_name,
                co.logo as company_logo,
                j.title, 
                j.description, 
                j.is_remote, 
                jc.country_id,
                cy.name as country_name,
                j.primary_lang_id,
                ppl.name as primary_lang_name,
                j.secondary_lang_id,
                spl.name as secondary_lang_name,
                j.industry_id,
                i.name as industry_name,
                j.salary_id,
                s.base as salary
            from 
                job j
                    join employer e on j.employer_id = e.id
                    join company co on e.company_id = co.id
                    left join jobs_countries jc on j.id = jc.job_id 
                    full outer join country cy on jc.country_id = cy.id
                    join prog_language ppl on j.primary_lang_id = ppl.id
                    join prog_language spl on j.secondary_lang_id = spl.id
                    join industry i on j.industry_id = i.id
                    join salary s on j.salary_id = s.id
            where j.primary_lang_id = $1 or j.secondary_lang_id = $2
            order by updated_at desc 
            limit $3
            offset $4
            ")
            .bind(developer.primary_lang_id)
            .bind(developer.secondary_lang_id)
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

#[async_trait]
pub trait QueryJobsByDevProfile {
    async fn query_jobs_by_dev_profile(&self, dev_id: i64, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error>;
}

#[async_trait]
impl QueryJobsByDevProfile for DbRepo {
    async fn query_jobs_by_dev_profile(&self, dev_id: i64, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
        internal::query_jobs_by_dev_profile(self.get_conn(), dev_id, page_size, last_offset).await
    }
}