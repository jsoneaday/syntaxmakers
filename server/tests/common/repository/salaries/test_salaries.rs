use syntaxmakers_server::common::repository::base::{Repository, DbRepo};
use syntaxmakers_server::common::repository::salaries::repo::QueryAllSalariesFn;
use syntaxmakers_server::common_test::fixtures::init_fixtures;

#[tokio::test]
async fn test_get_all_salaries() {
    let repo = DbRepo::init().await;
    init_fixtures().await;
    let result = repo.query_all_salaries().await.unwrap();
    
    assert!(result.len() > 1);
}