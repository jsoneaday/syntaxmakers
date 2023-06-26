use fake::Fake;
use fake::faker::company::en::CompanyName;
use fake::faker::internet::en::{Username, SafeEmail};
use fake::faker::lorem::en::{Sentence};
use syntaxmakers_server::common::repository::base::{ConnGetter, DbRepo};
use syntaxmakers_server::common::repository::companies::models::NewCompany;
use syntaxmakers_server::common::repository::employers::models::NewEmployer;
use syntaxmakers_server::common::repository::employers::repo::CreateEmployerFn;
use syntaxmakers_server::common::repository::jobs::models::NewJob;
use syntaxmakers_server::common::repository::jobs::repo::{GetJobFn, GetAllJobsFn, CreateJobFn};
use syntaxmakers_server::common::repository::industries::repo::GetAllIndustriesFn;
use syntaxmakers_server::common::repository::countries::repo::GetAllCountriesFn;
use syntaxmakers_server::common::repository::languages::repo::GetAllLanguagesFn;
use syntaxmakers_server::common::repository::salaries::repo::GetAllSalariesFn;
use syntaxmakers_server::common::repository::companies::repo::{CreateCompanyFn};
use syntaxmakers_server::common_test::fixtures::{ init_fixtures, get_fake_fullname};

#[tokio::test]
async fn test_create_job_and_get_back() {
    init_fixtures();
    let repo = DbRepo::init().await;
    let conn = &repo.get_conn();
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    
    let company_create_result = repo.create_company(conn, NewCompany{ name: CompanyName().fake::<String>() }).await.unwrap();
    let company_id = company_create_result.id;
    let create_employer_result = repo.create_employer(conn, NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        company_id
    }).await.unwrap();
    let countries_result = repo.get_all_countrie(conn).await.unwrap();
    let languages_result = repo.get_all_languages(conn).await.unwrap();
    let industry_result = repo.get_all_industries(conn).await.unwrap();
    let salary_result = repo.get_all_salaries(conn).await.unwrap();

    let create_result = repo.create_job(conn, NewJob {
        employer_id: create_employer_result.id,
        title: Sentence(5..6).fake::<String>(),
        description: Sentence(5..6).fake::<String>(),
        is_remote: true,
        headquarters_country_id: Some(countries_result.first().unwrap().id),
        primary_lang_id: languages_result.first().unwrap().id,
        secondary_lang_id: Some(languages_result.first().unwrap().id),
        industry_id: industry_result.first().unwrap().id,
        salary_id: salary_result.first().unwrap().id
    }).await.unwrap();
    let get_result = repo.get_job(conn, create_result.id).await.unwrap().unwrap();
    
    assert!(get_result.clone().id == create_result.id);
}


#[tokio::test]
async fn test_create_two_jobs_and_get_back_both() {
    init_fixtures();
    let repo = DbRepo::init().await;
    let conn = &repo.get_conn();
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    
    // setup needed data
    let company_create_result = repo.create_company(conn, NewCompany{ name: CompanyName().fake::<String>() }).await.unwrap();
    let company_id = company_create_result.id;
    let create_employer_result = repo.create_employer(conn, NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        company_id
    }).await.unwrap();
    let countries_result = repo.get_all_countrie(conn).await.unwrap();
    let languages_result = repo.get_all_languages(conn).await.unwrap();
    let industry_result = repo.get_all_industries(conn).await.unwrap();
    let salary_result = repo.get_all_salaries(conn).await.unwrap();

    // create two jobs
    let create_result1 = repo.create_job(conn, NewJob {
        employer_id: create_employer_result.id,
        title: Sentence(5..6).fake::<String>(),
        description: Sentence(5..6).fake::<String>(),
        is_remote: true,
        headquarters_country_id: Some(countries_result.first().unwrap().id),
        primary_lang_id: languages_result.first().unwrap().id,
        secondary_lang_id: Some(languages_result.first().unwrap().id),
        industry_id: industry_result.first().unwrap().id,
        salary_id: salary_result.first().unwrap().id
    }).await.unwrap();
    let create_result2 = repo.create_job(conn, NewJob {
        employer_id: create_employer_result.id,
        title: Sentence(5..6).fake::<String>(),
        description: Sentence(5..6).fake::<String>(),
        is_remote: true,
        headquarters_country_id: Some(countries_result.first().unwrap().id),
        primary_lang_id: languages_result.first().unwrap().id,
        secondary_lang_id: Some(languages_result.first().unwrap().id),
        industry_id: industry_result.first().unwrap().id,
        salary_id: salary_result.first().unwrap().id
    }).await.unwrap();

    // get all jobs and find two created
    let get_result = repo.get_all_jobs(conn, 10, 0).await.unwrap();
    
    assert!(get_result.iter().find(|job| { job.id == create_result1.id }).is_some());
    assert!(get_result.iter().find(|job| { job.id == create_result2.id }).is_some());
}