use actix_web::{web::{Data, Json}, HttpRequest};
use log::error;
use crate::{
    app_state::AppState, common::{
        authentication::auth_service::Authenticator, 
        repository::{application::{models::NewApplication, repo::{DevHasAppliedFn, InsertApplicationFn}}, base::Repository, developers::repo::QueryDeveloperFn, employers::repo::QueryEmployerFn}
    }, 
    routes::{auth_helper::check_is_authenticated, base_model::{OutputBool, OutputId}, user_error::UserError}
};
use super::models::NewApplicationForRoute;
use crate::routes::authentication::models::DeveloperOrEmployer as AuthDeveloperOrEmployer;

pub async fn create_application<T: InsertApplicationFn + Repository + QueryEmployerFn + QueryDeveloperFn, U: Authenticator>
    (app_data: Data<AppState<T, U>>, new_application: Json<NewApplicationForRoute>, req: HttpRequest) -> Result<OutputId, UserError> {
    
    let is_auth = check_is_authenticated(app_data.clone(), new_application.developer_id, AuthDeveloperOrEmployer::Developer, req).await;
    if !is_auth {
        error!("Authorization failed");
        return Err(UserError::AuthenticationFailed);
    }

    match app_data.repo.insert_application(NewApplication { job_id: new_application.job_id, developer_id: new_application.developer_id }).await {
        Ok(entity) => Ok(OutputId { id: entity.id }),
        Err(e) => Err(e.into())
    }
}

pub async fn developer_applied<T: DevHasAppliedFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<NewApplicationForRoute>) -> Result<OutputBool, UserError> {    
    match app_data.repo.dev_has_applied(json.job_id, json.developer_id).await {
        Ok(applied) => Ok(OutputBool { result: applied }),
        Err(e) => Err(e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{
            authentication::auth_service::AuthenticationError, repository::{
                base::EntityId, developers::models::Developer, employers::models::Employer, jobs::models::Job, user::{models::{AuthenticateResult, DeveloperOrEmployer as UserDeveloperOrEmployer}, repo::AuthenticateDbFn}
            }            
        }, 
        common_test::fixtures::{get_fake_fullname, get_fake_httprequest_with_bearer_token, init_fixtures, COUNTRIES, INDUSTRIES, LANGUAGES, SALARY_BASE}, 
        routes::authentication::{models::LoginCredential, routes::login}
    };
    use async_trait::async_trait;
    use chrono::Utc;
    use fake::{faker::{company::en::CompanyName, internet::en::FreeEmail}, Fake};
    use jsonwebtoken::DecodingKey;
    use crate::common_test::fixtures::{get_app_data, get_fake_desc, get_fake_title};
    use super::*;

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

    async fn _get_test_job(id: i64) -> Job {
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
    impl InsertApplicationFn for MockDbRepo {
        async fn insert_application(&self, _: NewApplication) -> Result<EntityId, sqlx::error::Error> {
            Ok(EntityId { id: 1 })
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
                FreeEmail().fake::<String>(),
                "".to_string(),
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
    impl DevHasAppliedFn for MockDbRepo {
        async fn dev_has_applied(&self, _: i64, _:i64) -> Result<bool, sqlx::Error> {
            Ok(true)
        }
    }   

    #[tokio::test]
    async fn test_create_job_application_route() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let login_result = login(
            app_data.clone(), 
            Json(LoginCredential { dev_or_emp: AuthDeveloperOrEmployer::Employer, email: FreeEmail().fake::<String>(), password: "test1234".to_string() })
        ).await;
        let cookie = login_result.cookies().last().unwrap();
        let req = get_fake_httprequest_with_bearer_token(
            EMP_USERNAME.to_string(), UserDeveloperOrEmployer::Employer, &app_data.auth_keys.encoding_key, "/v1/application/insert", 1, Some(60*2), Some(cookie)
        );

        let result = create_application(app_data, Json(NewApplicationForRoute {
            job_id: 1,
            developer_id: 1
        }), req).await;

        assert!(result.unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_developer_applied() {
        init_fixtures().await;
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let response = developer_applied(app_data, Json(NewApplicationForRoute { job_id: 1, developer_id: 1 })).await.unwrap();

        assert!(response.result == true);
    }
}