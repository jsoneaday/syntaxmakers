use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as, query};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::jobs::models::{NewJob, Job, UpdateJob, JobApplied, JobCountry};
use crate::common::repository::base::EntityId;
use crate::common::repository::{error::SqlxError, developers::models::Developer, base::CountResult};
use log::error;

mod internal {    
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
            .bind(if let None = new_job.secondary_lang_id { None::<i64> } else { new_job.secondary_lang_id })
            .bind(new_job.industry_id)
            .bind(new_job.salary_id)
            .fetch_one(&mut *tx)
            .await;
        
        let inserted_entity = match insert_result {
            Ok(row) => Ok(row.clone()),
            Err(e) => {
                error!("insert job error: {:?}", e);
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
                let insert_jobs_countries = query_as::<_, EntityId>(
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

                let jobs_countries_result = match insert_jobs_countries {
                    Ok(row) => Ok(row),
                    Err(e) => Err(e)
                };
                if let Err(e) = jobs_countries_result {
                    return Err(e);
                }
            } else {
                return Err(sqlx::Error::Database(Box::new(SqlxError::IsRemoteContstraintError)));
            }
        }

        _ = tx.commit().await;  

        Ok(EntityId { id: job_id })
    }

    pub async fn update_job(conn: &Pool<Postgres>, update_job: UpdateJob) -> Result<(), Error> {
        let mut tx = conn.begin().await.unwrap();
        let job_id = update_job.id;
        let update_result = query::<_>(
            r"
            update job 
            set employer_id = $1, 
                title = $2, 
                description = $3, 
                is_remote = $4, 
                primary_lang_id = $5, 
                secondary_lang_id = $6, 
                industry_id = $7, 
                salary_id = $8
            where id = $9   
            ")
            .bind(update_job.employer_id)
            .bind(update_job.title)
            .bind(update_job.description)
            .bind(update_job.is_remote)
            .bind(update_job.primary_lang_id)
            .bind(update_job.secondary_lang_id)
            .bind(update_job.industry_id)
            .bind(update_job.salary_id)
            .bind(job_id)
            .execute(&mut *tx)
            .await;
        
        if let Err(e) = update_result {
            error!("update job error: {:?}", e);
            return Err(e);
        }
        
        if update_job.is_remote {
            if let Some(_) = update_job.country_id {
                return Err(sqlx::Error::Database(Box::new(SqlxError::IsRemoteContstraintError)));
            } else {
                let delete_job_country_result = query::<_>(
                    r"
                    delete from jobs_countries where job_id = $1
                    "
                )
                .bind(job_id)
                .execute(&mut *tx)
                .await;
                if let Err(e) = delete_job_country_result {
                    error!("update job error: {:?}", e);
                    return Err(e);
                }
            }
        } else {
            if let Some(country_id) = update_job.country_id {
                let existing_jobs_countries = query_as::<_, JobCountry>(
                    r"
                    select * from jobs_countries where job_id = $1
                    "
                )
                .bind(job_id)
                .fetch_all(&mut *tx)
                .await;

                if let Ok(_) = existing_jobs_countries {
                    let deleted_jobs_countries = query::<_>(
                        r"
                        delete from jobs_countries where job_id = $1
                        "
                    )
                    .bind(job_id)
                    .execute(&mut *tx)
                    .await;
                    if let Err(e) = deleted_jobs_countries {
                        error!("update job error: {:?}", e);
                        return Err(e);
                    }

                    let insert_jobs_countries = query_as::<_, EntityId>(
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
    
                    if let Err(e) = insert_jobs_countries {
                        error!("update job error: {:?}", e);
                        return Err(e);
                    }
                } else {
                    error!("update job error: could not find existing jobs_countries");
                    return Err(existing_jobs_countries.err().unwrap());
                }
            } else {
                error!("update job error: countryid not provided when remote is false");
                return Err(sqlx::Error::Database(Box::new(SqlxError::IsRemoteContstraintError)));
            }
        }

        _ = tx.commit().await;  

        Ok(())
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

    pub async fn query_all_jobs_count(conn: &Pool<Postgres>) -> Result<CountResult, Error> {
        query_as::<_, CountResult>(
            r"
            select count(*) as count
            from job 
            ")
        .fetch_one(conn).await
    }

    pub async fn query_jobs_by_employer(conn: &Pool<Postgres>, emp_id: i64, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
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
            where j.employer_id = $1
            order by updated_at desc             
            limit $2
            offset $3
            ")
            .bind(emp_id)
            .bind(page_size)
            .bind(last_offset)
            .fetch_all(conn).await
    }

    pub async fn query_jobs_by_search_terms(conn: &Pool<Postgres>, search_terms: Vec<String>, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
        let mut first_param: Vec<String> = vec![];
        for item in search_terms {
            first_param.push(format!(
                "%{}%",
                item
            ));
        }

        let jobs = query_as::<_, Job>(
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
            where j.title ILIKE ANY ($1)
                or ppl.name ILIKE ANY($1)
                or spl.name ILIKE ANY ($1)  
                or co.name ILIKE ANY ($1)  
                or cy.name ILIKE ANY ($1)  
                or i.name ILIKE ANY ($1)  
            order by updated_at desc             
            limit $2
            offset $3
            ")
            .bind(first_param)
            .bind(page_size)
            .bind(last_offset)
            .fetch_all(conn).await;
        jobs
    }

    pub async fn query_jobs_by_developer(conn: &Pool<Postgres>, dev_id: i64, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
        let developer_result = query_as::<_, Developer>(
            r"
            select d.id, d.created_at, d.updated_at, d.user_name, d.full_name, d.email, d.password, d.primary_lang_id, dsl.secondary_lang_id
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

        let jobs = query_as::<_, Job>(
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
            order by j.updated_at desc 
            limit $3
            offset $4
            ")
            .bind(developer.primary_lang_id)
            .bind(developer.secondary_lang_id)
            .bind(page_size)
            .bind(last_offset)
            .fetch_all(conn).await;
        
        match jobs {
            Ok(jobs) => {
                Ok(jobs)
            },
            Err(e) => Err(e)
        }
    }

    pub async fn query_jobs_by_applier(conn: &Pool<Postgres>, dev_id: i64, page_size: i32, last_offset: i64) -> Result<Vec<JobApplied>, Error> { 
        let jobs = query_as::<_, JobApplied>(
            r"
            select 
                j.id, 
                j.created_at, 
                j.updated_at, 
                a.created_at as dev_applied_at,
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
                    join application a on j.id = a.job_id
                    join developer d on a.developer_id = d.id
            where d.id = $1 
            order by a.created_at desc             
            limit $2
            offset $3
            ")
            .bind(dev_id)
            .bind(page_size)
            .bind(last_offset)
            .fetch_all(conn).await;
        jobs
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
pub trait UpdateJobFn {
    async fn update_job(&self, new_job: UpdateJob) -> Result<(), Error>;
}

#[async_trait]
impl UpdateJobFn for DbRepo {
    async fn update_job(&self, new_job: UpdateJob) -> Result<(), Error> {
        internal::update_job(self.get_conn(), new_job).await
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
pub trait QueryJobsCountFn {
    async fn query_all_jobs_count(&self) -> Result<CountResult, Error>;
}

#[async_trait]
impl QueryJobsCountFn for DbRepo {
    async fn query_all_jobs_count(&self) -> Result<CountResult, Error> {
        internal::query_all_jobs_count(self.get_conn()).await
    }
}

#[async_trait]
pub trait QueryJobsByEmployerFn {
    async fn query_jobs_by_employer(&self, emp_id: i64, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error>;
}

#[async_trait]
impl QueryJobsByEmployerFn for DbRepo {
    async fn query_jobs_by_employer(&self, emp_id: i64, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
        internal::query_jobs_by_employer(self.get_conn(), emp_id, page_size, last_offset).await
    }
}

#[async_trait]
pub trait QueryJobsBySearchTermsFn {
    async fn query_jobs_by_search_terms(&self, search_terms: Vec<String>, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error>;
}

#[async_trait]
impl QueryJobsBySearchTermsFn for DbRepo {
    async fn query_jobs_by_search_terms(&self, search_terms: Vec<String>, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
        internal::query_jobs_by_search_terms(self.get_conn(), search_terms, page_size, last_offset).await
    }
}

#[async_trait]
pub trait QueryJobsByDeveloperFn {
    async fn query_jobs_by_developer(&self, dev_id: i64, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error>;
}

#[async_trait]
impl QueryJobsByDeveloperFn for DbRepo {
    async fn query_jobs_by_developer(&self, dev_id: i64, page_size: i32, last_offset: i64) -> Result<Vec<Job>, Error> {
        internal::query_jobs_by_developer(self.get_conn(), dev_id, page_size, last_offset).await
    }
}

#[async_trait]
pub trait QueryJobsByApplierFn {
    async fn query_jobs_by_applier(&self, dev_id: i64, page_size: i32, last_offset: i64) -> Result<Vec<JobApplied>, Error>;
}

#[async_trait]
impl QueryJobsByApplierFn for DbRepo {
    async fn query_jobs_by_applier(&self, dev_id: i64, page_size: i32, last_offset: i64) -> Result<Vec<JobApplied>, Error> {
        internal::query_jobs_by_applier(self.get_conn(), dev_id, page_size, last_offset).await
    }
}