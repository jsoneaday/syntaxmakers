use actix_web::web::Data;
use crate::{routes::user_error::UserError, app_state::AppState, common::{repository::{industries::repo::QueryAllIndustriesFn, base::Repository}, authentication::auth_keys_service::Authenticator}};
use super::models::{IndustryResponders, IndustryResponder};

#[allow(unused)]
pub async fn get_all_industries<T: QueryAllIndustriesFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>) -> Result<IndustryResponders, UserError>{
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
    use crate::{common::{repository::industries::{repo::QueryAllIndustriesFn, models::Industry}, authentication::auth_keys_service::AuthenticationError}, common_test::fixtures::{MockDbRepo, get_app_data}};

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
        let app_data = get_app_data(repo, auth_service).await;

        let result = get_all_industries(app_data).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }
}