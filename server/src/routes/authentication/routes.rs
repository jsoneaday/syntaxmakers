use actix_http::header::HeaderMap;
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    web::{Data, Json}, 
    HttpResponse,
    http::header::ContentType
};
use chrono::Utc;
use jsonwebtoken::DecodingKey;
use crate::{
    app_state::AppState, 
    common::{
        repository::{
            base::Repository, user::{repo::AuthenticateFn, models::{AuthenticateResult, DeveloperOrEmployer as UserDeveloperOrEmployer}}, developers::repo::QueryDeveloperFn
        }, 
        authentication::auth_service::{get_token, decode_token}
    }, 
    routes::{authentication::models::DeveloperOrEmployer as AuthDeveloperOrEmployer, user_error::UserError}
};
use super::models::LoginCredential;

// todo: need to figure out how to mock this
pub async fn is_authenticated(user_name: String, headers: &HeaderMap, decoding_key: &DecodingKey) -> Result<bool, UserError> {
    let mut result: Result<bool, UserError> = Err(UserError::InternalError);

    _ = headers.iter().for_each(|header| {
        let header_name = header.0.as_str();
        let header_val = header.1.to_str();
        
        if header_name == "authorization" {
            match header_val {
                Ok(bearer) => {
                    let bearer_items: Vec<&str> = bearer.split(' ').collect();
                    let claims = decode_token(bearer_items.get(1).unwrap(), decoding_key);
                    
                    if claims.sub == user_name {
                        if claims.exp >= (Utc::now().timestamp() as usize) {
                            result = Ok(true);
                        }
                    }                    
                },
                Err(_) => result = Err(UserError::InternalError)
            }
        }
    });

    result
}

pub async fn login<T: AuthenticateFn + QueryDeveloperFn + Repository>(app_data: Data<AppState<T>>, json: Json<LoginCredential>) -> HttpResponse {
    println!("start login {}, {}", json.email, json.password);

    let dev_or_emp = if json.is_dev_or_emp == AuthDeveloperOrEmployer::Developer {
        UserDeveloperOrEmployer::Developer
    } else {
        UserDeveloperOrEmployer::Employer
    };
    let auth_result = app_data.repo.authenticate(dev_or_emp, json.email.clone(), json.password.clone()).await;
    
    match auth_result {
        Ok(result) => {
            match result {
                AuthenticateResult::Success { id } => {
                    let developer = app_data.repo.query_developer(id).await;
                    match developer {
                        Ok(opt_dev) => {
                            if let Some(dev) = opt_dev {
                                let token = get_token(dev.user_name, &app_data.auth_keys.encoding_key, None);
                                let cookie = Cookie::build("token", token.to_owned())
                                    .path("/")
                                    .max_age(ActixWebDuration::new(60 * 60, 0))
                                    .http_only(true)
                                    .finish();
            
                                HttpResponse::Ok()
                                    .cookie(cookie)
                                    .content_type(ContentType::json())
                                    .body(format!("{}", id))
                            } else {
                                HttpResponse::Unauthorized()
                                    .content_type(ContentType::json())
                                    .body("Authentication failed. User not found")
                            }
                        },
                        Err(_) => {
                            HttpResponse::Unauthorized()
                                .content_type(ContentType::json())
                                .body("Authentication failed. Error occurred while trying to get user")
                        }
                    }
                    
                },
                _ => {
                    HttpResponse::Unauthorized()
                        .content_type(ContentType::json())
                        .body("Authentication failed. Wrong email or password")
                }
            }
        }
        Err(_) => {
            HttpResponse::Unauthorized()
                .content_type(ContentType::json())
                .body("Authentication failed. Server error occurred while trying to authenticate")
        }
    }  
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_http::StatusCode;
    use async_trait::async_trait;
    use chrono::Duration;
    use fake::{faker::internet::en::FreeEmail, Fake};
    use jsonwebtoken::{decode, Validation};
    use crate::{common::{repository::{user::repo::AuthenticateFn, developers::models::Developer}, authentication::auth_service::Claims}, common_test::fixtures::{get_app_data, get_fake_request_with_bearer_token}};

    const DEV_USERNAME: &str = "tester";
    struct MockDbRepo;

    #[async_trait]
    impl Repository for MockDbRepo {
        async fn init() -> Self {
            MockDbRepo
        }
    }

    #[async_trait]
    impl AuthenticateFn for MockDbRepo {
        async fn authenticate(&self, _: UserDeveloperOrEmployer, _: String, _: String) -> Result<AuthenticateResult, sqlx::Error> {
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

    #[tokio::test]
    async fn test_is_authenticated() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let req = get_fake_request_with_bearer_token(DEV_USERNAME.to_string(), &app_data.auth_keys.encoding_key, "/v1/developer", 1);

        let result = is_authenticated(DEV_USERNAME.to_string(), req.headers(), &app_data.auth_keys.decoding_key).await.unwrap();

        assert!(result == true);
    }

    #[tokio::test]
    async fn test_login_route() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let result = login(app_data.clone(), Json(LoginCredential { is_dev_or_emp: AuthDeveloperOrEmployer::Developer, email: FreeEmail().fake::<String>(), password: "test123".to_string() })).await;

        assert!(result.status() == StatusCode::OK);
        let cookie = result.cookies().last().unwrap();
        let token = cookie.value();
        let claims = decode::<Claims>(token, &app_data.auth_keys.decoding_key, &Validation::new(jsonwebtoken::Algorithm::EdDSA)).unwrap().claims;
        assert!(claims.sub == DEV_USERNAME.to_string());
        assert!(claims.exp <= (Utc::now() + Duration::days(90)).timestamp() as usize);
    }
}