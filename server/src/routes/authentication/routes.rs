use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    web::{Data, Json}, 
    HttpResponse,
    http::header::ContentType, HttpRequest
};
use chrono::Utc;
use crate::{
    app_state::AppState, 
    common::{
        repository::{
            base::Repository, 
            user::{repo::AuthenticateDbFn, models::{AuthenticateResult, DeveloperOrEmployer as UserDeveloperOrEmployer}}, 
            developers::repo::QueryDeveloperFn
        }, 
        authentication::auth_service::{get_token, STANDARD_REFRESH_TOKEN_EXPIRATION, Authenticator, STANDARD_ACCESS_TOKEN_EXPIRATION, REFRESH_TOKEN_LABEL, decode_token}
    }, 
    routes::authentication::models::DeveloperOrEmployer as AuthDeveloperOrEmployer
};
use super::models::{LoginCredential, RefreshToken};

pub async fn refresh_access_token<T: Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<RefreshToken>, req: HttpRequest) -> HttpResponse {
    let dev_or_emp = if json.dev_or_emp == AuthDeveloperOrEmployer::Developer {
        UserDeveloperOrEmployer::Developer
    } else {
        UserDeveloperOrEmployer::Employer
    };

    let refresh_cookie = req.cookie(REFRESH_TOKEN_LABEL);

    match refresh_cookie {
        Some(cookie) => {
            let cookie_val = cookie.value();            
            let refresh_token = decode_token(cookie_val, &app_data.auth_keys.decoding_key);
            let refresh_user_name = refresh_token.sub;
            let current_access_token = decode_token(&json.old_token, &app_data.auth_keys.decoding_key);
            if refresh_user_name == current_access_token.sub && refresh_token.exp >= (Utc::now().timestamp() as usize) {
                let new_access_token = get_token(refresh_user_name, dev_or_emp, &app_data.auth_keys.encoding_key, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(new_access_token);
            } else {
                return HttpResponse::BadRequest()
                    .content_type(ContentType::json())
                    .body("Authentication failed. Your request token is expired");
            }            
        },
        None => {
            println!("No refresh cookie found");
            return HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .body("Authentication failed. Your request is missing the refresh token")
        }
    };
}


pub async fn login<T: AuthenticateDbFn + QueryDeveloperFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<LoginCredential>) 
    -> HttpResponse {
    println!("start login {}, {}", json.email, json.password);

    let dev_or_emp = if json.dev_or_emp == AuthDeveloperOrEmployer::Developer {
        UserDeveloperOrEmployer::Developer
    } else {
        UserDeveloperOrEmployer::Employer
    };
    let auth_result = app_data.repo.authenticate_db(dev_or_emp.clone(), json.email.clone(), json.password.clone()).await;
    
    match auth_result {
        Ok(result) => {
            match result {
                AuthenticateResult::Success { id } => {
                    let developer = app_data.repo.query_developer(id).await;
                    match developer {
                        Ok(opt_dev) => {
                            if let Some(dev) = opt_dev {
                                let access_token = get_token(dev.user_name.clone(), dev_or_emp.clone(), &app_data.auth_keys.encoding_key, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));
                                let refresh_token = get_token(dev.user_name, dev_or_emp, &app_data.auth_keys.encoding_key, None);
                                let refresh_cookie = Cookie::build(REFRESH_TOKEN_LABEL, refresh_token.to_owned())
                                    .path("/")
                                    .max_age(ActixWebDuration::new(STANDARD_REFRESH_TOKEN_EXPIRATION, 0))
                                    .http_only(true)
                                    .secure(false)
                                    //.same_site(SameSite::Lax)
                                    .finish();
                                
                                HttpResponse::Ok()
                                    .cookie(refresh_cookie)
                                    .body(access_token)
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
    use chrono::Utc;
    use fake::{faker::internet::en::FreeEmail, Fake};
    use jsonwebtoken::{decode, Validation, DecodingKey};
    use crate::{
        common::{
            repository::{user::repo::AuthenticateDbFn, developers::models::Developer}, authentication::auth_service::{Claims, STANDARD_REFRESH_TOKEN_EXPIRATION, AuthenticationError}
        }, 
        common_test::fixtures::get_app_data
    };

    const DEV_USERNAME: &str = "tester";
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
    async fn test_login_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let result = login(app_data.clone(), Json(LoginCredential { dev_or_emp: AuthDeveloperOrEmployer::Developer, email: FreeEmail().fake::<String>(), password: "test123".to_string() })).await;

        assert!(result.status() == StatusCode::OK);
        let cookie = result.cookies().last().unwrap();
        let refresh_token = cookie.value();
        let claims = decode::<Claims>(refresh_token, &app_data.auth_keys.decoding_key, &Validation::new(jsonwebtoken::Algorithm::EdDSA)).unwrap().claims;
        
        assert!(claims.exp >= STANDARD_REFRESH_TOKEN_EXPIRATION as usize);
        assert!(claims.sub == DEV_USERNAME.to_string());        
    }
   
}