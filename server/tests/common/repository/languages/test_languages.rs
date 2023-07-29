use syntaxmakers_server::common::repository::base::{Repository, DbRepo};
use syntaxmakers_server::common::repository::languages::repo::QueryAllLanguagesFn;
use syntaxmakers_server::common_test::fixtures::{LANGUAGE_NAMES, init_fixtures};

#[tokio::test]
async fn test_get_all_languages() {
    init_fixtures();
    let repo = DbRepo::init().await;

    let languages = LANGUAGE_NAMES.get().unwrap();

    let result = repo.query_all_languages().await.unwrap();
    
    result.iter().for_each(|language| {
        assert!(languages.iter().find(|name| name.to_string() == language.name) != None);
    });
}