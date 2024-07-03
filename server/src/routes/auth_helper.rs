use actix_web::{web::Data, HttpRequest};
use log::{error, info};
use crate::{common::{repository::{developers::repo::QueryDeveloperFn, employers::repo::QueryEmployerFn, base::Repository}, authentication::auth_service::Authenticator}, app_state::AppState};
use super::{authentication::models::DeveloperOrEmployer as AuthDeveloperOrEmployer, route_utils::get_header_strings};


pub async fn check_is_authenticated<T: QueryDeveloperFn + QueryEmployerFn + Repository, U: Authenticator>(
    app_data: Data<AppState<T, U>>, 
    id: i64,
    dev_or_emp: AuthDeveloperOrEmployer,
    req: HttpRequest
) -> bool {
    #[allow(unused)]
    let mut user_name: String = "".to_string();

    if dev_or_emp == AuthDeveloperOrEmployer::Employer {
        let employer_result = app_data.repo.query_employer(id).await;
        if employer_result.is_err() {
            error!("User not found, {:?}", employer_result.err().unwrap());
            return false;
        }
        let wrapped_employer = employer_result.ok().unwrap();
        if let None = wrapped_employer {
            error!("User not found");
            return false;
        } 
        let employer = wrapped_employer.unwrap();
        user_name = employer.user_name;
    } else {
        let dev_result = app_data.repo.query_developer(id).await;
        if dev_result.is_err() {
            error!("User not found, {:?}", dev_result.err().unwrap());
            return false;
        }
        let wrapped_dev = dev_result.ok().unwrap();
        if let None = wrapped_dev {
            error!("User not found");
            return false;
        } 
        let dev = wrapped_dev.unwrap();
        user_name = dev.user_name;
    }
    
    let headers = get_header_strings(req.headers());
    info!("headers {:?}", headers);
    let is_authenticated_result = app_data.auth_service.is_authenticated(user_name, headers, &app_data.auth_keys.decoding_key).await;
    match is_authenticated_result {
        Ok(result) => match result {
            true => {
                info!("Successfully authorized");
                true
            },
            false => {
                info!("Failed authorization");
                false
            }
        },
        Err(_) => {
            error!("Authorization attempt failed");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use actix_web::web::Json;
    use chrono::Utc;
    use fake::{faker::internet::en::FreeEmail, Fake};
    use jsonwebtoken::DecodingKey;
    use crate::{
        common::{
            authentication::auth_service::{Authenticator, AuthenticationError}, 
            repository::{user::{repo::AuthenticateDbFn, models::{DeveloperOrEmployer as UserDeveloperOrEmployer, AuthenticateResult}}, developers::models::Developer, employers::models::Employer}
        }, 
        common_test::fixtures::{get_app_data, get_fake_httprequest_with_bearer_token}, routes::authentication::{routes::login, models::LoginCredential}
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

    #[tokio::test]
   async fn test_check_is_authenticated() {
    let repo = MockDbRepo::init().await;
    let auth_service = MockAuthService;
    let app_data = get_app_data(repo, auth_service).await;
    let dev_or_emp = AuthDeveloperOrEmployer::Developer;
    let id: i64 = 1;

    let login_result = login(app_data.clone(), Json(LoginCredential { dev_or_emp: dev_or_emp.clone(), email: FreeEmail().fake::<String>(), password: "test1234".to_string() })).await;
    let cookie = login_result.cookies().last().unwrap();
    let req = get_fake_httprequest_with_bearer_token(
        EMP_USERNAME.to_string(), 
        UserDeveloperOrEmployer::Developer, 
        &app_data.auth_keys.encoding_key, 
        "/v1/developer", 
        1, 
        Some(60*2), 
        Some(cookie)
    );
    
    let check_result = check_is_authenticated(app_data, id, dev_or_emp, req).await;
    assert!(check_result == true);
   }
}