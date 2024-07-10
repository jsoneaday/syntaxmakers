use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    web::{Data, Json}, 
    HttpResponse,
    http::header::ContentType, HttpRequest
};
use chrono::Utc;
use log::error;
use crate::{
    app_state::AppState, 
    common::{
        authentication::auth_keys_service::{
            decode_token, get_token, Authenticator, REFRESH_TOKEN_LABEL, STANDARD_ACCESS_TOKEN_EXPIRATION, STANDARD_REFRESH_TOKEN_EXPIRATION
        }, 
        emailer::emailer::EmailerSendService, repository::{
            base::Repository, 
            developers::repo::{HasUnconfirmedDevEmailFn, QueryDeveloperFn}, 
            employers::repo::{HasUnconfirmedEmpEmailFn, QueryEmployerFn}, 
            user::{models::{AuthenticateResult, DeveloperOrEmployer as UserDeveloperOrEmployer}, repo::AuthenticateDbFn}
        }
    }, 
    routes::authentication::models::DeveloperOrEmployer as AuthDeveloperOrEmployer
};
use super::models::{LoginCredential, RefreshToken};


pub async fn refresh_access_token<T: Repository, E: EmailerSendService, U: Authenticator>(app_data: Data<AppState<T, E, U>>, json: Json<RefreshToken>, req: HttpRequest) -> HttpResponse {
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
            error!("No refresh cookie found");
            return HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .body("Authentication failed. Your request is missing the refresh token")
        }
    };
}

pub async fn login<
    T: HasUnconfirmedDevEmailFn + HasUnconfirmedEmpEmailFn + AuthenticateDbFn + QueryDeveloperFn + QueryEmployerFn + Repository, 
    E: EmailerSendService, 
    U: Authenticator
    >(app_data: Data<AppState<T, E, U>>, json: Json<LoginCredential>) 
    -> HttpResponse {
    let dev_or_emp = if json.dev_or_emp == AuthDeveloperOrEmployer::Developer {
        match app_data.repo.has_unconfirmed_dev_email(json.email.clone()).await {
            Ok(has_unconfirmed) => if has_unconfirmed {
                println!("Has unconfirmed email");
                return HttpResponse::NotAcceptable()
                .content_type(ContentType::json())
                .body("An email change confirmation is still pending. Please check your email.");
            },
            Err(e) => {
                println!("Failed to check if has unconfirmed email {}", e);
                return HttpResponse::NotAcceptable()
                .content_type(ContentType::json())
                .body("Something has gone wrong while checking for unconfirmed email confirmations");
            }
        };

        UserDeveloperOrEmployer::Developer
    } else {
        match app_data.repo.has_unconfirmed_emp_email(json.email.clone()).await {
            Ok(has_unconfirmed) => if has_unconfirmed {
                println!("Has unconfirmed email");
                return HttpResponse::NotAcceptable()
                .content_type(ContentType::json())
                .body("An email change confirmation is still pending. Please check your email.");
            },
            Err(e) => {
                println!("Failed to check if has unconfirmed email {}", e);
                return HttpResponse::NotAcceptable()
                .content_type(ContentType::json())
                .body("Something has gone wrong while checking for unconfirmed email confirmations");
            }
        };

        UserDeveloperOrEmployer::Employer
    };
    let auth_result = app_data.repo.authenticate_db(dev_or_emp.clone(), json.email.clone(), json.password.clone()).await;
    
    match auth_result {
        Ok(result) => {
            match result {
                AuthenticateResult::Success { id } => {
                    #[allow(unused)] let mut user_name = "".to_string();                    
                    #[allow(unused)] let mut http_response: Option<HttpResponse> = None;
                    
                    if dev_or_emp == UserDeveloperOrEmployer::Developer {
                        let developer = app_data.repo.query_developer(id).await;
                        match developer {
                            Ok(opt_dev) => {
                                if let Some(dev) = opt_dev {
                                    user_name = dev.user_name;
                                    let (refresh_cookie, access_token) = get_refresh_and_access_token_response(app_data, user_name.as_str(), &dev_or_emp);
                                    http_response = Some(HttpResponse::Ok()
                                        .cookie(refresh_cookie)
                                        .body(access_token));
                                } else {
                                    http_response = Some(HttpResponse::Unauthorized()
                                        .content_type(ContentType::json())
                                        .body("Authentication failed. Developer not found"));
                                }
                            },
                            Err(_) => {
                                error!("Authentication failed. Error occurred while trying to get developer");
                                http_response = Some(HttpResponse::Unauthorized()
                                    .content_type(ContentType::json())
                                    .body("Authentication failed. Error occurred while trying to get developer"));
                            }
                        }
                    } else {
                        let employer = app_data.repo.query_employer(id).await;
                        match employer {
                            Ok(opt_emp) => {
                                if let Some(emp) = opt_emp {
                                    user_name = emp.user_name;
                                    let (refresh_cookie, access_token) = get_refresh_and_access_token_response(app_data, user_name.as_str(), &dev_or_emp);
                                    http_response = Some(HttpResponse::Ok()
                                        .cookie(refresh_cookie)
                                        .body(access_token));
                                } else {
                                    error!("Authentication faild. Developer not found");
                                    http_response = Some(HttpResponse::Unauthorized()
                                        .content_type(ContentType::json())
                                        .body("Authentication failed. Developer not found"));
                                }
                            },
                            Err(_) => {
                                error!("Authentication failed. Error occurred while trying to get developer");
                                http_response = Some(HttpResponse::Unauthorized()
                                    .content_type(ContentType::json())
                                    .body("Authentication failed. Error occurred while trying to get developer"));
                            }
                        }
                    }          
                    return http_response.unwrap();          
                },
                _ => {
                    HttpResponse::Unauthorized()
                        .content_type(ContentType::json())
                        .body("Authentication failed. Wrong email or password")
                }
            }
        }
        Err(_) => {
            error!("Authentication failed. Server error");
            HttpResponse::Unauthorized()
                .content_type(ContentType::json())
                .body("Authentication failed. Server error occurred while trying to authenticate")
        }
    }  
}

fn get_refresh_and_access_token_response<'a, T: AuthenticateDbFn + QueryDeveloperFn + Repository, E: EmailerSendService, U: Authenticator>(
    app_data: Data<AppState<T, E, U>>, user_name: &'a str, dev_or_emp: &'a UserDeveloperOrEmployer
) -> (Cookie<'a>, String) {
    let access_token = get_token(user_name.to_string(), dev_or_emp.clone(), &app_data.auth_keys.encoding_key, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));
    let refresh_token = get_token(user_name.to_string(), dev_or_emp.clone(), &app_data.auth_keys.encoding_key, None);
    let refresh_cookie = Cookie::build(REFRESH_TOKEN_LABEL, refresh_token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(STANDARD_REFRESH_TOKEN_EXPIRATION, 0))
        .http_only(true)
        .secure(false)
        //.same_site(SameSite::Lax)
        .finish();
                    
    (refresh_cookie, access_token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_http::StatusCode;
    use async_trait::async_trait;
    use chrono::Utc;
    use fake::{faker::internet::en::FreeEmail, Fake};
    use jsonwebtoken::DecodingKey;
    use crate::{
        common::{
            authentication::auth_keys_service::{AuthenticationError, STANDARD_REFRESH_TOKEN_EXPIRATION}, 
            repository::{developers::models::Developer, employers::models::Employer, user::repo::AuthenticateDbFn}
        }, 
        common_test::fixtures::{get_app_data, get_fake_dev_desc, get_fake_email, MockEmailer}
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

    #[tokio::test]
    async fn test_login_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await; 

        let result = login(app_data.clone(), Json(LoginCredential { 
            dev_or_emp: AuthDeveloperOrEmployer::Developer, email: FreeEmail().fake::<String>(), password: "test1234".to_string() 
        })).await;

        assert!(result.status() == StatusCode::OK);
        let cookie = result.cookies().last().unwrap();
        let refresh_token = cookie.value();
        let claims = decode_token(refresh_token, &app_data.auth_keys.decoding_key);
        
        assert!(claims.exp >= STANDARD_REFRESH_TOKEN_EXPIRATION as usize);
        assert!(claims.sub == DEV_USERNAME.to_string());        
    }
}