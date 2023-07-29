use actix_web::web::{Data, Json, Path};
use crate::{common::repository::{jobs::{repo::{InsertJobFn, QueryJobFn, QueryAllJobsFn}, models::NewJob}, base::Repository}, app_state::AppState, routes::{base_model::{OutputId, PagingModel}, user_error::UserError}};
use super::models::{NewJobForRoute, JobResponders, JobResponder};

#[allow(unused)]
async fn create_job<T: InsertJobFn + Repository>(app_data: Data<AppState<T>>, json: Json<NewJobForRoute>) -> Result<OutputId, UserError> {
    let result = app_data.repo.insert_job(NewJob {
        employer_id: json.employer_id,
        title: json.title.to_owned(),
        description: json.description.to_owned(),
        is_remote: json.is_remote,
        headquarters_country_id: json.headquarters_country_id,
        primary_lang_id: json.primary_lang_id,
        secondary_lang_id: json.secondary_lang_id,
        industry_id: json.industry_id,
        salary_id: json.salary_id
    }).await;

    match result {
        Ok(entity) => Ok(OutputId { id: entity.id }),
        Err(e) => Err(e.into())
    }
}

#[allow(unused)]
async fn get_job<T: QueryJobFn + Repository>(app_data: Data<AppState<T>>, path: Path<i64>) -> Result<Option<JobResponder>, UserError> {
    let result = app_data.repo.query_job(path.into_inner()).await;
    
    match result {
        Ok(opt_job) => match opt_job {
            Some(job) => Ok(Some(JobResponder { 
                id: job.id, 
                updated_at: job.updated_at, 
                employer_id: job.employer_id, 
                title: job.title, 
                description: job.description, 
                is_remote: job.is_remote, 
                headquarters_country_id: job.headquarters_country_id, 
                primary_lang_id: job.primary_lang_id, 
                secondary_lang_id: job.secondary_lang_id, 
                industry_id: job.industry_id, 
                salary_id: job.salary_id 
            })),
            None => Ok(None)
        },
        Err(e) => Err(e.into())
    }
}

#[allow(unused)]
async fn get_all_jobs<T: QueryAllJobsFn + Repository>(app_data: Data<AppState<T>>, json: Json<PagingModel>) -> Result<JobResponders, UserError> {
    let result = app_data.repo.query_all_jobs(json.page_size, json.last_offset).await;
    
    match result {
        Ok(jobs) => {
            let responders = jobs.iter().map(|job| {
                JobResponder { 
                    id: job.id, 
                    updated_at: job.updated_at, 
                    employer_id: job.employer_id, 
                    title: job.title.to_owned(), 
                    description: job.description.to_owned(), 
                    is_remote: job.is_remote, 
                    headquarters_country_id: job.headquarters_country_id, 
                    primary_lang_id: job.primary_lang_id, 
                    secondary_lang_id: job.secondary_lang_id, 
                    industry_id: job.industry_id, 
                    salary_id: job.salary_id 
                }
            })
            .collect::<Vec<JobResponder>>();
            Ok(JobResponders(responders))
        },
        Err(e) => Err(e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::common::repository::jobs::models::Job;
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use crate::{common::repository::{jobs::repo::InsertJobFn, base::EntityId}, common_test::fixtures::{MockDbRepo, get_app_data, get_fake_title, get_fake_desc}};

    #[async_trait]
    impl InsertJobFn for MockDbRepo {
        async fn insert_job(&self, _: NewJob) -> Result<EntityId, sqlx::Error> {
            Ok(EntityId { id: 1 })
        }
    }

    #[async_trait]
    impl QueryJobFn for MockDbRepo {
        async fn query_job(&self, _: i64) -> Result<Option<Job>, sqlx::Error> {
            Ok(Some(
                Job { 
                    id: 1, 
                    created_at: Utc::now(), 
                    updated_at: Utc::now(), 
                    employer_id: 1, 
                    title: get_fake_title(), 
                    description: get_fake_desc(), 
                    is_remote: true, 
                    headquarters_country_id: 1, 
                    primary_lang_id: 1, 
                    secondary_lang_id: 2, 
                    industry_id: 1, 
                    salary_id: 1 
                }
            ))
        }
    }

    #[async_trait]
    impl QueryAllJobsFn for MockDbRepo {
        async fn query_all_jobs(&self, _: i32, _: i64) -> Result<Vec<Job>, sqlx::Error> {
            Ok(vec![
                Job { 
                    id: 1, 
                    created_at: Utc::now(), 
                    updated_at: Utc::now(), 
                    employer_id: 1, 
                    title: get_fake_title(), 
                    description: get_fake_desc(), 
                    is_remote: true, 
                    headquarters_country_id: 1, 
                    primary_lang_id: 1, 
                    secondary_lang_id: 2, 
                    industry_id: 1, 
                    salary_id: 1 
                }
            ])
        }
    }

    #[tokio::test]
    async fn test_create_job_route() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let result = create_job(app_data, Json(NewJobForRoute {
            employer_id: 1,
            title: get_fake_title(),
            description: get_fake_desc(),
            is_remote: false,
            headquarters_country_id: Some(1),
            primary_lang_id: 1,
            secondary_lang_id: Some(2),
            industry_id: 1,
            salary_id: 1
        })).await.unwrap();

        assert!(result.id == 1);
    }

    #[tokio::test]
    async fn test_get_job_route() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let result = get_job(app_data, Path::from(1)).await.unwrap();

        assert!(result.unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_all_jobs_route() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let result = get_all_jobs(app_data, Json(PagingModel { page_size: 10, last_offset: 1 })).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }
}