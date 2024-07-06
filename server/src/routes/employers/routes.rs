use actix_web::{web::{Data, Json, Path}, HttpRequest};
use crate::{
    app_state::AppState, common::{
        authentication::auth_keys_service::Authenticator, repository::{
            base::Repository, 
            companies::{models::NewCompany, repo::InsertCompanyFn}, 
            countries::repo::QueryAllCountriesFn, 
            developers::repo::QueryDeveloperFn, 
            employers::{models::{NewEmployer, UpdateEmployer}, repo::{InsertEmployerFn, QueryAllEmployersFn, QueryEmployerByEmailFn, QueryEmployerByUsernameFn, QueryEmployerFn, UpdateEmployerFn}}
        }
    }, routes::{auth_helper::check_is_authenticated, base_model::{OutputBool, OutputId, PagingModel}, route_utils::get_header_strings, user_error::UserError}
};
use super::models::{EmployerResponder, EmployerResponders, NewEmployerForRoute, UpdateEmployerForRoute};
use crate::routes::authentication::models::DeveloperOrEmployer as AuthDeveloperOrEmployer;
use log::error;

/// register a new employer profile
pub async fn create_employer<T: QueryAllCountriesFn + InsertCompanyFn + QueryEmployerByUsernameFn + QueryEmployerByEmailFn + InsertEmployerFn + Repository, U: Authenticator>(
    app_data: Data<AppState<T, U>>, 
    json: Json<NewEmployerForRoute>
) -> Result<OutputId, UserError> {
    match app_data.repo.query_employer_by_email(json.email.clone()).await {
        Ok(result) => match result {
            Some(_) => {
                return Err(UserError::EmailAlreadyInUse);
            },
            None => ()
        },
        Err(_) => ()
    };
    match app_data.repo.query_employer_by_user_name(json.user_name.clone()).await {
        Ok(result) => match result {
            Some(_) => {
                return Err(UserError::UsernameAlreadyInUse);
            },
            None => ()
        },
        Err(_) => ()
    };

    if (json.company_id.is_none() && json.new_company_name.clone().is_none()) || (json.company_id.is_some() && json.new_company_name.is_some()){
        println!("company_id and new_company_name cannot both have or not have values");
        return Err(UserError::ValidationError { field: "company_id".to_string() });
    }

    let new_emp = if let Some(company_id) = json.company_id {
        NewEmployer {
            user_name: json.user_name.to_owned(),
            full_name: json.full_name.to_owned(),
            email: json.email.to_owned(),
            password: json.password.to_owned(),
            company_id
        }
    } else {
        let countries = app_data.repo.query_all_countries().await.unwrap();
        match app_data.repo.insert_company(NewCompany { 
            name: json.new_company_name.as_ref().unwrap().to_owned(), 
            logo: None, 
            headquarters_country_id: countries[0].id 
        }).await {
            Ok(company_entity) => {
                NewEmployer {
                    user_name: json.user_name.to_owned(),
                    full_name: json.full_name.to_owned(),
                    email: json.email.to_owned(),
                    password: json.password.to_owned(),
                    company_id: company_entity.id
                }
            },
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.is_unique_violation() {
                        return Err(UserError::CompanyNameAlreadyInUse);
                    }
                }
                return Err(e.into());
            }
        }        
    };

    let result = app_data.repo.insert_employer(new_emp).await;
    match result {
        Ok(entity) => Ok(OutputId { id: entity.id }),
        Err(e) => {
            error!("create emp error {}", e);
            Err(e.into())
        }
    }
}

pub async fn update_employer<T: QueryAllCountriesFn + QueryEmployerByEmailFn + InsertCompanyFn + QueryDeveloperFn + QueryEmployerFn + UpdateEmployerFn + Repository, U: Authenticator>(
    app_data: Data<AppState<T, U>>, 
    json: Json<UpdateEmployerForRoute>,
    req: HttpRequest
) -> Result<OutputBool, UserError> {
    let is_auth = check_is_authenticated(app_data.clone(), json.id, AuthDeveloperOrEmployer::Employer, req).await;
    if !is_auth {
        error!("Authorization failed");
        return Err(UserError::AuthenticationFailed);
    }

    if (json.company_id.is_none() && json.new_company_name.clone().is_none()) || (json.company_id.is_some() && json.new_company_name.is_some()){
        return Err(UserError::ValidationError { field: "company_id".to_string() });
    }

    let updated_employer = if let Some(company_id) = json.company_id {
        UpdateEmployer {
            id: json.id,
            full_name: json.full_name.to_owned(),
            email: json.email.to_owned(),
            company_id
        }        
    } else {
        let countries = app_data.repo.query_all_countries().await.unwrap();
        match app_data.repo.insert_company(NewCompany { 
            name: json.new_company_name.as_ref().unwrap().to_owned(), 
            logo: None, 
            headquarters_country_id: countries[0].id 
        }).await {
            Ok(company_entity) => {
                UpdateEmployer {
                    id: json.id,
                    full_name: json.full_name.to_owned(),
                    email: json.email.to_owned(),
                    company_id: company_entity.id
                }
            },
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.is_unique_violation() {
                        return Err(UserError::CompanyNameAlreadyInUse);
                    }
                }
                return Err(e.into());
            }
        }                  
    };
    
    let result = app_data.repo.update_employer(updated_employer).await;
    match result {
        Ok(_) => {
            // todo: if email changed need to create a confirm email record
            Ok(OutputBool { result: true })
        },
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

pub async fn get_employer_by_email<T: QueryEmployerByEmailFn + Repository, U: Authenticator>(
    app_data: Data<AppState<T, U>>, 
    path: Path<String>,
    req: HttpRequest
) -> Result<Option<EmployerResponder>, UserError> {
    let email = path.into_inner();
    let result = app_data.repo.query_employer_by_email(email).await;

    match result {
        Ok(opt_emp) => match opt_emp {
            Some(emp) => {
                let headers = get_header_strings(req.headers());
                let authenticated = app_data.auth_service.is_authenticated(emp.user_name.clone(), headers, &app_data.auth_keys.decoding_key).await;
                match authenticated {
                    Ok(auth) => {
                        if auth {
                            Ok(Some(EmployerResponder { 
                                id: emp.id, 
                                updated_at: emp.updated_at, 
                                user_name: emp.user_name.to_owned(), 
                                full_name: emp.full_name.to_owned(), 
                                email: emp.email.to_owned(), 
                                company_id: emp.company_id
                            }))
                        } else {
                            Err(UserError::AuthenticationFailed)
                        }
                    },
                    Err(_) => Err(UserError::InternalError)
                }
            },
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
    use crate::{
        common::{
            authentication::auth_keys_service::AuthenticationError, 
            repository::{
                base::EntityId, employers::{models::Employer, repo::QueryAllEmployersFn}, user::models::DeveloperOrEmployer
            }
        }, 
        common_test::fixtures::{get_app_data, get_fake_email, get_fake_fullname, get_fake_httprequest_with_bearer_token, MockDbRepo}
    };
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
            Ok(EntityId { id: 1 })
        }
    }

    #[async_trait]
    impl UpdateEmployerFn for MockDbRepo {
        async fn update_employer(&self, _: UpdateEmployer) -> Result<(), sqlx::Error> {
            Ok(())
        }
    }

    #[async_trait]
    impl QueryEmployerFn for MockDbRepo {
        async fn query_employer(&self, _: i64) -> Result<Option<Employer>, sqlx::Error> {
            Ok(Some(Employer::new(
                1,
                Utc::now(),
                Utc::now(),
                Username().fake::<String>(),
                get_fake_fullname(),
                "".to_string(),
                FreeEmail().fake::<String>(),                
                1
            )))
        }
    }

    #[async_trait]
    impl QueryAllEmployersFn for MockDbRepo {
        async fn query_all_employers(&self, _: i32, _: i64) -> Result<Vec<Employer>, sqlx::Error> {
            Ok(vec![
              Employer::new(
                    1,
                    Utc::now(),
                    Utc::now(),
                    Username().fake::<String>(),
                    get_fake_fullname(),
                    "".to_string(),
                    FreeEmail().fake::<String>(),                    
                    1
                )
            ])
        }
    }

    mod mod_create_employer_route {
        use super::*;

        #[async_trait]
        impl QueryEmployerByEmailFn for MockDbRepo {
            async fn query_employer_by_email(&self, _: String) -> Result<Option<Employer>, sqlx::Error> {
                Ok(Some(Employer::new(
                    1,
                    Utc::now(),
                    Utc::now(),
                    Username().fake::<String>(),
                    get_fake_fullname(),                
                    "".to_string(),
                    FreeEmail().fake::<String>(),
                    1
                )))
            }
        }

        #[async_trait]
        impl QueryEmployerByUsernameFn for MockDbRepo {
            async fn query_employer_by_user_name(&self, _: String) -> Result<Option<Employer>, sqlx::Error> {
                Ok(Some(Employer::new(
                    1,
                    Utc::now(),
                    Utc::now(),
                    Username().fake::<String>(),
                    get_fake_fullname(),                
                    "".to_string(),
                    FreeEmail().fake::<String>(),
                    1
                )))
            }
        }

        #[tokio::test]
        async fn test_create_employer_route() {
            let repo = MockDbRepo::init().await;
            let auth_service = MockAuthService;
            let app_data = get_app_data(repo, auth_service).await;        
            let email = get_fake_email();
    
            let result = create_employer(app_data, Json(NewEmployerForRoute { 
                user_name: Username().fake::<String>(), 
                full_name: get_fake_fullname(), 
                email, 
                password: "test1234".to_string(),
                company_id: Some(1),
                new_company_name: None
            })).await;
            assert!(result.err().unwrap() == UserError::EmailAlreadyInUse)
        }
    }

    #[tokio::test]
    async fn test_update_employer_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;        
        let email = get_fake_email();
        let req = get_fake_httprequest_with_bearer_token("linda".to_string(), DeveloperOrEmployer::Employer, &app_data.auth_keys.encoding_key, "/employer_email/{email}", "lshin@AmazingAndCo.com".to_string(), None, None);

        let result = update_employer(app_data, Json(UpdateEmployerForRoute { 
            id: 1, 
            full_name: get_fake_fullname(), 
            email, 
            company_id: Some(1),
            new_company_name: None
        }),
        req).await;

        assert!(result.is_ok())
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
    async fn test_get_employer_by_email_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let req = get_fake_httprequest_with_bearer_token("linda".to_string(), DeveloperOrEmployer::Employer, &app_data.auth_keys.encoding_key, "/employer_email/{email}", "lshin@AmazingAndCo.com".to_string(), None, None);
        let result = get_employer_by_email(app_data, Path::from("lshin@AmazingAndCo.com".to_string()), req).await.unwrap().unwrap();

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