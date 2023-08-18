use syntaxmakers_server::common::repository::base::{DbRepo, Repository};
use syntaxmakers_server::common::repository::countries::repo::QueryAllCountriesFn;
use syntaxmakers_server::common_test::fixtures::init_fixtures;

#[tokio::test]
async fn test_get_all_countries() {
    init_fixtures();
    let repo = DbRepo::init().await;

    let result = repo.query_all_countries().await.unwrap();
    
    assert!(result.get(0).unwrap().name == "United States".to_string())
}