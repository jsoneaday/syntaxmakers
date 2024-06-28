use actix_web::{web::{Data, Json, Path}, HttpResponse, HttpRequest};
use log::{error, info};
use crate::{
    app_state::AppState, common::{
        authentication::auth_service::Authenticator, repository::{
            base::Repository, developers::repo::QueryDeveloperFn, employers::repo::QueryEmployerFn, jobs::{models::{Job, JobApplied, NewJob, UpdateJob}, repo::{InsertJobFn, QueryAllJobsFn, QueryJobFn, QueryJobsByApplierFn, QueryJobsByDeveloperFn, QueryJobsByEmployerFn, QueryJobsBySearchTermsFn, UpdateJobFn}}
        }
    }, routes::{
        auth_helper::check_is_authenticated, base_model::{IdAndPagingModel, OutputId, PagingModel, SearchAndPagingModel}, user_error::UserError
    }
};
use super::models::{JobAppliedResponders, JobAppliedResponder, JobResponder, JobResponders, NewJobForRoute, UpdateJobForRoute};
use crate::routes::authentication::models::DeveloperOrEmployer as AuthDeveloperOrEmployer;

#[allow(unused)]
pub async fn create_job<T: InsertJobFn + QueryEmployerFn + QueryDeveloperFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<NewJobForRoute>, req: HttpRequest)
 -> Result<OutputId, UserError> {    
    info!("start insert_job {}", json.description);
    let is_auth = check_is_authenticated(app_data.clone(), json.employer_id, AuthDeveloperOrEmployer::Employer, req).await;
    if !is_auth {
        error!("Authorization failed");
        return Err(UserError::AuthenticationFailed);
    }

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
pub async fn update_job<T: UpdateJobFn + QueryEmployerFn + QueryDeveloperFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<UpdateJobForRoute>, req: HttpRequest)
 -> HttpResponse {
    info!("start update_job {}", json.description);
    let is_auth = check_is_authenticated(app_data.clone(), json.employer_id, AuthDeveloperOrEmployer::Employer, req).await;
    if !is_auth {
        error!("Authorization failed");
        return HttpResponse::Unauthorized().body("Request was not authenticated");
    }
    let result = app_data.repo.update_job(UpdateJob {
        id: json.id,
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
        Ok(entity) => HttpResponse::NoContent().into(),
        Err(e) => HttpResponse::InternalServerError().body("Failed to update job")
    }
}

#[allow(unused)]
pub async fn get_job<T: QueryJobFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, path: Path<i64>) -> Result<Option<JobResponder>, UserError> {
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
pub async fn get_all_jobs<T: QueryAllJobsFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<PagingModel>) -> Result<JobResponders, UserError> {
    let result = app_data.repo.query_all_jobs(json.page_size, json.last_offset).await;
    
    return_jobs_result(result)
}

#[allow(unused)]
pub async fn get_jobs_by_developer<T: QueryJobsByDeveloperFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<IdAndPagingModel>) -> Result<JobResponders, UserError> {
    let result = app_data.repo.query_jobs_by_developer(json.id, json.page_size, json.last_offset).await;
    // remove unneeded match
    match result {
        Ok(jobs) => {
            println!("jobs before convert: {:?}", jobs.clone().iter().map(|job| { job.title.clone() }).collect::<Vec<String>>());
            return_jobs_result(Ok(jobs))
        },
        Err(e) => Err(e.into())
    }    
}

#[allow(unused)]
pub async fn get_jobs_by_employer<T: QueryJobsByEmployerFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<IdAndPagingModel>) 
    -> Result<JobResponders, UserError> {
    let result = app_data.repo.query_jobs_by_employer(json.id, json.page_size, json.last_offset).await;
    
    return_jobs_result(result)
}

#[allow(unused)]
pub async fn get_jobs_by_search_terms<T: QueryJobsBySearchTermsFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<SearchAndPagingModel>) 
    -> Result<JobResponders, UserError> {
    let result = app_data.repo.query_jobs_by_search_terms(json.search_terms.clone(), json.page_size, json.last_offset).await;
    
    return_jobs_result(result)
}

#[allow(unused)]
pub async fn get_jobs_by_applier<T: QueryJobsByApplierFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<IdAndPagingModel>) -> Result<JobAppliedResponders, UserError> {
    let result = app_data.repo.query_jobs_by_applier(json.id, json.page_size, json.last_offset).await;
    // remove unneeded match
    match result {
        Ok(jobs) => {
            println!("jobs before convert: {:?}", jobs.clone().iter().map(|job| { job.title.clone() }).collect::<Vec<String>>());
            return_job_applied_result(Ok(jobs))
        },
        Err(e) => Err(e.into())
    }    
}

fn return_jobs_result(result: Result<Vec<Job>, sqlx::error::Error>) -> Result<JobResponders, UserError> {
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

fn return_job_applied_result(result: Result<Vec<JobApplied>, sqlx::error::Error>) -> Result<JobAppliedResponders, UserError> {
    match result {
        Ok(jobs) => {
            let responders = jobs.iter().map(|job| {
                convert_job_applied(job)
            })
            .collect::<Vec<JobAppliedResponder>>();
            Ok(JobAppliedResponders(responders))
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
        secondary_lang_name: job.secondary_lang_name.clone(), 
        industry_id: job.industry_id, 
        industry_name: job.industry_name.to_string(),
        salary_id: job.salary_id,
        salary: job.salary
    }
}

fn convert_job_applied(job: &JobApplied) -> JobAppliedResponder {
    JobAppliedResponder {
        id: job.id, 
        updated_at: job.updated_at, 
        dev_applied_at: job.dev_applied_at,
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
        secondary_lang_name: job.secondary_lang_name.clone(), 
        industry_id: job.industry_id, 
        industry_name: job.industry_name.to_string(),
        salary_id: job.salary_id,
        salary: job.salary
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{
            repository::{
                jobs::models::Job, 
                developers::models::Developer, 
                employers::models::Employer, 
                user::{repo::AuthenticateDbFn, models::AuthenticateResult},
                user::models::DeveloperOrEmployer as UserDeveloperOrEmployer
            }, 
            authentication::auth_service::AuthenticationError            
        }, 
        common_test::fixtures::{get_fake_fullname, init_fixtures, COUNTRIES, LANGUAGES, INDUSTRIES, SALARY_BASE}, routes::authentication::models::LoginCredential,
        common_test::fixtures::get_fake_httprequest_with_bearer_token,
        routes::authentication::routes::login
    };
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use fake::{faker::{company::en::CompanyName, internet::en::FreeEmail}, Fake};
    use jsonwebtoken::DecodingKey;
    use crate::{
        common::repository::{jobs::repo::InsertJobFn, base::EntityId}, common_test::fixtures::{get_app_data, get_fake_title, get_fake_desc}
    };

    const DEV_USERNAME: &str = "tester";
    const EMP_USERNAME: &str = "employer";

    struct MockDbRepo;
    struct MockAuthService;
    #[async_trait]
    impl Authenticator for MockAuthService {
        async fn is_authenticated(&self, _: String, _: Vec<(&str, &str)>, _: &DecodingKey) -> Result<bool, AuthenticationError> {
            Ok(true)
        }
    }

    async fn get_test_job(id: i64) -> Job {
        init_fixtures().await;
        Job { 
            id, 
            created_at: Utc::now(), 
            updated_at: Utc::now(), 
            employer_id: id, 
            employer_name: get_fake_fullname(),
            company_id: id,
            company_name: CompanyName().fake::<String>(),
            company_logo: None,
            title: get_fake_title().to_string(), 
            description: get_fake_desc().to_string(), 
            is_remote: true, 
            country_id: None, 
            country_name: Some(COUNTRIES.get().unwrap().get(0).unwrap().name.clone()),
            primary_lang_id: id, 
            primary_lang_name: LANGUAGES.get().unwrap().get(0).unwrap().name.clone(),
            secondary_lang_id: Some(id + 1), 
            secondary_lang_name: Some(LANGUAGES.get().unwrap().get(0).unwrap().name.clone()),
            industry_id: id, 
            industry_name: INDUSTRIES.get().unwrap().get(0).unwrap().name.clone(),
            salary_id: id,
            salary: SALARY_BASE.get().unwrap().get(0).unwrap().base
        }
    }

    #[async_trait]
    impl Repository for MockDbRepo {
        async fn init() -> Self {
            MockDbRepo
        }
    }

    #[async_trait]
    impl AuthenticateDbFn for MockDbRepo {
        async fn authenticate_db(&self, _: UserDeveloperOrEmployer, _: String, _: String) -> Result<AuthenticateResult, sqlx::Error> {
            Ok(AuthenticateResult::Success{ id: 1 })
        }
    }

    #[async_trait]
    impl QueryDeveloperFn for MockDbRepo {
        async fn query_developer(&self, _: i64) -> Result<Option<Developer>, sqlx::Error> {
            Ok(Some(Developer {
                id: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                user_name: DEV_USERNAME.to_string(),
                full_name: "Tester Test".to_string(),
                email: FreeEmail().fake::<String>(),
                primary_lang_id: 1,
                secondary_lang_id: None
            }))
        }
    }    

    #[async_trait]
    impl QueryEmployerFn for MockDbRepo {
        async fn query_employer(&self, _: i64) -> Result<Option<Employer>, sqlx::Error> {
            Ok(Some(Employer {
                id: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                user_name: EMP_USERNAME.to_string(),
                full_name: "Tester Test".to_string(),
                email: FreeEmail().fake::<String>(),
                company_id: 1
            }))
        }
    }    

    #[async_trait]
    impl InsertJobFn for MockDbRepo {
        async fn insert_job(&self, _: NewJob) -> Result<EntityId, sqlx::Error> {
            Ok(EntityId { id: 1 })
        }
    }

    #[async_trait]
    impl UpdateJobFn for MockDbRepo {
        async fn update_job(&self, _: UpdateJob) -> Result<(), sqlx::Error> {
            Ok(())
        }
    }

    #[async_trait]
    impl QueryJobFn for MockDbRepo {
        async fn query_job(&self, _: i64) -> Result<Option<Job>, sqlx::Error> {
            Ok(Some(
                get_test_job(1).await
            ))
        }
    }

    #[async_trait]
    impl QueryAllJobsFn for MockDbRepo {
        async fn query_all_jobs(&self, _: i32, _: i64) -> Result<Vec<Job>, sqlx::Error> {
            Ok(vec![
                get_test_job(1).await
            ])
        }
    }

    #[async_trait]
    impl QueryJobsByEmployerFn for MockDbRepo {
        async fn query_jobs_by_employer(&self, _: i64, _: i32, _: i64) -> Result<Vec<Job>, sqlx::Error> {
            Ok(vec![
                get_test_job(1).await
            ])
        }
    }

    #[async_trait]
    impl QueryJobsBySearchTermsFn for MockDbRepo {
        async fn query_jobs_by_search_terms(&self, _: Vec<String>, _: i32, _: i64) -> Result<Vec<Job>, sqlx::Error> {
            Ok(vec![
                get_test_job(1).await
            ])
        }
    }

    #[async_trait]
    impl QueryJobsByDeveloperFn for MockDbRepo {
        async fn query_jobs_by_developer(&self, _: i64, _: i32, _: i64) -> Result<Vec<Job>, sqlx::Error> {
            Ok(vec![
                get_test_job(1).await
            ])
        }
    }

    #[tokio::test]
    async fn test_create_job_route() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let login_result = login(app_data.clone(), Json(LoginCredential { dev_or_emp: AuthDeveloperOrEmployer::Employer, email: FreeEmail().fake::<String>(), password: "test123".to_string() })).await;
        let cookie = login_result.cookies().last().unwrap();
        let req = get_fake_httprequest_with_bearer_token(EMP_USERNAME.to_string(), UserDeveloperOrEmployer::Employer, &app_data.auth_keys.encoding_key, "/v1/job/update", 1, Some(60*2), Some(cookie));

        let result = create_job(app_data, Json(NewJobForRoute {
            employer_id: 1,
            title: get_fake_title().to_string(),
            description: get_fake_desc().to_string(),
            is_remote: false,
            country_id: Some(1),
            primary_lang_id: 1,
            secondary_lang_id: Some(2),
            industry_id: 1,
            salary_id: 1
        }), req).await.unwrap();

        assert!(result.id == 1);
    }

    #[tokio::test]
    async fn test_update_job_route() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let login_result = login(app_data.clone(), Json(LoginCredential { dev_or_emp: AuthDeveloperOrEmployer::Employer, email: FreeEmail().fake::<String>(), password: "test123".to_string() })).await;
        let cookie = login_result.cookies().last().unwrap();
        let req = get_fake_httprequest_with_bearer_token(EMP_USERNAME.to_string(), UserDeveloperOrEmployer::Employer, &app_data.auth_keys.encoding_key, "/v1/job/update", 1, Some(60*2), Some(cookie));
        let result = update_job(app_data, Json(UpdateJobForRoute {
            id: 1,
            employer_id: 1,
            title: get_fake_title().to_string(),
            description: get_fake_desc().to_string(),
            is_remote: false,
            country_id: Some(1),
            primary_lang_id: 1,
            secondary_lang_id: Some(2),
            industry_id: 1,
            salary_id: 1
        }), req).await;

        assert!(result.status().is_success());
    }

    #[tokio::test]
    async fn test_get_job_route() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let result = get_job(app_data, Path::from(1)).await.unwrap();

        assert!(result.unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_all_jobs_route() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let result = get_all_jobs(app_data, Json(PagingModel { page_size: 10, last_offset: 1 })).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_jobs_by_employer_route() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let result = get_jobs_by_employer(app_data, Json(IdAndPagingModel { id: 1, page_size: 10, last_offset: 1 })).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_jobs_by_search_terms_route() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let result = get_jobs_by_search_terms(app_data, Json(SearchAndPagingModel { search_terms: vec![], page_size: 10, last_offset: 1 })).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_jobs_by_dev_profile() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let result = get_jobs_by_developer(app_data, Json(IdAndPagingModel { id: 1, page_size: 10, last_offset: 1 })).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }
}