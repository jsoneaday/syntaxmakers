use syntaxmakers_server::common_test::fixtures::get_fake_email;
use syntaxmakers_server::common_test::fixtures::get_fake_user_name;
use syntaxmakers_server::{
    common::repository::{
        application::{models::NewApplication, repo::InsertApplicationFn}, base::{DbRepo, Repository}, developers::{models::NewDeveloper, repo::InsertDeveloperFn}
    }, 
    common_test::fixtures::{get_fake_fullname, init_fixtures}
};

#[tokio::test]
async fn test_insert_application_fails_if_user_already_applied() {
    let repo = DbRepo::init().await;
    init_fixtures().await;

    let developer = repo.insert_developer(NewDeveloper { 
        user_name: get_fake_user_name(), 
        full_name: get_fake_fullname(), 
        email: get_fake_email(), 
        password: "123".to_string(), 
        primary_lang_id: 1, 
        secondary_lang_id: None
    }).await.unwrap();

    let result = repo.insert_application(NewApplication { job_id: 1, developer_id: developer.clone().id}).await;
    assert!(result.unwrap().id > 0);

    let result = repo.insert_application(NewApplication { job_id: 1, developer_id: developer.id}).await;
    assert!(result.is_err());
}