use actix_web::web::{Json, Data};
use crate::{common::repository::{companies::{repo::{InsertCompanyFn, QueryAllCompaniesFn}, models::NewCompany}, base::Repository}, app_state::AppState, routes::{base_model::OutputId, user_error::UserError}};
use super::models::{NewCompanyForRoute, CompanyResponder, CompanyResponders};

pub async fn create_company<T: InsertCompanyFn + Repository>(
    app_state: Data<AppState<T>>, 
    json: Json<NewCompanyForRoute>
) -> Result<OutputId, UserError> {
    let result = app_state.repo.insert_company(NewCompany { name: json.name.clone(), logo: json.logo.clone(), headquarters_country_id: json.headquarters_country_id }).await;

    match result {
        Ok(entity) => Ok(OutputId { id: entity.id }),
        Err(e) => Err(e.into())
    }
}

pub async fn get_all_companies<T: QueryAllCompaniesFn + Repository>(app_state: Data<AppState<T>>) -> Result<CompanyResponders, UserError> {
    let result = app_state.repo.query_all_companies().await;

    match result {
        Ok(companies) => {
            let companyresponders = companies.iter().map(|company| {
                CompanyResponder {
                    id: company.id,
                    updated_at: company.updated_at,
                    name: company.name.to_owned(),
                    logo: company.logo.clone(),
                    headquarters_country_id: company.headquarters_country_id
                }
            }).collect::<Vec<CompanyResponder>>();

            Ok(CompanyResponders(companyresponders))
        },
        Err(e) => Err(e.into())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::Error as SqlxError;
    use chrono::Utc;
    use super::*;
    use crate::{
        common::repository::{companies::{repo::{InsertCompanyFn, QueryAllCompaniesFn}, models::Company}, base::EntityId}, 
        common_test::fixtures::{get_app_data, MockDbRepo}
    };
    use async_trait::async_trait;
    use fake::{faker::company::en::CompanyName, Fake};

    const ID: i64 = 1;

    #[async_trait]
    impl InsertCompanyFn for MockDbRepo {
        async fn insert_company(&self, _: NewCompany) -> Result<EntityId, SqlxError> {
            Ok(EntityId { id: ID })
        }
    }

    #[async_trait]
    impl QueryAllCompaniesFn for MockDbRepo {
        async fn query_all_companies(&self) -> Result<Vec<Company>, SqlxError> {
            Ok(vec![
                Company { id: ID, created_at: Utc::now(), updated_at: Utc::now(), name: CompanyName().fake::<String>(), logo: None, headquarters_country_id: 1 }
            ])
        }
    }

    #[tokio::test]
    async fn test_create_company_route() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let output = create_company(app_data, Json(NewCompanyForRoute{ name: CompanyName().fake::<String>(), logo: None, headquarters_country_id: 1 })).await.unwrap();
        
        assert!(output.id == ID);
    }

    #[tokio::test]
    async fn test_get_all_companies_route() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let result = get_all_companies(app_data).await.unwrap();

        assert!(result.0.get(0).unwrap().id == ID);
    }
}