use actix_web::web::Json;
use chrono::Utc;
use syntaxmakers_server::{
    common_test::fixtures::{get_app_data, get_fake_httprequest_with_bearer_token, get_httpresponse_body_as_string}, 
    common::{
        repository::base::{Repository, DbRepo}, 
        repository::user::models::DeveloperOrEmployer as UserDeveloperOrEmployer,
        authentication::auth_service::{AuthService, STANDARD_ACCESS_TOKEN_EXPIRATION, decode_token, REFRESH_TOKEN_LABEL}, datetime_utils::convert_timestamp_to_local_datetime
    }, 
    routes::authentication::{
        models::{LoginCredential, DeveloperOrEmployer as AuthDeveloperOrEmployer, RefreshToken}, routes::{login, refresh_access_token}
    }    
};
use actix_http::StatusCode;
use actix_http::body::MessageBody;

const EXP_SEC_BUFFER: i64 = 5;

#[tokio::test]
async fn test_refresh_access_token_route() {
    let repo = DbRepo::init().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;
    let user_name = "jon".to_string();
    let email = "jon@jon.com".to_string();
    let password = "test123".to_string();
     
    let login_token_result = login(
        app_data.clone(), Json(LoginCredential { dev_or_emp: AuthDeveloperOrEmployer::Developer, email, password })
    ).await;    
    let (login_header, login_body) = login_token_result.into_parts();
    let refresh_cookie = login_header.cookies().find(|cookie| {
        cookie.name() == REFRESH_TOKEN_LABEL
    }).unwrap();
    let login_bytes_body = login_body.try_into_bytes();
    let old_access_token = get_httpresponse_body_as_string(login_bytes_body);

    let parameter_data = RefreshToken { old_token: old_access_token, dev_or_emp: AuthDeveloperOrEmployer::Developer };
    let url = "/v1/refreshtoken";
    let req = get_fake_httprequest_with_bearer_token(
        user_name.clone(), 
        UserDeveloperOrEmployer::Developer, 
        &app_data.auth_keys.encoding_key, 
        url, 
        parameter_data.clone(), 
        Some(STANDARD_ACCESS_TOKEN_EXPIRATION),
        Some(refresh_cookie.clone())
    );
    let access_token_result = refresh_access_token(
        app_data.clone(), Json(parameter_data), req
    ).await;
    let (access_head, access_body) = access_token_result.into_parts();

    assert!(access_head.status() == StatusCode::OK);
    let access_bytes = access_body.try_into_bytes();
    let access_token = get_httpresponse_body_as_string(access_bytes);
    let claims = decode_token(access_token.as_str(), &app_data.auth_keys.decoding_key);
    
    let local_date = convert_timestamp_to_local_datetime(claims.exp as i64);
    let comp_time = Utc::now().timestamp() + STANDARD_ACCESS_TOKEN_EXPIRATION - EXP_SEC_BUFFER;
    let local_comp_time = convert_timestamp_to_local_datetime(comp_time);

    println!("exp {local_date} vs {local_comp_time}");
    assert!(claims.exp >= comp_time as usize);
    assert!(claims.sub == user_name);        
}