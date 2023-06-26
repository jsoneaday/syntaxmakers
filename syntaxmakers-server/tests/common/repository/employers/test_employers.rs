use fake::Fake;
use fake::faker::company::en::CompanyName;
use fake::faker::internet::en::{Username, SafeEmail};
use syntaxmakers_server::common::repository::base::{ConnGetter, DbRepo};
use syntaxmakers_server::common::repository::companies::models::NewCompany;
use syntaxmakers_server::common::repository::employers::models::NewEmployer;
use syntaxmakers_server::common::repository::employers::repo::{GetEmployerFn, GetAllEmployersFn, CreateEmployerFn};
use syntaxmakers_server::common::repository::companies::repo::{CreateCompanyFn};
use syntaxmakers_server::common_test::fixtures::{ init_fixtures, get_fake_fullname};

#[tokio::test]
async fn test_create_employer_and_get_back() {
    init_fixtures();
    let repo = DbRepo::init().await;
    let conn = &repo.get_conn();
    let user_name = Username().fake::<String>();
    let full_name = get_fake_fullname();
    let email = SafeEmail().fake::<String>();
    
    let company_create_result = repo.create_company(conn, NewCompany{ name: CompanyName().fake::<String>() }).await.unwrap();
    let company_id = company_create_result.id;

    let create_result = repo.create_employer(conn, NewEmployer {
        user_name: user_name.clone(),
        full_name: full_name.clone(),
        email: email.clone(),
        company_id
    }).await.unwrap();
    let get_result = repo.get_employer(conn, create_result.id).await.unwrap().unwrap();
    
    assert!(get_result.clone().id == create_result.id);
    assert!(get_result.clone().user_name == user_name);
    assert!(get_result.clone().full_name == full_name);
    assert!(get_result.clone().email == email);
    assert!(get_result.clone().company_id == company_id);
}

#[tokio::test]
async fn test_create_two_employers_and_get_back_both() {
    init_fixtures();
    let repo = DbRepo::init().await;
    let conn = &repo.get_conn();

    let company_create_result = repo.create_company(conn, NewCompany{ name: CompanyName().fake::<String>() }).await.unwrap();
    let company_id = company_create_result.id;

    let create_result1 = repo.create_employer(conn, NewEmployer {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: SafeEmail().fake::<String>(),
        company_id
    }).await.unwrap();
    let create_result2 = repo.create_employer(conn, NewEmployer {
        user_name: Username().fake::<String>(),
        full_name: get_fake_fullname(),
        email: SafeEmail().fake::<String>(),
        company_id
    }).await.unwrap();

    let get_all_result = repo.get_all_employers(conn, 10, 0).await.unwrap();
    
    assert!(get_all_result.iter().find(|dev| {
        dev.id == create_result1.id
    }).is_some());
    assert!(get_all_result.iter().find(|dev| {
        dev.id == create_result2.id
    }).is_some());
}