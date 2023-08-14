use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    web::{Data, Json}, HttpResponse, http::header::ContentType
};
use chrono::{Utc, Duration};
use jsonwebtoken::encode;
use crate::{
    app_state::AppState, 
    common::{repository::{base::Repository, user::{repo::AuthenticateFn, models::{AuthenticateResult, DeveloperOrEmployer as UserDeveloperOrEmployer}}}, authentication::auth_service::Claims}, 
    routes::authentication::models::DeveloperOrEmployer as AuthDeveloperOrEmployer
};
use super::models::LoginCredential;

pub async fn login<T: AuthenticateFn + Repository>(app_data: Data<AppState<T>>, json: Json<LoginCredential>) -> HttpResponse {
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
                    let claims = Claims { sub: "dave".to_string(), exp: (Utc::now() + Duration::days(90)).timestamp() as usize };
                    let token = encode(&jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA), &claims, &app_data.auth_keys.encoding_key).unwrap();
                    let cookie = Cookie::build("token", token.to_owned())
                        .path("/")
                        .max_age(ActixWebDuration::new(60 * 60, 0))
                        .http_only(true)
                        .finish();

                    HttpResponse::Ok()
                        .cookie(cookie)
                        .content_type(ContentType::json())
                        .body(format!("{}", id))
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
                .body("Server error occurred while trying to authenticate")
        }
    }  
}

//pub async fn register()

#[cfg(test)]
mod tests {
    use super::*;
    use actix_http::StatusCode;
    use async_trait::async_trait;
    use fake::{faker::internet::en::FreeEmail, Fake};
    use jsonwebtoken::{decode, Validation};
    use crate::{common::repository::user::repo::AuthenticateFn, common_test::fixtures::{MockDbRepo, get_app_data}};

    #[async_trait]
    impl AuthenticateFn for MockDbRepo {
        async fn authenticate(&self, _: UserDeveloperOrEmployer, _: String, _: String) -> Result<AuthenticateResult, sqlx::Error> {
            Ok(AuthenticateResult::Success{ id: 1 })
        }
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
        assert!(claims.sub == "dave".to_string());
        assert!(claims.exp <= (Utc::now() + Duration::days(90)).timestamp() as usize);
    }
}