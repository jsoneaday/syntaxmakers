use fake::{faker::internet::en::{FreeEmail, Username}, Fake};
use syntaxmakers_server::{
    common::{
        authentication::auth_keys_service::AuthService, 
        repository::{
            base::{DbRepo, Repository}, 
            developers::{models::NewDeveloper, repo::{InsertDevForgotPasswordConfirmFn, InsertDeveloperFn, QueryDeveloperFn}}, 
            user::{models::{AuthenticateResult, RepoDeveloperOrEmployer, RepoResetPassword}, repo::{AuthenticateDbFn, ResetPasswordFn}}
        }
    }, 
    common_test::fixtures::{get_app_data, get_fake_dev_desc, get_fake_email, get_fake_fullname, init_fixtures, MockEmailer, LANGUAGES}
};

// note: the tests for change_password are in test_developer or test_employer!!!

#[tokio::test]
async fn test_authenticate_returns_authenticateresult() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let auth_service = AuthService;
    let emailer = MockEmailer;
    let app_data = get_app_data(repo, emailer.clone(), auth_service).await;

    let password = "test1234".to_string();
    let dev_entity = app_data.repo.insert_developer(NewDeveloper {
        user_name:Username().fake::<String>(), 
        full_name: get_fake_fullname(), 
        email: FreeEmail().fake::<String>(), 
        description: get_fake_dev_desc(),
        password: password.clone(), 
        primary_lang_id: 1, 
        secondary_lang_id: None 
    }, &emailer).await.unwrap();
    let developer = app_data.repo.query_developer(dev_entity.id).await.unwrap().unwrap();

    let auth_result = app_data.repo.authenticate_db(RepoDeveloperOrEmployer::Developer, developer.email.clone(), password).await.unwrap();
    assert!(auth_result == AuthenticateResult::Success { id: developer.id });

    let auth_result = app_data.repo.authenticate_db(RepoDeveloperOrEmployer::Developer, developer.email, "wrong password".to_string()).await.unwrap();
    assert!(auth_result == AuthenticateResult::Failure);
}

#[tokio::test]
async fn test_reset_dev_password_succeeds() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let emailer = MockEmailer;
    
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let primary_lang_id = LANGUAGES.get().unwrap()[0].id;

    let create_result = repo.insert_developer(NewDeveloper {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        description: get_fake_dev_desc(),
        password: "old1234".to_string(),
        primary_lang_id,
        secondary_lang_id: None
    }, &emailer).await.unwrap();

    let forgot_password = repo.insert_dev_forgot_password_confirm(email, &emailer).await.unwrap();

    let update_result = repo.reset_password(RepoResetPassword { 
        user_id: create_result.id, 
        new_password: "test4567".to_string(),
        dev_or_emp: RepoDeveloperOrEmployer::Developer,
        unique_key: forgot_password.unique_key
    }).await;
    assert!(update_result.is_ok());
}