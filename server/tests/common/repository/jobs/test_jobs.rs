use fake::Fake;
use fake::faker::company::en::CompanyName;
use fake::faker::internet::en::{Username, SafeEmail, FreeEmail};
use syntaxmakers_server::common::repository::base::{Repository, DbRepo};
use syntaxmakers_server::common::repository::companies::models::NewCompany;
use syntaxmakers_server::common::repository::developers::models::NewDeveloper;
use syntaxmakers_server::common::repository::developers::repo::InsertDeveloperFn;
use syntaxmakers_server::common::repository::employers::models::NewEmployer;
use syntaxmakers_server::common::repository::employers::repo::InsertEmployerFn;
use syntaxmakers_server::common::repository::jobs::models::NewJob;
use syntaxmakers_server::common::repository::jobs::repo::{QueryJobFn, QueryAllJobsFn, InsertJobFn, QueryJobsByDevProfile};
use syntaxmakers_server::common::repository::industries::repo::QueryAllIndustriesFn;
use syntaxmakers_server::common::repository::languages::repo::QueryAllLanguagesFn;
use syntaxmakers_server::common::repository::companies::repo::InsertCompanyFn;
use syntaxmakers_server::common_test::fixtures::{ init_fixtures, get_fake_fullname, get_company_logo_randomly, get_fake_title, get_fake_desc, get_random_salary};

#[tokio::test]
async fn test_create_job_and_get_back() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    let logo = get_company_logo_randomly();
    
    let company_create_result = repo.insert_company(NewCompany{ name: CompanyName().fake::<String>(), logo: Some(logo), headquarters_country_id: 1 }).await.unwrap();
    let company_id = company_create_result.id;
    let create_employer_result = repo.insert_employer(NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: "test123".to_string(),
        company_id
    }).await.unwrap();
    let languages_result = repo.query_all_languages().await.unwrap();
    let industry_result = repo.query_all_industries().await.unwrap();

    let create_result = repo.insert_job(NewJob {
        employer_id: create_employer_result.id,
        title: get_fake_title().to_string(),
        description: get_fake_desc().to_string(),
        is_remote: true,
        country_id: None,
        primary_lang_id: languages_result.first().unwrap().id,
        secondary_lang_id: Some(languages_result.first().unwrap().id),
        industry_id: industry_result.first().unwrap().id,
        salary_id: get_random_salary().await.id
    }).await.unwrap();
    let get_result = repo.query_job(create_result.id).await.unwrap().unwrap();
    
    assert!(get_result.id == create_result.id);
}


#[tokio::test]
async fn test_create_two_jobs_and_get_back_both() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    let logo = get_company_logo_randomly();
    
    // setup needed data
    let company_create_result = repo.insert_company(NewCompany{ name: CompanyName().fake::<String>(), logo: Some(logo), headquarters_country_id: 1 }).await.unwrap();
    let company_id = company_create_result.id;
    let create_employer_result = repo.insert_employer(NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: "test123".to_string(),
        company_id
    }).await.unwrap();
    let languages_result = repo.query_all_languages().await.unwrap();
    let industry_result = repo.query_all_industries().await.unwrap();

    // create two jobs
    let create_result1 = repo.insert_job(NewJob {
        employer_id: create_employer_result.id,
        title: get_fake_title().to_string(),
        description: get_fake_desc().to_string(),
        is_remote: true,
        country_id: None,
        primary_lang_id: languages_result.first().unwrap().id,
        secondary_lang_id: Some(languages_result.first().unwrap().id),
        industry_id: industry_result.first().unwrap().id,
        salary_id: get_random_salary().await.id
    }).await.unwrap();
    let create_result2 = repo.insert_job(NewJob {
        employer_id: create_employer_result.id,
        title: get_fake_title().to_string(),
        description: get_fake_desc().to_string(),
        is_remote: true,
        country_id: None,
        primary_lang_id: languages_result.first().unwrap().id,
        secondary_lang_id: Some(languages_result.first().unwrap().id),
        industry_id: industry_result.first().unwrap().id,
        salary_id: get_random_salary().await.id
    }).await.unwrap();

    // get all jobs and find two created
    let get_result = repo.query_all_jobs(10, 0).await.unwrap();
    
    assert!(get_result.iter().find(|job| { job.id == create_result1.id }).is_some());
    assert!(get_result.iter().find(|job| { job.id == create_result2.id }).is_some());
}

#[tokio::test]
async fn test_create_two_jobs_and_get_back_only_one_that_matches_dev_profile() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    let logo = get_company_logo_randomly();
    
    // setup needed data    
    let company_create_result = repo.insert_company(NewCompany{ name: CompanyName().fake::<String>(), logo: Some(logo), headquarters_country_id: 1 }).await.unwrap();
    let company_id = company_create_result.id;
    let create_employer_result = repo.insert_employer(NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: "test123".to_string(),
        company_id
    }).await.unwrap();
    let languages_result = repo.query_all_languages().await.unwrap();
    let industry_result = repo.query_all_industries().await.unwrap();
    let developer = repo.insert_developer(NewDeveloper {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: FreeEmail().fake::<String>(),
        password: "test123".to_string(),
        primary_lang_id: languages_result.get(0).unwrap().id,
        secondary_lang_id: Some(languages_result.get(1).unwrap().id)
    }).await.unwrap();

    // create two jobs
    let create_result1 = repo.insert_job(NewJob {
        employer_id: create_employer_result.id,
        title: get_fake_title().to_string(),
        description: get_fake_desc().to_string(),
        is_remote: true,
        country_id: None,
        primary_lang_id: languages_result.get(0).unwrap().id,
        secondary_lang_id: Some(languages_result.get(1).unwrap().id),
        industry_id: industry_result.first().unwrap().id,
        salary_id: get_random_salary().await.id
    }).await.unwrap();
    let create_result2 = repo.insert_job(NewJob {
        employer_id: create_employer_result.id,
        title: get_fake_title().to_string(),
        description: get_fake_desc().to_string(),
        is_remote: true,
        country_id: None,
        primary_lang_id: languages_result.get(2).unwrap().id,
        secondary_lang_id: Some(languages_result.get(3).unwrap().id),
        industry_id: industry_result.first().unwrap().id,
        salary_id: get_random_salary().await.id
    }).await.unwrap();

    // get only jobs that match dev's profile
    let get_result = repo.query_jobs_by_dev_profile(developer.id, 10, 0).await.unwrap();
    
    assert!(get_result.iter().find(|job| { job.id == create_result1.id }).is_some());
    assert!(get_result.iter().find(|job| { job.id == create_result2.id }).is_none());
}