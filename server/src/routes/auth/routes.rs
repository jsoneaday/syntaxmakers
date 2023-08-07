use actix_jwt_auth_middleware::{TokenSigner, AuthResult};
use actix_web::{web::Data, get, HttpResponse, Responder};
use jwt_compact::alg::Ed25519;

use crate::{routes::auth::models::User, app_state::AppState, common::repository::base::DbRepo};



#[get("/login")]
pub async fn login(cookie_signer: Data<TokenSigner<User, Ed25519>>) -> AuthResult<HttpResponse> {
    println!("start login");
    let user = User { id: 1 };
    Ok(HttpResponse::Ok()
        .cookie(cookie_signer.create_access_cookie(&user)?)
        .cookie(cookie_signer.create_refresh_cookie(&user)?)
        .body("You are now logged in"))
}

#[get("/hello")]
pub async fn hello(app_data: Data<AppState<DbRepo>>, user: User) -> impl Responder {
    println!("Hello there, i see your user id is {} {:?}.", user.id, app_data);
    format!("Hello there, i see your user id is {}.", user.id)
}