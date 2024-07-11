use actix_web::{web::{Data, Json, Path}, HttpResponse, HttpRequest};
use log::error;
use crate::{
    app_state::AppState, common::{
        authentication::auth_keys_service::Authenticator, emailer::emailer::EmailerSendService, repository::{
            base::Repository, 
            developers::repo::QueryDeveloperFn, 
            employers::repo::QueryEmployerFn, 
            jobs::{
                models::{Job, JobApplicant, JobApplied, NewJob, UpdateJob}, 
                repo::{InsertJobFn, QueryAllJobsFn, QueryJobFn, QueryJobsAndAppliersFn, QueryJobsByApplierFn, QueryJobsByDeveloperFn, QueryJobsByEmployerFn, QueryJobsBySearchTermsFn, QueryJobsBySearchTermsForEmpFn, UpdateJobFn}
            }
        }
    }, routes::{
        auth_helper::check_is_authenticated, base_model::{IdAndPagingModel, OutputId, PagingModel, SearchAndPagingModel, SearchForEmpAndPagingModel}, user_error::UserError
    }
};
use super::models::{JobAndApplicantResponder, JobAndApplicantResponders, JobAppliedResponder, JobAppliedResponders, JobResponder, JobResponders, NewJobForRoute, UpdateJobForRoute};
use crate::routes::authentication::models::DeveloperOrEmployer as AuthDeveloperOrEmployer;

#[allow(unused)]
pub async fn create_job<T: InsertJobFn + QueryEmployerFn + QueryDeveloperFn + Repository, E: EmailerSendService, U: Authenticator>(app_data: Data<AppState<T, E, U>>, json: Json<NewJobForRoute>, req: HttpRequest)
 -> Result<OutputId, UserError> {    
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
pub async fn update_job<T: UpdateJobFn + QueryEmployerFn + QueryDeveloperFn + Repository, E: EmailerSendService, U: Authenticator>(app_data: Data<AppState<T, E, U>>, json: Json<UpdateJobForRoute>, req: HttpRequest)
 -> HttpResponse {
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
        Err(e) => {
            error!("update job failed {}", e);
            HttpResponse::InternalServerError().body("Failed to update job")
        }
    }
}

#[allow(unused)]
pub async fn get_job<T: QueryJobFn + Repository, E: EmailerSendService, U: Authenticator>(app_data: Data<AppState<T, E, U>>, path: Path<i64>) -> Result<Option<JobResponder>, UserError> {
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
pub async fn get_all_jobs<T: QueryAllJobsFn + Repository, E: EmailerSendService, U: Authenticator>(app_data: Data<AppState<T, E, U>>, json: Json<PagingModel>) -> Result<JobResponders, UserError> {
    let result = app_data.repo.query_all_jobs(json.page_size, json.last_offset).await;
    
    return_jobs_result(result)
}

#[allow(unused)]
pub async fn get_jobs_by_developer<T: QueryJobsByDeveloperFn + Repository, E: EmailerSendService, U: Authenticator>(app_data: Data<AppState<T, E, U>>, json: Json<IdAndPagingModel>) -> Result<JobResponders, UserError> {
    let result = app_data.repo.query_jobs_by_developer(json.id, json.page_size, json.last_offset).await;
    // remove unneeded match
    match result {
        Ok(jobs) => {
            return_jobs_result(Ok(jobs))
        },
        Err(e) => Err(e.into())
    }    
}

#[allow(unused)]
pub async fn get_jobs_by_employer<T: QueryJobsByEmployerFn + Repository, E: EmailerSendService, U: Authenticator>(app_data: Data<AppState<T, E, U>>, json: Json<IdAndPagingModel>) 
    -> Result<JobResponders, UserError> {
    let result = app_data.repo.query_jobs_by_employer(json.id, json.page_size, json.last_offset).await;
    
    return_jobs_result(result)
}

#[allow(unused)]
pub async fn get_jobs_by_search_terms<T: QueryJobsBySearchTermsFn + Repository, E: EmailerSendService, U: Authenticator>(app_data: Data<AppState<T, E, U>>, json: Json<SearchAndPagingModel>) 
    -> Result<JobResponders, UserError> {
    let result = app_data.repo.query_jobs_by_search_terms(json.search_terms.clone(), json.page_size, json.last_offset).await;
    
    return_jobs_result(result)
}

#[allow(unused)]
pub async fn get_jobs_by_search_terms_for_emp<T: QueryJobsBySearchTermsForEmpFn + Repository, E: EmailerSendService, U: Authenticator>(
    app_data: Data<AppState<T, E, U>>, 
    json: Json<SearchForEmpAndPagingModel>
) -> Result<JobResponders, UserError> {
    let result = app_data.repo.query_jobs_by_search_terms_for_emp(json.emp_id, json.search_terms.clone(), json.page_size, json.last_offset).await;
    
    return_jobs_result(result)
}

#[allow(unused)]
pub async fn get_jobs_by_applier<T: QueryJobsByApplierFn + Repository, E: EmailerSendService, U: Authenticator>(app_data: Data<AppState<T, E, U>>, json: Json<IdAndPagingModel>) -> Result<JobAppliedResponders, UserError> {
    let result = app_data.repo.query_jobs_by_applier(json.id, json.page_size, json.last_offset).await;
    // remove unneeded match
    match result {
        Ok(jobs) => {
            return_job_applied_result(Ok(jobs))
        },
        Err(e) => Err(e.into())
    }    
}

#[allow(unused)]
pub async fn get_jobs_and_appliers<T: QueryJobsAndAppliersFn + Repository, E: EmailerSendService, U: Authenticator>(
    app_data: Data<AppState<T, E, U>>, 
    json: Json<IdAndPagingModel>
) -> Result<JobAndApplicantResponders, UserError> {
    let result = app_data.repo.query_jobs_and_appliers(json.id, json.page_size, json.last_offset).await;
    // remove unneeded match
    match result {
        Ok(jobs) => {
            return_job_applicant_result(Ok(jobs))
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

fn return_job_applicant_result(result: Result<Vec<JobApplicant>, sqlx::error::Error>) -> Result<JobAndApplicantResponders, UserError> {
    match result {
        Ok(jobs) => {
            let responders = jobs.iter().map(|job| {
                convert_job_applicant(job)
            })
            .collect::<Vec<JobAndApplicantResponder>>();
            Ok(JobAndApplicantResponders(responders))
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

fn convert_job_applicant(job_applicant: &JobApplicant) -> JobAndApplicantResponder {
    JobAndApplicantResponder {
        job_id: job_applicant.job_id,
        job_updated_at: job_applicant.job_updated_at,
        applied_at: job_applicant.applied_at,
        dev_id: job_applicant.dev_id,
        dev_full_name: job_applicant.dev_full_name.to_string(),
        dev_description: job_applicant.dev_description.to_string(),
        job_title: job_applicant.job_title.to_string(),
        dev_primary_lang_id: job_applicant.dev_primary_lang_id,
        dev_primary_lang_name: job_applicant.dev_primary_lang_name.to_string(),
        dev_secondary_lang_id: job_applicant.dev_secondary_lang_id,
        dev_secondary_lang_name: job_applicant.dev_secondary_lang_name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{
            authentication::auth_keys_service::AuthenticationError, repository::{
                developers::{models::Developer, repo::HasUnconfirmedDevEmailFn}, 
                employers::{models::Employer, repo::HasUnconfirmedEmpEmailFn}, 
                jobs::models::Job, 
                user::{models::{AuthenticateResult, DeveloperOrEmployer as UserDeveloperOrEmployer}, repo::AuthenticateDbFn}
            }            
        }, 
        common_test::fixtures::{
            get_app_data, get_fake_desc, get_fake_dev_desc, get_fake_email, get_fake_fullname, get_fake_httprequest_with_bearer_token, get_fake_title, init_fixtures, MockEmailer, COUNTRIES, INDUSTRIES, LANGUAGES, SALARY_BASE
        }, 
        routes::authentication::{models::LoginCredential, routes::login}
    };
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use fake::{faker::{company::en::CompanyName, internet::en::FreeEmail}, Fake};
    use jsonwebtoken::DecodingKey;
    use crate::common::repository::{jobs::repo::InsertJobFn, base::EntityId};

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
            Ok(Some(Developer::new(
                1,
                Utc::now(),
                Utc::now(),
                DEV_USERNAME.to_string(),
                "Tester Test".to_string(),
                "".to_string(),
                get_fake_email(),
                get_fake_dev_desc(),
                1,
                None
            )))
        }
    }    

    #[async_trait]
    impl QueryEmployerFn for MockDbRepo {
        async fn query_employer(&self, _: i64) -> Result<Option<Employer>, sqlx::Error> {
            Ok(Some(Employer::new(
                1,
                Utc::now(),
                Utc::now(),
                EMP_USERNAME.to_string(),
                "Tester Test".to_string(),
                FreeEmail().fake::<String>(),
                "".to_string(),
                1
            )))
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

    #[async_trait]
    impl HasUnconfirmedDevEmailFn for MockDbRepo {
        async fn has_unconfirmed_dev_email(&self, _: String) -> Result<bool, sqlx::Error> {
            Ok(false)
        }
    }  

    #[async_trait]
    impl HasUnconfirmedEmpEmailFn for MockDbRepo {
        async fn has_unconfirmed_emp_email(&self, _: String) -> Result<bool, sqlx::Error> {
            Ok(false)
        }
    } 

    #[async_trait]
    impl QueryJobsAndAppliersFn for MockDbRepo {
        async fn query_jobs_and_appliers(&self, _: i64, _: i32, _: i64) -> Result<Vec<JobApplicant>, sqlx::Error> {
            Ok(vec![JobApplicant {
                dev_id: 1,
                job_id: 1,
                job_updated_at: Utc::now(),
                applied_at: Utc::now(),
                dev_full_name: get_fake_fullname(),
                dev_description: get_fake_dev_desc(),
                job_title: get_fake_title().to_string(),
                dev_primary_lang_id: 1,
                dev_primary_lang_name: LANGUAGES.get().unwrap()[0].name.clone(),
                dev_secondary_lang_id: 2,
                dev_secondary_lang_name: LANGUAGES.get().unwrap()[1].name.clone(),
            }])
        }
    }

    #[tokio::test]
    async fn test_create_job_route() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await;

        let login_result = login(app_data.clone(), Json(LoginCredential { dev_or_emp: AuthDeveloperOrEmployer::Employer, email: FreeEmail().fake::<String>(), password: "test1234".to_string() })).await;
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
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await;

        let login_result = login(app_data.clone(), Json(LoginCredential { dev_or_emp: AuthDeveloperOrEmployer::Employer, email: FreeEmail().fake::<String>(), password: "test1234".to_string() })).await;
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
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await;

        let result = get_job(app_data, Path::from(1)).await.unwrap();

        assert!(result.unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_all_jobs_route() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await;

        let result = get_all_jobs(app_data, Json(PagingModel { page_size: 10, last_offset: 1 })).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_jobs_by_employer_route() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await;

        let result = get_jobs_by_employer(app_data, Json(IdAndPagingModel { id: 1, page_size: 10, last_offset: 1 })).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_jobs_by_search_terms_route() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await;

        let result = get_jobs_by_search_terms(app_data, Json(SearchAndPagingModel { search_terms: vec![], page_size: 10, last_offset: 1 })).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_jobs_by_dev_profile() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await;

        let result = get_jobs_by_developer(app_data, Json(IdAndPagingModel { id: 1, page_size: 10, last_offset: 1 })).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_job_and_applicants_route() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await;

        let result = get_jobs_and_appliers(app_data, Json(IdAndPagingModel {
            id: 1,
            page_size: 10,
            last_offset: 0
        })).await.unwrap();

        assert!(result.0.len() == 1);
    }
}