use syntaxmakers_server::common::repository::base::{DbRepo, ConnGetter};
use syntaxmakers_server::common::repository::countries::repo::GetAllCountriesFn;
use syntaxmakers_server::common_test::fixtures::{COUNTRY_NAMES, init_fixtures};

#[tokio::test]
async fn test_get_all_countries() {
    init_fixtures();
    let repo = DbRepo::init().await;

    let result = repo.get_all_countrie(&repo.get_conn()).await.unwrap();
    let countries = COUNTRY_NAMES.get().unwrap();
    
    result.iter().for_each(|country| {
        assert!(countries.iter().find(|name| name.to_string() == country.name) != None);
    });
}