use syntaxmakers_server::{
    common::{
        authentication::auth_keys_service::{AuthService, Authenticator as _}, repository::{base::{DbRepo, Repository}, user::models::RepoDeveloperOrEmployer}
    }, common_test::fixtures::{get_app_data, get_fake_httprequest_with_bearer_token, init_fixtures, MockEmailer}, routes::route_utils::get_header_strings
};

#[tokio::test]
async fn test_is_authenticated() {    
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let auth_service = AuthService;
    let emailer = MockEmailer;
    let app_data = get_app_data(repo, emailer, auth_service).await;
    let user_name = "jon@jon.com".to_string();
    
    let req = get_fake_httprequest_with_bearer_token(
        user_name.clone(), RepoDeveloperOrEmployer::Developer, &app_data.auth_keys.encoding_key, "/v1/developer", 1, Some(60*2), None
    );
    let headers = get_header_strings(req.headers());

    let result = app_data.auth_service.is_authenticated(user_name, headers, &app_data.auth_keys.decoding_key).await.unwrap();

    assert!(result == true);
}