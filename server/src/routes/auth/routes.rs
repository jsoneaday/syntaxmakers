use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    web::{Data, Json}, HttpResponse, http::header::ContentType
};
use chrono::{Utc, Duration};
use jsonwebtoken::encode;
use crate::{app_state::AppState, common::{repository::{base::Repository, user::{repo::AuthenticateFn, models::AuthenticateResult}}, authentication::auth_service::Claims}};
use super::models::LoginCredential;

pub async fn login<T: AuthenticateFn + Repository>(app_data: Data<AppState<T>>, json: Json<LoginCredential>) -> HttpResponse {
    println!("start login {}, {}", json.email, json.password);

    let auth_result = app_data.repo.authenticate(json.email.clone(), json.password.clone()).await;
    
    match auth_result {
        Ok(result) => {
            if result == AuthenticateResult::Success {
                let claims = Claims { sub: "dave".to_string(), exp: (Utc::now() + Duration::days(90)).timestamp() as usize };
                let token = encode(&jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA), &claims, &app_data.auth_keys.encoding_key).unwrap();
                let cookie = Cookie::build("token", token.to_owned())
                    .path("/")
                    .max_age(ActixWebDuration::new(60 * 60, 0))
                    .http_only(true)
                    .finish();

                HttpResponse::Ok()
                    .cookie(cookie)
                    .body("Login successful")
            } else {
                HttpResponse::Unauthorized()
                .content_type(ContentType::json())
                .body("Authentication failed. Wrong email or password")
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
