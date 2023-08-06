use actix_web::web::{Data, Json, Path};
use crate::{common::repository::{jobs::{repo::{InsertJobFn, QueryJobFn, QueryAllJobsFn, QueryJobsByDevProfile}, models::{NewJob, Job}}, base::Repository}, app_state::AppState, routes::{base_model::{OutputId, PagingModel, IdAndPagingModel}, user_error::UserError}};
use super::models::{NewJobForRoute, JobResponders, JobResponder};

#[allow(unused)]
pub async fn create_job<T: InsertJobFn + Repository>(app_data: Data<AppState<T>>, json: Json<NewJobForRoute>)
 -> Result<OutputId, UserError> {
    let result = app_data.repo.insert_job(NewJob {
        employer_id: json.employer_id,
        title: json.title.to_owned(),
        description: json.description.to_owned(),
        is_remote: json.is_remote,
        country_id: json.country_id,
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
pub async fn get_job<T: QueryJobFn + Repository>(app_data: Data<AppState<T>>, path: Path<i64>) -> Result<Option<JobResponder>, UserError> {
    let result = app_data.repo.query_job(path.into_inner()).await;
    
    match result {
        Ok(opt_job) => match opt_job {
            Some(job) => Ok(Some(convert(&job))),
            None => Ok(None)
        },
        Err(e) => Err(e.into())
    }
}

#[allow(unused)]
pub async fn get_all_jobs<T: QueryAllJobsFn + Repository>(app_data: Data<AppState<T>>, json: Json<PagingModel>) -> Result<JobResponders, UserError> {
    let result = app_data.repo.query_all_jobs(json.page_size, json.last_offset).await;
    
    match result {
        Ok(jobs) => {
            let responders = jobs.iter().map(|job| {
                convert(job)
            })
            .collect::<Vec<JobResponder>>();
            Ok(JobResponders(responders))
        },
        Err(e) => Err(e.into())
    }
}

#[allow(unused)]
pub async fn get_jobs_by_dev_profile<T: QueryJobsByDevProfile + Repository>(app_data: Data<AppState<T>>, json: Json<IdAndPagingModel>) -> Result<JobResponders, UserError> {
    let result = app_data.repo.query_jobs_by_dev_profile(json.id, json.page_size, json.last_offset).await;
    
    match result {
        Ok(jobs) => {
            let responders = jobs.iter().map(|job| {
                convert(job)
            })
            .collect::<Vec<JobResponder>>();
            Ok(JobResponders(responders))
        },
        Err(e) => Err(e.into())
    }
}

fn convert(job: &Job) -> JobResponder {
    JobResponder {
        id: job.id, 
        updated_at: job.updated_at, 
        employer_id: job.employer_id, 
        employer_name: job.employer_name.to_string(),
        company_id: job.company_id,
        company_name: job.company_name.to_string(),
        company_logo: job.company_logo.clone(),
        title: job.title.to_string(), 
        description: job.description.to_string(), 
        is_remote: job.is_remote, 
        country_id: job.country_id, 
        country_name: if let Some(country_name) = job.country_name.to_owned() {
            Some(country_name)
        } else {
            None
        },
        primary_lang_id: job.primary_lang_id, 
        primary_lang_name: job.primary_lang_name.to_string(),
        secondary_lang_id: job.secondary_lang_id,
        secondary_lang_name: job.secondary_lang_name.to_string(), 
        industry_id: job.industry_id, 
        industry_name: job.industry_name.to_string(),
        salary_id: job.salary_id,
        salary: job.salary
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::repository::jobs::models::Job, common_test::fixtures::{get_fake_fullname, init_fixtures, COUNTRY_NAMES, LANGUAGE_NAMES, INDUSTRY_NAMES, SALARY_BASE}};
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use fake::{faker::company::en::CompanyName, Fake};
    use crate::{common::repository::{jobs::repo::InsertJobFn, base::EntityId}, common_test::fixtures::{MockDbRepo, get_app_data, get_fake_title, get_fake_desc}};

    fn get_test_job(id: i64) -> Job {
        init_fixtures();
        Job { 
            id, 
            created_at: Utc::now(), 
            updated_at: Utc::now(), 
            employer_id: id, 
            employer_name: get_fake_fullname(),
            company_id: id,
            company_name: CompanyName().fake::<String>(),
            company_logo: None,
            title: get_fake_title(), 
            description: get_fake_desc(), 
            is_remote: true, 
            country_id: None, 
            country_name: Some(COUNTRY_NAMES.get().unwrap().get(0).unwrap().to_string()),
            primary_lang_id: id, 
            primary_lang_name: LANGUAGE_NAMES.get().unwrap().get(0).unwrap().to_string(),
            secondary_lang_id: id + 1, 
            secondary_lang_name: LANGUAGE_NAMES.get().unwrap().get(1).unwrap().to_string(),
            industry_id: id, 
            industry_name: INDUSTRY_NAMES.get().unwrap().get(0).unwrap().to_string(),
            salary_id: id,
            salary: SALARY_BASE.get().unwrap().get(0).unwrap().to_string().parse::<i32>().unwrap()
        }
    }

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
                get_test_job(1)
            ))
        }
    }

    #[async_trait]
    impl QueryAllJobsFn for MockDbRepo {
        async fn query_all_jobs(&self, _: i32, _: i64) -> Result<Vec<Job>, sqlx::Error> {
            Ok(vec![
                get_test_job(1)
            ])
        }
    }

    #[async_trait]
    impl QueryJobsByDevProfile for MockDbRepo {
        async fn query_jobs_by_dev_profile(&self, _: i64, _: i32, _: i64) -> Result<Vec<Job>, sqlx::Error> {
            Ok(vec![
                get_test_job(1)
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
            country_id: Some(1),
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

    #[tokio::test]
    async fn test_get_jobs_by_dev_profile() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let result = get_jobs_by_dev_profile(app_data, Json(IdAndPagingModel { id: 1, page_size: 10, last_offset: 1 })).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }
}