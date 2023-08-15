use fake::{faker::internet::en::{FreeEmail, Username}, Fake};
use syntaxmakers_server::{common::repository::{base::{Repository, DbRepo}, user::{repo::AuthenticateFn, models::{DeveloperOrEmployer, AuthenticateResult}}, developers::{repo::{InsertDeveloperFn, QueryDeveloperFn}, models::NewDeveloper}}, common_test::fixtures::{init_fixtures, get_app_data, get_fake_fullname}};

#[tokio::test]
async fn test_authenticate_returns_authenticateresult() {
    init_fixtures();
    let repo = DbRepo::init().await;
    let app_data = get_app_data(repo).await;

    let password = "test123".to_string();
    let dev_entity = app_data.repo.insert_developer(NewDeveloper {
        user_name:Username().fake::<String>(), 
        full_name: get_fake_fullname(), 
        email: FreeEmail().fake::<String>(), 
        password: password.clone(), 
        primary_lang_id: 1, 
        secondary_lang_id: None 
    }).await.unwrap();
    let developer = app_data.repo.query_developer(dev_entity.id).await.unwrap().unwrap();

    let auth_result = app_data.repo.authenticate(DeveloperOrEmployer::Developer, developer.email.clone(), password).await.unwrap();
    assert!(auth_result == AuthenticateResult::Success { id: developer.id });

    let auth_result = app_data.repo.authenticate(DeveloperOrEmployer::Developer, developer.email, "wrong password".to_string()).await.unwrap();
    assert!(auth_result == AuthenticateResult::Failure);
}