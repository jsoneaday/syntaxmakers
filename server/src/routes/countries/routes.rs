use crate::{common::{repository::{countries::repo::QueryAllCountriesFn, base::Repository}, authentication::auth_service::Authenticator}, app_state::AppState, routes::user_error::UserError};
use actix_web::web::Data;
use super::models::{CountryResponder, CountryResponders};

pub async fn get_all_countries<T: QueryAllCountriesFn + Repository, U: Authenticator>(app_state: Data<AppState<T, U>>) -> Result<CountryResponders, UserError> {
    let result = app_state.repo.query_all_countries().await;

    match result {
        Ok(countries) => {
            let country_responders = countries
            .iter()
            .map(|country| CountryResponder { id: country.id, updated_at: country.updated_at, name: country.name.to_owned() })
            .collect::<Vec<CountryResponder>>();

            Ok(CountryResponders(country_responders))
        },
        Err(e) => Err(e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{common::{repository::{base::Repository, countries::models::Country}, authentication::auth_service::AuthenticationError}, common_test::fixtures::{get_app_data, MockDbRepo}};
    use async_trait::async_trait;
    use chrono::Utc;
    use fake::{faker::address::en::CountryName, Fake};
    use jsonwebtoken::DecodingKey;

    struct MockAuthService;
    #[async_trait]
    impl Authenticator for MockAuthService {
        async fn is_authenticated(&self, _: String, _: Vec<(&str, &str)>, _: &DecodingKey) -> Result<bool, AuthenticationError> {
            Ok(true)
        }
    }

    #[async_trait]
    impl QueryAllCountriesFn for MockDbRepo {
        async fn query_all_countries(&self) -> Result<Vec<Country>, sqlx::Error> {
            Ok(vec![
                Country {
                    id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    name: CountryName().fake::<String>()
                }
            ])
        }
    }

    #[tokio::test]
    async fn test_get_all_countries_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let countries = get_all_countries(app_data).await.unwrap();
        assert!(countries.0.get(0).unwrap().id == 1);
    }
}