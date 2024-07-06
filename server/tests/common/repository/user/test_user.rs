use fake::{faker::internet::en::{FreeEmail, Username}, Fake};
use syntaxmakers_server::{common::{authentication::auth_keys_service::AuthService, repository::{base::{DbRepo, Repository}, developers::{models::NewDeveloper, repo::{InsertDeveloperFn, QueryDeveloperFn}}, user::{models::{AuthenticateResult, DeveloperOrEmployer}, repo::AuthenticateDbFn}}}, common_test::fixtures::{get_app_data, get_fake_dev_desc, get_fake_fullname, init_fixtures}};

#[tokio::test]
async fn test_authenticate_returns_authenticateresult() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;

    let password = "test1234".to_string();
    let dev_entity = app_data.repo.insert_developer(NewDeveloper {
        user_name:Username().fake::<String>(), 
        full_name: get_fake_fullname(), 
        email: FreeEmail().fake::<String>(), 
        description: get_fake_dev_desc(),
        password: password.clone(), 
        primary_lang_id: 1, 
        secondary_lang_id: None 
    }).await.unwrap();
    let developer = app_data.repo.query_developer(dev_entity.id).await.unwrap().unwrap();

    let auth_result = app_data.repo.authenticate_db(DeveloperOrEmployer::Developer, developer.email.clone(), password).await.unwrap();
    assert!(auth_result == AuthenticateResult::Success { id: developer.id });

    let auth_result = app_data.repo.authenticate_db(DeveloperOrEmployer::Developer, developer.email, "wrong password".to_string()).await.unwrap();
    assert!(auth_result == AuthenticateResult::Failure);
}