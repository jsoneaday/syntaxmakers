use syntaxmakers_server::common::repository::base::{ConnGetter, DbRepo};
use syntaxmakers_server::common::repository::salaries::repo::GetAllSalariesFn;
use syntaxmakers_server::common_test::fixtures::{ init_fixtures, SALARY_BASE};

#[tokio::test]
async fn test_get_all_salaries() {
    init_fixtures();
    let repo = DbRepo::init().await;

    let salaries = SALARY_BASE.get().unwrap();

    let result = repo.get_all_salaries(&repo.get_conn()).await.unwrap();
    
    result.iter().for_each(|salary| {
        assert!(salaries.iter().find(|base| **base == salary.base) != None);
    });
}