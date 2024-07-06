use actix_web::web::Data;
use crate::{
    common::{repository::{base::Repository, salaries::repo::QueryAllSalariesFn}, authentication::auth_keys_service::Authenticator}, 
    app_state::AppState, 
    routes::user_error::UserError
};
use super::models::{SalaryResponder, SalaryResponders};

pub async fn get_all_salaries<T: QueryAllSalariesFn + Repository, U: Authenticator>(
    app_data: Data<AppState<T, U>>
) -> Result<SalaryResponders, UserError> {
    let result = app_data.repo.query_all_salaries().await;

    match result {
        Ok(salaries) => {
            let responders = salaries.iter().map(|lang| {
                SalaryResponder {
                    id: lang.id,
                    updated_at: lang.updated_at,
                    base: lang.base
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
    use crate::{common_test::fixtures::{MockDbRepo, get_app_data}, common::{repository::salaries::models::Salary, authentication::auth_keys_service::AuthenticationError}};
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use jsonwebtoken::DecodingKey;

    struct MockAuthService;
    
    #[async_trait]
    impl Authenticator for MockAuthService {
        async fn is_authenticated(&self, _: String, _: Vec<(&str, &str)>, _: &DecodingKey) -> Result<bool, AuthenticationError> {
            Ok(true)
        }
    }

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
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let result = get_all_salaries(app_data).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }
}