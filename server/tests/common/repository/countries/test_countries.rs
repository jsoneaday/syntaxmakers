use syntaxmakers_server::common::repository::base::{DbRepo, Repository};
use syntaxmakers_server::common::repository::countries::repo::QueryAllCountriesFn;
use syntaxmakers_server::common_test::fixtures::{COUNTRY_NAMES, init_fixtures};

#[tokio::test]
async fn test_get_all_countries() {
    init_fixtures();
    let repo = DbRepo::init().await;

    let result = repo.query_all_countries().await.unwrap();
    let countries = COUNTRY_NAMES.get().unwrap();
    
    result.iter().for_each(|country| {
        assert!(countries.iter().find(|name| name.to_string() == country.name) != None);
    });
}