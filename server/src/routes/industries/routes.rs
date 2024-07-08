use actix_web::web::Data;
use crate::{app_state::AppState, common::{authentication::auth_keys_service::Authenticator, emailer::emailer::EmailerService, repository::{base::Repository, industries::repo::QueryAllIndustriesFn}}, routes::user_error::UserError};
use super::models::{IndustryResponders, IndustryResponder};

#[allow(unused)]
pub async fn get_all_industries<T: QueryAllIndustriesFn + Repository, E: EmailerService, U: Authenticator>(app_data: Data<AppState<T, E, U>>) -> Result<IndustryResponders, UserError>{
    let result = app_data.repo.query_all_industries().await;

    match result {
        Ok(industries) => {
            let industry_responders = industries.iter().map(|ind| {
                IndustryResponder {
                    id: ind.id,
                    updated_at: ind.updated_at,
                    name: ind.name.to_owned()
                }
            })
            .collect::<Vec<IndustryResponder>>();
            Ok(IndustryResponders(industry_responders))
        },
        Err(e) => Err(e.into())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use fake::{faker::address::en::CountryName, Fake};
    use async_trait::async_trait;
    use jsonwebtoken::DecodingKey;
    use super::*;
    use crate::{
        common::{authentication::auth_keys_service::AuthenticationError, repository::industries::{models::Industry, repo::QueryAllIndustriesFn}}, 
        common_test::fixtures::{get_app_data, MockDbRepo, MockEmailer}
    };

    struct MockAuthService;
    #[async_trait]
    impl Authenticator for MockAuthService {
        async fn is_authenticated(&self, _: String, _: Vec<(&str, &str)>, _: &DecodingKey) -> Result<bool, AuthenticationError> {
            Ok(true)
        }
    }

    #[async_trait]
    impl QueryAllIndustriesFn for MockDbRepo {
        async fn query_all_industries(&self) -> Result<Vec<Industry>, sqlx::Error> {
            Ok(vec![
                Industry {
                    id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    name: CountryName().fake::<String>()
                }
            ])
        }
    }

    #[tokio::test]
    async fn test_get_all_industries_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await;

        let result = get_all_industries(app_data).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }
}