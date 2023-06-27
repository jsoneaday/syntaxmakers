use fake::{faker::{ company::en::CompanyName}, Fake};
use syntaxmakers_server::{
    common::repository::{base::{DbRepo, ConnGetter}, companies::{repo::{CreateCompanyFn, GetAllCompaniesFn}, models::NewCompany}}, common_test::fixtures::{ init_fixtures}
};



#[tokio::test]
async fn test_create_companies_and_get_back() {
    init_fixtures();
    let repo = DbRepo::init().await;
    let conn = &repo.get_conn();
    
    let company_create_result = repo.create_company(conn, NewCompany{ name: CompanyName().fake::<String>() }).await.unwrap();
    let company_id = company_create_result.id;

    let get_result = repo.get_all_companies(conn, 10, 0).await.unwrap();
    
    assert!(get_result.iter().find(|co| { co.id == company_id }).is_some());
}