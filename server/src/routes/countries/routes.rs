use crate::{common::repository::{countries::repo::QueryAllCountriesFn, base::Repository}, app_state::AppState, routes::user_error::UserError};
use actix_web::web::Data;
use super::models::{CountryResponder, CountryResponders};

pub async fn get_all_countries<T: QueryAllCountriesFn + Repository>(app_state: Data<AppState<T>>) -> Result<CountryResponders, UserError> {
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
    use crate::{common::repository::{base::Repository, countries::models::Country}, common_test::fixtures::{get_app_data, MockDbRepo}};
    use async_trait::async_trait;
    use chrono::Utc;
    use fake::{faker::address::en::CountryName, Fake};

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
    async fn test_get_all_companies_route() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let countries = get_all_countries(app_data).await.unwrap();
        assert!(countries.0.get(0).unwrap().id == 1);
    }
}