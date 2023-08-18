use syntaxmakers_server::common::repository::base::{Repository, DbRepo};
use syntaxmakers_server::common::repository::languages::repo::QueryAllLanguagesFn;
use syntaxmakers_server::common_test::fixtures::init_fixtures;

#[tokio::test]
async fn test_get_all_languages() {
    init_fixtures();
    let repo = DbRepo::init().await;

    let result = repo.query_all_languages().await.unwrap();
    
    assert!(result.len() > 1)
}