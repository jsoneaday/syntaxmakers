use actix_web::web::{Data, Json, Path};
use crate::{
    common::{repository::{employers::{repo::{InsertEmployerFn, QueryEmployerFn, QueryAllEmployersFn}, models::NewEmployer}, base::Repository}, authentication::auth_service::Authenticator}, 
    routes::{base_model::{OutputId, PagingModel}, user_error::UserError}, 
    app_state::AppState
};
use super::models::{NewEmployerForRoute, EmployerResponder, EmployerResponders};

pub async fn create_employer<T: InsertEmployerFn + Repository, U: Authenticator>(
    app_data: Data<AppState<T, U>>, 
    json: Json<NewEmployerForRoute>
) -> Result<OutputId, UserError> {
    let result = app_data.repo.insert_employer(NewEmployer {
        user_name: json.user_name.to_owned(),
        full_name: json.full_name.to_owned(),
        email: json.email.to_owned(),
        password: json.password.to_owned(),
        company_id: json.company_id
    }).await;

    match result {
        Ok(entity) => Ok(OutputId { id: entity.id }),
        Err(e) => Err(e.into())
    }
}

pub async fn get_employer<T: QueryEmployerFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, path: Path<i64>) -> Result<Option<EmployerResponder>, UserError> {
    let result = app_data.repo.query_employer(path.into_inner()).await;

    match result {
        Ok(opt_employer) => match opt_employer {
            Some(employer) => Ok(Some(EmployerResponder {
                id: employer.id,
                updated_at: employer.updated_at,
                user_name: employer.user_name,
                full_name: employer.full_name,
                email: employer.email.to_string(),
                company_id: employer.company_id
            })),
            None => Ok(None)
        },
        Err(e) => Err(e.into())
    }
}

pub async fn get_all_employers<T: QueryAllEmployersFn + Repository, U: Authenticator>(
    app_data: Data<AppState<T, U>>, 
    json: Json<PagingModel>
) -> Result<EmployerResponders, UserError> {
    let result = app_data.repo.query_all_employers(json.page_size, json.last_offset).await;

    match result {
        Ok(employers) => {
            let emp_responders = employers.iter().map(|employer| {
                EmployerResponder {
                    id: employer.id,
                    updated_at: employer.updated_at,
                    user_name: employer.user_name.to_owned(),
                    full_name: employer.full_name.to_owned(),
                    email: employer.email.to_string(),
                    company_id: employer.company_id
                }
            })
            .collect::<Vec<EmployerResponder>>();
            Ok(EmployerResponders(emp_responders))
        },
        Err(e) => Err(e.into())
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::{common_test::fixtures::{MockDbRepo, get_app_data, get_fake_fullname}, common::{repository::{base::EntityId, employers::{models::Employer, repo::QueryAllEmployersFn}}, authentication::auth_service::AuthenticationError}};
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use fake::{faker::internet::en::{Username, FreeEmail}, Fake};
    use jsonwebtoken::DecodingKey;

    struct MockAuthService;
    #[async_trait]
    impl Authenticator for MockAuthService {
        async fn is_authenticated(&self, _: String, _: Vec<(&str, &str)>, _: &DecodingKey) -> Result<bool, AuthenticationError> {
            Ok(true)
        }
    }

    #[async_trait]
    impl InsertEmployerFn for MockDbRepo {
        async fn insert_employer(&self, _: NewEmployer) -> Result<EntityId, sqlx::Error> {
            Ok(EntityId {id: 1 })
        }
    }

    #[async_trait]
    impl QueryEmployerFn for MockDbRepo {
        async fn query_employer(&self, _: i64) -> Result<Option<Employer>, sqlx::Error> {
            Ok(Some(Employer {
                id: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                user_name: Username().fake::<String>(),
                full_name: get_fake_fullname(),
                email: FreeEmail().fake::<String>(),
                company_id: 1
            }))
        }
    }

    #[async_trait]
    impl QueryAllEmployersFn for MockDbRepo {
        async fn query_all_employers(&self, _: i32, _: i64) -> Result<Vec<Employer>, sqlx::Error> {
            Ok(vec![
              Employer {
                id: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                user_name: Username().fake::<String>(),
                full_name: get_fake_fullname(),
                email: FreeEmail().fake::<String>(),
                company_id: 1
              }  
            ])
        }
    }

    #[tokio::test]
    async fn test_insert_employer_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let result = create_employer(app_data, Json(NewEmployerForRoute { 
            user_name: Username().fake::<String>(), 
            full_name: get_fake_fullname(), 
            email: FreeEmail().fake::<String>(), 
            password: "test123".to_string(),
            company_id: 1 
        })).await.unwrap();

        assert!(result.id == 1)
    }

    #[tokio::test]
    async fn test_get_employer_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let result = get_employer(app_data, Path::from(1)).await.unwrap().unwrap();

        assert!(result.id == 1);
    }

    #[tokio::test]
    async fn test_get_all_employers_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let result = get_all_employers(app_data, Json(PagingModel { page_size: 10, last_offset: 1 })).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }
}