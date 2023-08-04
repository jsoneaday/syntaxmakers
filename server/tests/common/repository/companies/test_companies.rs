use fake::{faker::company::en::CompanyName, Fake};
use syntaxmakers_server::{
    common::repository::{base::{DbRepo, Repository}, companies::{repo::{InsertCompanyFn, QueryAllCompaniesFn}, models::NewCompany}}, 
    common_test::fixtures::init_fixtures
};


#[tokio::test]
async fn test_create_companies_and_get_back() {
    init_fixtures();
    let repo = DbRepo::init().await;
    
    let company_name = CompanyName().fake::<String>();
    let company_create_result = repo.insert_company(NewCompany{ name: company_name.clone(), logo: None, headquarters_country_id: 1 }).await.unwrap();
    let company_id = company_create_result.id;

    let get_result = repo.query_all_companies().await.unwrap();
    
    let matching_co = get_result.iter().find(|co| { co.id == company_id });
    assert!(matching_co.is_some());
    assert!(matching_co.unwrap().name == company_name);
}