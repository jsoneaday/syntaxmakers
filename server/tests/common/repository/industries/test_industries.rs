use syntaxmakers_server::common::repository::base::{ConnGetter, DbRepo};
use syntaxmakers_server::common::repository::industries::repo::GetAllIndustriesFn;
use syntaxmakers_server::common_test::fixtures::{INDUSTRY_NAMES, init_fixtures};

#[tokio::test]
async fn test_get_all_industries() {
    init_fixtures();
    let repo = DbRepo::init().await;

    let industries = INDUSTRY_NAMES.get().unwrap();

    let result = repo.get_all_industries(&repo.get_conn()).await.unwrap();
    
    result.iter().for_each(|industry| {
        assert!(industries.iter().find(|name| name.to_string() == industry.name) != None);
    });
}