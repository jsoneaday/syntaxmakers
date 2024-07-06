use fake::Fake;
use fake::faker::company::en::CompanyName;
use fake::faker::internet::en::{Username, SafeEmail};
use syntaxmakers_server::common::repository::base::{Repository, DbRepo};
use syntaxmakers_server::common::repository::companies::models::NewCompany;
use syntaxmakers_server::common::repository::employers::models::{NewEmployer, UpdateEmployer};
use syntaxmakers_server::common::repository::employers::repo::{ConfirmEmailFn, InsertEmployerFn, QueryAllEmployersFn, QueryEmployerByEmailFn, QueryEmployerFn, UpdateEmployerFn};
use syntaxmakers_server::common::repository::companies::repo::InsertCompanyFn;
use syntaxmakers_server::common_test::fixtures::{ get_company_logo_randomly, get_fake_email, get_fake_fullname, get_fake_user_name, init_fixtures};

#[tokio::test]
async fn test_create_employer_and_get_back() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let logo = get_company_logo_randomly();
    
    let company_create_result = repo.insert_company(NewCompany{ name: CompanyName().fake::<String>(), logo: Some(logo), headquarters_country_id: 1 }).await.unwrap();
    let company_id = company_create_result.id;

    let create_result = repo.insert_employer(NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: "test1234".to_string(),
        company_id
    }).await.unwrap();
    let get_result = repo.query_employer(create_result.id).await.unwrap().unwrap();
    
    assert!(get_result.clone().id == create_result.id);
    assert!(get_result.clone().user_name == user_name);
    assert!(get_result.clone().full_name == full_name);
    assert!(get_result.clone().email == email);
    assert!(get_result.clone().company_id == company_id);
}

#[tokio::test]
async fn test_create_employer_and_check_does_not_allow_existing_email() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let logo = get_company_logo_randomly();
    
    let company_create_result = repo.insert_company(NewCompany{ name: CompanyName().fake::<String>(), logo: Some(logo), headquarters_country_id: 1 }).await.unwrap();
    let company_id = company_create_result.id;

    _ = repo.insert_employer(NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: "test1234".to_string(),
        company_id
    }).await.unwrap();
    let create_result2 = repo.insert_employer(NewEmployer {
        user_name: get_fake_user_name(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: "test1234".to_string(),
        company_id
    }).await;
    
    assert!(create_result2.is_err());
    assert!(create_result2.err().unwrap().as_database_error().unwrap().is_unique_violation());    
}

#[tokio::test]
async fn test_create_employer_and_check_does_not_allow_existing_username() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    let logo = get_company_logo_randomly();
    
    let company_create_result = repo.insert_company(NewCompany{ name: CompanyName().fake::<String>(), logo: Some(logo), headquarters_country_id: 1 }).await.unwrap();
    let company_id = company_create_result.id;

    _ = repo.insert_employer(NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: "test1234".to_string(),
        company_id
    }).await.unwrap();
    let create_result2 = repo.insert_employer(NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: get_fake_email(),
        password: "test1234".to_string(),
        company_id
    }).await;
    
    assert!(create_result2.is_err());
    assert!(create_result2.err().unwrap().as_database_error().unwrap().is_unique_violation());    
}

#[tokio::test]
async fn test_create_employer_and_get_back_by_email() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    let logo = get_company_logo_randomly();
    
    let company_create_result = repo.insert_company(NewCompany{ name: CompanyName().fake::<String>(), logo: Some(logo), headquarters_country_id: 1 }).await.unwrap();
    let company_id = company_create_result.id;

    let create_result = repo.insert_employer(NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        password: "test1234".to_string(),
        company_id
    }).await.unwrap();
    
    let get_result = repo.query_employer_by_email(email.clone()).await.unwrap().unwrap();
    
    assert!(get_result.clone().id == create_result.id);
    assert!(get_result.clone().user_name == user_name);
    assert!(get_result.clone().full_name == full_name);
    assert!(get_result.clone().email == email);
    assert!(get_result.clone().company_id == company_id);
}

#[tokio::test]
async fn test_create_two_employers_and_get_back_both() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let logo = get_company_logo_randomly();

    let company_create_result = repo.insert_company(NewCompany{ name: CompanyName().fake::<String>(), logo: Some(logo), headquarters_country_id: 1 }).await.unwrap();
    let company_id = company_create_result.id;

    let create_result1 = repo.insert_employer(NewEmployer {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: SafeEmail().fake::<String>(),
        password: "test1234".to_string(),
        company_id
    }).await.unwrap();
    let create_result2 = repo.insert_employer(NewEmployer {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: SafeEmail().fake::<String>(),
        password: "test1234".to_string(),
        company_id
    }).await.unwrap();

    let get_all_result = repo.query_all_employers(10, 0).await.unwrap();
    
    assert!(get_all_result.iter().find(|dev| {
        dev.id == create_result1.id
    }).is_some());
    assert!(get_all_result.iter().find(|dev| {
        dev.id == create_result2.id
    }).is_some());
}

#[tokio::test]
async fn test_create_employer_then_update_and_confirm_new_field_values() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let logo = get_company_logo_randomly();

    let company_create_result = repo.insert_company(NewCompany{ name: CompanyName().fake::<String>(), logo: Some(logo.clone()), headquarters_country_id: 1 }).await.unwrap();
    let company_id = company_create_result.id;

    let create_result = repo.insert_employer(NewEmployer {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: get_fake_email(),
        password: "test1234".to_string(),
        company_id
    }).await.unwrap();

    let company_update_result = repo.insert_company(NewCompany{ name: CompanyName().fake::<String>(), logo: Some(logo), headquarters_country_id: 1 }).await.unwrap();
    let update_company_id = company_update_result.id;
    let full_name = get_fake_fullname();
    let email = get_fake_email();
    _ = repo.update_employer(UpdateEmployer {
        id: create_result.id,
        full_name: full_name.clone(),
        email: email.clone(),
        company_id: update_company_id
    }).await.unwrap();

    let updated = repo.query_employer(create_result.id).await.unwrap().unwrap();

    assert!(updated.full_name == full_name);
    assert!(updated.email == email);
    assert!(updated.company_id == update_company_id);
}

#[tokio::test]
async fn test_insert_emp_and_confirm_email() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let email = get_fake_email();
    
    // insert_developer should create a new email confirm
    let create_result1 = repo.insert_employer(NewEmployer {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: email.clone(),
        password: "test1234".to_string(),
        company_id: 1
    }).await.unwrap();

    match repo.confirm_email(email, create_result1.id).await {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    }
}

#[tokio::test]
async fn test_update_emp_email_and_confirm_it() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let old_email = get_fake_email();
    
    let created_result1 = repo.insert_employer(NewEmployer {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: old_email.clone(),
        password: "test1234".to_string(),
        company_id: 1
    }).await.unwrap();

    let new_email = get_fake_email();
    _ = repo.update_employer(UpdateEmployer {
        id: created_result1.id,
        full_name: get_fake_fullname(),
        email: new_email.clone(),
        company_id: 2
    }).await.unwrap();
    
    match repo.confirm_email(new_email.clone(), created_result1.id).await {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    }

    match repo.query_employer(created_result1.id).await {
        Ok(dev) => match dev {
            Some(dev) => dev.email == new_email,
            None => panic!("Employer's email does not match after email confirm")
        },
        Err(e) => panic!("{}", e)
    };
}