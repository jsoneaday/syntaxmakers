use syntaxmakers_server::common::repository::jobs::models::NewJob;
use syntaxmakers_server::common::repository::jobs::repo::InsertJobFn;
use syntaxmakers_server::common_test::fixtures::get_fake_desc;
use syntaxmakers_server::common_test::fixtures::get_fake_dev_desc;
use syntaxmakers_server::common_test::fixtures::get_fake_email;
use syntaxmakers_server::common_test::fixtures::get_fake_title;
use syntaxmakers_server::common_test::fixtures::get_fake_user_name;
use syntaxmakers_server::common_test::fixtures::COUNTRIES;
use syntaxmakers_server::common_test::fixtures::INDUSTRIES;
use syntaxmakers_server::common_test::fixtures::LANGUAGES;
use syntaxmakers_server::common_test::fixtures::SALARY_BASE;
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

    let job = repo.insert_job(NewJob {
        employer_id: 1,
        title: get_fake_title().to_string(),
        description: get_fake_desc().to_string(),
        is_remote: false,
        country_id: Some(COUNTRIES.get().unwrap()[0].id),
        primary_lang_id: LANGUAGES.get().unwrap()[0].id,
        secondary_lang_id: Some(LANGUAGES.get().unwrap()[1].id),
        industry_id: INDUSTRIES.get().unwrap()[0].id,
        salary_id: SALARY_BASE.get().unwrap()[0].id
    }).await;

    let developer = repo.insert_developer(NewDeveloper { 
        user_name: get_fake_user_name(), 
        full_name: get_fake_fullname(), 
        email: get_fake_email(), 
        description: get_fake_dev_desc(),
        password: "123".to_string(), 
        primary_lang_id: 1, 
        secondary_lang_id: None
    }).await.unwrap();

    let result = repo.insert_application(NewApplication { job_id: job.as_ref().unwrap().id, developer_id: developer.clone().id}).await;
    assert!(result.unwrap().id > 0);

    let result = repo.insert_application(NewApplication { job_id: job.unwrap().id, developer_id: developer.id}).await;
    assert!(result.is_err());
}