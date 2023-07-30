use actix_web::web::Data;
use crate::{
    common::repository::{base::Repository, salaries::repo::QueryAllSalariesFn}, 
    app_state::AppState, 
    routes::user_error::UserError
};
use super::models::{SalaryResponder, SalaryResponders};

pub async fn get_all_salaries<T: QueryAllSalariesFn + Repository>(
    app_data: Data<AppState<T>>
) -> Result<SalaryResponders, UserError> {
    let result = app_data.repo.query_all_salaries().await;

    match result {
        Ok(salaries) => {
            let responders = salaries.iter().map(|lang| {
                SalaryResponder {
                    id: lang.id,
                    updated_at: lang.updated_at,
                    base: 200000
                }
            })
            .collect::<Vec<SalaryResponder>>();
            Ok(SalaryResponders(responders))
        },
        Err(e) => Err(e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{common_test::fixtures::{MockDbRepo, get_app_data}, common::repository::salaries::models::Salary};
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;

    #[async_trait]
    impl QueryAllSalariesFn for MockDbRepo {
        async fn query_all_salaries(&self) -> Result<Vec<Salary>, sqlx::Error> {
            Ok(vec![
                Salary {
                    id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    base: 200000
                }
            ])
        }
    }
    
    #[tokio::test]
    async fn test_get_all_salaries_route() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let result = get_all_salaries(app_data).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }
}