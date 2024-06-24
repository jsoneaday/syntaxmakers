use fake::Fake;
use fake::faker::company::en::CompanyName;
use fake::faker::internet::en::{Username, SafeEmail, FreeEmail};
use syntaxmakers_server::common::repository::base::{Repository, DbRepo};
use syntaxmakers_server::common::repository::companies::models::NewCompany;
use syntaxmakers_server::common::repository::countries::repo::QueryAllCountriesFn;
use syntaxmakers_server::common::repository::developers::models::NewDeveloper;
use syntaxmakers_server::common::repository::developers::repo::InsertDeveloperFn;
use syntaxmakers_server::common::repository::employers::models::NewEmployer;
use syntaxmakers_server::common::repository::employers::repo::InsertEmployerFn;
use syntaxmakers_server::common::repository::jobs::models::{NewJob, Job, UpdateJob};
use syntaxmakers_server::common::repository::jobs::repo::{
    QueryJobFn, QueryAllJobsFn, QueryJobsByEmployerFn, InsertJobFn, UpdateJobFn, QueryJobsByDeveloper, QueryJobsBySearchTerms
};
use syntaxmakers_server::common::repository::industries::repo::QueryAllIndustriesFn;
use syntaxmakers_server::common::repository::languages::repo::QueryAllLanguagesFn;
use syntaxmakers_server::common::repository::companies::repo::InsertCompanyFn;
use syntaxmakers_server::common::repository::salaries::repo::QueryAllSalariesFn;
use syntaxmakers_server::common_test::fixtures::{ 
    init_fixtures, get_fake_fullname, get_company_logo_randomly, get_fake_title, get_fake_desc, get_random_salary, LANGUAGES, INDUSTRIES, COUNTRIES, get_random_email
};

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
    let get_result = repo.query_jobs_by_developer(developer.id, 10, 0).await.unwrap();
    
    assert!(get_result.iter().find(|job| { job.id == create_result1.id }).is_some());
    assert!(get_result.iter().find(|job| { job.id == create_result2.id }).is_none());
}

#[tokio::test]
async fn test_create_two_jobs_and_get_back_both_as_employer() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    let logo = get_company_logo_randomly();
    
    let company_create_result = repo.insert_company(NewCompany{ name: CompanyName().fake::<String>(), logo: Some(logo), headquarters_country_id: 1 }).await.unwrap();
    let company_id = company_create_result.id;
    let insert_employer_result = repo.insert_employer(NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: "test123".to_string(),
        company_id
    }).await.unwrap();
    let languages_result = LANGUAGES.get().unwrap();
    let industry_result = INDUSTRIES.get().unwrap();
    let emp_id = insert_employer_result.id;

    let insert_first_job_result = repo.insert_job(NewJob {
        employer_id: emp_id,
        title: get_fake_title().to_string(),
        description: get_fake_desc().to_string(),
        is_remote: true,
        country_id: None,
        primary_lang_id: languages_result.first().unwrap().id,
        secondary_lang_id: Some(languages_result.first().unwrap().id),
        industry_id: industry_result.first().unwrap().id,
        salary_id: get_random_salary().await.id
    }).await.unwrap();
    let first_job_id = insert_first_job_result.id;
    let insert_second_job_result = repo.insert_job(NewJob {
        employer_id: emp_id,
        title: get_fake_title().to_string(),
        description: get_fake_desc().to_string(),
        is_remote: false,
        country_id: Some(COUNTRIES.get().unwrap().first().unwrap().id),
        primary_lang_id: languages_result.first().unwrap().id,
        secondary_lang_id: Some(languages_result.first().unwrap().id),
        industry_id: industry_result.first().unwrap().id,
        salary_id: get_random_salary().await.id
    }).await.unwrap();
    let second_job_id = insert_second_job_result.id;
    
    let job_result = repo.query_jobs_by_employer(emp_id, 10, 0).await.unwrap();
    let jobs = job_result.iter().filter(|job| {
        job.id == first_job_id || job.id == second_job_id
    }).collect::<Vec<&Job>>();
    assert!(jobs.len() == 2);
    assert!(jobs.get(0).unwrap().id == second_job_id);
    assert!(jobs.get(1).unwrap().id == first_job_id);
}

#[tokio::test]
async fn test_update_job_that_is_remote_and_get_back() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_random_email();
    let logo = get_company_logo_randomly();
    
    let company_create_result = repo.insert_company(NewCompany{ name: CompanyName().fake::<String>(), logo: Some(logo), headquarters_country_id: 1 }).await.unwrap();
    let company_id = company_create_result.id;
    let insert_employer_a_result = repo.insert_employer(NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: "test123".to_string(),
        company_id
    }).await.unwrap();
    let countries_result = repo.query_all_countries().await.unwrap();
    let languages_result = repo.query_all_languages().await.unwrap();
    let industry_result = repo.query_all_industries().await.unwrap();
    let salary_result = repo.query_all_salaries().await.unwrap();

    let insert_job_result = repo.insert_job(NewJob {
        employer_id: insert_employer_a_result.id,
        title: get_fake_title().to_string(),
        description: get_fake_desc().to_string(),
        is_remote: true,
        country_id: None,
        primary_lang_id: languages_result.first().unwrap().id,
        secondary_lang_id: Some(languages_result.first().unwrap().id),
        industry_id: industry_result.first().unwrap().id,
        salary_id: salary_result.first().unwrap().id
    }).await.unwrap();
    let get_job_inserted_result = repo.query_job(insert_job_result.id).await.unwrap().unwrap();

    let insert_employer_b_result = repo.insert_employer(NewEmployer {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: SafeEmail().fake::<String>(),
        password: "test123".to_string(),
        company_id
    }).await.unwrap();
    let employer_id = insert_employer_b_result.id;
    let title = get_fake_title().to_string();
    let description = get_fake_desc().to_string();
    let is_remote = false;
    let country_id = Some(countries_result.first().unwrap().id);
    let primary_lang_id = languages_result.get(1).unwrap().id;
    let secondary_lang_id = Some(languages_result.get(2).unwrap().id);
    let industry_id = industry_result.get(1).unwrap().id;
    let salary_id = salary_result.get(1).unwrap().id;
    _ = repo.update_job(UpdateJob {
        id: insert_job_result.id,
        employer_id,
        title: title.clone(),
        description,
        is_remote,
        country_id,
        primary_lang_id,
        secondary_lang_id,
        industry_id,
        salary_id
    })
    .await;
    let get_job_updated_result = repo.query_job(insert_job_result.id).await.unwrap().unwrap();
    
    assert!(get_job_inserted_result.clone().employer_id != get_job_updated_result.clone().employer_id);
    assert!(get_job_inserted_result.clone().title != get_job_updated_result.clone().title);
    assert!(get_job_inserted_result.is_remote != get_job_updated_result.is_remote);
    assert!(get_job_inserted_result.country_id != get_job_updated_result.country_id);
    assert!(get_job_inserted_result.primary_lang_id != get_job_updated_result.primary_lang_id);
    assert!(get_job_inserted_result.secondary_lang_id != get_job_updated_result.secondary_lang_id);
    assert!(get_job_inserted_result.industry_id != get_job_updated_result.industry_id);
    assert!(get_job_inserted_result.salary_id != get_job_updated_result.salary_id);

    assert!(get_job_updated_result.clone().employer_id == employer_id);
    assert!(get_job_updated_result.title == title.clone());
    assert!(get_job_updated_result.is_remote == is_remote);
    assert!(get_job_updated_result.country_id == country_id);
    assert!(get_job_updated_result.primary_lang_id == primary_lang_id);
    assert!(get_job_updated_result.secondary_lang_id == secondary_lang_id);
    assert!(get_job_updated_result.industry_id == industry_id);
    assert!(get_job_updated_result.salary_id == salary_id);
}

#[tokio::test]
async fn test_create_two_distinct_jobs_and_run_search_on_them_to_get_correct_results() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    let logo = get_company_logo_randomly();
    let languages_result = LANGUAGES.get().unwrap();
    let industry_result = INDUSTRIES.get().unwrap();
    let countries = COUNTRIES.get().unwrap();
    
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
    
    let _developer = repo.insert_developer(NewDeveloper {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: FreeEmail().fake::<String>(),
        password: "test123".to_string(),
        primary_lang_id: languages_result.get(0).unwrap().id,
        secondary_lang_id: Some(languages_result.get(1).unwrap().id)
    }).await.unwrap();

    let country1 = countries.get(0).unwrap();
    let primary_lang1 = languages_result.get(0).unwrap();
    let secondary_lang1 = languages_result.get(1).unwrap();
    let industry1 = industry_result.get(0).unwrap();
    let create_job1 = repo.insert_job(NewJob {
        employer_id: create_employer_result.id,
        title: get_fake_title().to_string(),
        description: get_fake_desc().to_string(),
        is_remote: false,
        country_id: Some(country1.id),
        primary_lang_id: primary_lang1.id,
        secondary_lang_id: Some(secondary_lang1.id),
        industry_id: industry1.id,
        salary_id: get_random_salary().await.id
    }).await.unwrap();

    let country2 = countries.get(0).unwrap();
    let primary_lang2 = languages_result.get(2).unwrap();
    let secondary_lang2 = languages_result.get(3).unwrap();
    let industry2 = industry_result.get(1).unwrap();
    let create_job2 = repo.insert_job(NewJob {
        employer_id: create_employer_result.id,
        title: get_fake_title().to_string(),
        description: get_fake_desc().to_string(),
        is_remote: false,
        country_id: Some(country2.id),
        primary_lang_id: primary_lang2.id,
        secondary_lang_id: Some(secondary_lang2.id),
        industry_id: industry2.id,
        salary_id: get_random_salary().await.id
    }).await.unwrap();

    // get only jobs that match dev's profile
    let search_by_country1 = repo.query_jobs_by_search_terms(vec![country1.name.clone()], 10, 0).await;    
    assert!(search_by_country1.unwrap().iter().find(|job| {job.id == create_job1.id}).is_some());
}