use actix_http::StatusCode;
use actix_jwt_auth_middleware::{TokenSigner, AuthResult, AuthError};
use actix_web::{web::{Data, Json}, get, HttpResponse, Responder, error::InternalError};
use jwt_compact::alg::Ed25519;
use serde::Deserialize;
use crate::{routes::auth::models::User, app_state::AppState, common::repository::base::DbRepo};

#[derive(Deserialize)]
pub struct LoginCredential {
    email: String,
    password: String
}

pub async fn login(cookie_signer: Data<TokenSigner<User, Ed25519>>, json: Json<LoginCredential>) -> AuthResult<HttpResponse> {
    println!("start login {}, {}", json.email, json.password);
    
    if json.email == "test@test.com".to_string() && json.password == "test123".to_string() {
        let user = User { id: 1 };
        Ok(HttpResponse::Ok()
            .cookie(cookie_signer.create_access_cookie(&user)?)
            .cookie(cookie_signer.create_refresh_cookie(&user)?)
            .body("You are now logged in"))
    } else {
        Err(AuthError::RefreshAuthorizerDenied(InternalError::new(
            "Wrong email or password",
            StatusCode::UNAUTHORIZED
        ).into()))
    }    
}

//pub async fn register()

#[get("/hello")]
pub async fn hello(app_data: Data<AppState<DbRepo>>, user: User) -> impl Responder {
    println!("Hello there, i see your user id is {} {:?}.", user.id, app_data);
    format!("Hello there, i see your user id is {}.", user.id)
}