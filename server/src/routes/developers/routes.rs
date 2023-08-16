use actix_web::{web::{Data, Json, Path}, HttpRequest};
use crate::{
    app_state::AppState, 
    common::repository::{developers::{repo::{InsertDeveloperFn, QueryDeveloperFn, QueryAllDevelopersFn, QueryDeveloperByEmailFn}, models::NewDeveloper}, base::Repository}, 
    routes::{base_model::{OutputId, IdAndPagingModel}, user_error::UserError, authentication::routes::is_authenticated}
};
use super::models::{NewDeveloperForRoute, DeveloperResponder, DeveloperResponders};

pub async fn create_developer<T: InsertDeveloperFn + Repository>(
    app_data: Data<AppState<T>>, 
    json: Json<NewDeveloperForRoute>
) -> Result<OutputId, UserError> {
    let result = app_data.repo.insert_developer(NewDeveloper {
        user_name: json.user_name.to_owned(),
        full_name: json.full_name.to_owned(),
        email: json.email.to_owned(),
        password: json.password.to_owned(),
        primary_lang_id: json.primary_lang_id,
        secondary_lang_id: json.secondary_lang_id
    }).await;

    match result {
        Ok(entity) => Ok(OutputId { id: entity.id }),
        Err(e) => Err(e.into())
    }
}

pub async fn get_developer<T: QueryDeveloperFn + Repository>(
    app_data: Data<AppState<T>>, 
    path: Path<i64>,
    req: HttpRequest
) -> Result<Option<DeveloperResponder>, UserError> {
    let id = path.into_inner();
    let result = app_data.repo.query_developer(id).await;

    match result {
        Ok(optional_dev) => match optional_dev {
            Some(dev) => {
                println!("cookies {:?}", req.cookies());
                let authenticated = is_authenticated(dev.user_name.clone(), req.headers(), &app_data.auth_keys.decoding_key).await;
                match authenticated {
                    Ok(auth) => {
                        if auth {
                            Ok(Some(DeveloperResponder { 
                                id: dev.id, 
                                updated_at: dev.updated_at, 
                                user_name: dev.user_name.to_owned(), 
                                full_name: dev.full_name.to_owned(), 
                                email: dev.email.to_owned(), 
                                primary_lang_id: dev.primary_lang_id,
                                secondary_lang_id: dev.secondary_lang_id
                            }))
                        } else {
                            Err(UserError::InternalError)
                        }
                    },
                    Err(e) => Err(e.into())
                }
            },
            None => Ok(None)
        },
        Err(e) => Err(e.into())
    }
}

pub async fn get_developer_by_email<T: QueryDeveloperByEmailFn + Repository>(
    app_data: Data<AppState<T>>, 
    path: Path<String>,
    req: HttpRequest
) -> Result<Option<DeveloperResponder>, UserError> {    
    let email = path.into_inner();
    println!("get_developer_by_email {}", email);
    let result = app_data.repo.query_developer_by_email(email).await;

    match result {
        Ok(optional_dev) => match optional_dev {
            Some(dev) => {
                let authenticated = is_authenticated(dev.user_name.clone(), req.headers(), &app_data.auth_keys.decoding_key).await;
                match authenticated {
                    Ok(auth) => {
                        if auth {
                            Ok(Some(DeveloperResponder { 
                                id: dev.id, 
                                updated_at: dev.updated_at, 
                                user_name: dev.user_name.to_owned(), 
                                full_name: dev.full_name.to_owned(), 
                                email: dev.email.to_owned(), 
                                primary_lang_id: dev.primary_lang_id,
                                secondary_lang_id: dev.secondary_lang_id
                            }))
                        } else {
                            Err(UserError::InternalError)
                        }
                    },
                    Err(e) => Err(e.into())
                }
            },
            None => Ok(None)
        },
        Err(e) => Err(e.into())
    }
}

pub async fn get_all_developers<T: QueryAllDevelopersFn + QueryDeveloperFn + Repository>(
    app_data: Data<AppState<T>>, 
    json: Json<IdAndPagingModel>,
    req: HttpRequest
) -> Result<DeveloperResponders, UserError> {
    let result = app_data.repo.query_all_developers(json.page_size, json.last_offset).await;

    match result {
        Ok(developers) => {
            let requestor_dev_result = app_data.repo.query_developer(json.id).await;
            match requestor_dev_result {
                Ok(opt_requestor_dev) => {
                    if let Some(requestor_dev) = opt_requestor_dev {
                        let is_auth_result = is_authenticated(requestor_dev.user_name, req.headers(), &app_data.auth_keys.decoding_key).await;
                        
                        match is_auth_result {
                            Ok(is_auth) => {
                                if is_auth {
                                    let devs = developers.iter().map(|dev| {
                                        DeveloperResponder { 
                                            id: dev.id, 
                                            updated_at: dev.updated_at, 
                                            user_name: dev.user_name.to_owned(), 
                                            full_name: dev.full_name.to_owned(), 
                                            email: dev.email.to_owned(), 
                                            primary_lang_id: dev.primary_lang_id,
                                            secondary_lang_id: dev.secondary_lang_id
                                        }
                                    })
                                    .collect::<Vec<DeveloperResponder>>();
                                    Ok(DeveloperResponders(devs))
                                } else {
                                    Err(UserError::InternalError)
                                }
                            },
                            Err(e) => Err(e.into())
                        }
                    } else {
                        Err(UserError::InternalError)
                    }
                },
                Err(e) => Err(e.into())
            }            
        },
        Err(e) => Err(e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{common_test::fixtures::{MockDbRepo, get_app_data, get_fake_fullname, get_fake_httprequest_with_bearer_token}, common::repository::{base::EntityId, developers::models::Developer, user::models::DeveloperOrEmployer}};
    use async_trait::async_trait;
    use chrono::Utc;
    use fake::{faker::internet::en::{Username, FreeEmail}, Fake};
    use super::*;

    const DEV_USERNAME: &str = "tester";

    #[async_trait]
    impl InsertDeveloperFn for MockDbRepo {
        async fn insert_developer(&self, _: NewDeveloper) -> Result<EntityId, sqlx::Error> {
            Ok(EntityId { id: 1 })
        }
    }

    #[async_trait]
    impl QueryDeveloperFn for MockDbRepo {
        async fn query_developer(&self, _: i64) -> Result<Option<Developer>, sqlx::Error> {
            Ok(Some(Developer { 
                id: 1, 
                user_name: DEV_USERNAME.to_string(), 
                created_at: Utc::now(), 
                updated_at: Utc::now(), 
                full_name: get_fake_fullname(), 
                email: FreeEmail().fake::<String>(), 
                primary_lang_id: 1,
                secondary_lang_id: Some(2)
            }))
        }
    }

    #[async_trait]
    impl QueryDeveloperByEmailFn for MockDbRepo {
        async fn query_developer_by_email(&self, _: String) -> Result<Option<Developer>, sqlx::Error> {
            Ok(Some(Developer { 
                id: 1, 
                user_name: DEV_USERNAME.to_string(), 
                created_at: Utc::now(), 
                updated_at: Utc::now(), 
                full_name: get_fake_fullname(), 
                email: FreeEmail().fake::<String>(), 
                primary_lang_id: 1,
                secondary_lang_id: Some(2)
            }))
        }
    }

    #[async_trait]
    impl QueryAllDevelopersFn for MockDbRepo {
        async fn query_all_developers(&self, _: i32, _: i64) -> Result<Vec<Developer>, sqlx::Error> {
            Ok(vec![
                Developer { 
                    id: 1, 
                    user_name: Username().fake::<String>(), 
                    created_at: Utc::now(), 
                    updated_at: Utc::now(), 
                    full_name: get_fake_fullname(), 
                    email: FreeEmail().fake::<String>(), 
                    primary_lang_id: 1,
                    secondary_lang_id: Some(2)
                }
            ])
        }
    }

    #[tokio::test]
    async fn test_insert_developer_route() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let result = app_data.repo.insert_developer(NewDeveloper { 
            user_name: Username().fake::<String>(), 
            full_name: get_fake_fullname(), 
            email: FreeEmail().fake::<String>(), 
            password: "test123".to_string(),
            primary_lang_id: 1, 
            secondary_lang_id: Some(2) 
        }).await.unwrap();

        assert!(result.id == 1);
    }

    #[tokio::test]
    async fn test_get_developer_route() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let req = get_fake_httprequest_with_bearer_token(DEV_USERNAME.to_string(), DeveloperOrEmployer::Developer, &app_data.auth_keys.encoding_key, "/v1/developer", 1, Some(60*2));

        let result = get_developer(app_data, Path::from(1), req).await.unwrap();

        assert!(result.unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_developer_by_email_route() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let req = get_fake_httprequest_with_bearer_token(DEV_USERNAME.to_string(), DeveloperOrEmployer::Developer, &app_data.auth_keys.encoding_key, "/v1/developer", 1, Some(60*2));

        let result = get_developer_by_email(app_data, Path::from("jon@jon.com".to_string()), req).await.unwrap();

        assert!(result.unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_all_developers_route() {
        let repo = MockDbRepo::init().await;
        let app_data = get_app_data(repo).await;

        let parameters = IdAndPagingModel { id: 1, page_size: 10, last_offset: 1 };
        let req = get_fake_httprequest_with_bearer_token(DEV_USERNAME.to_string(), DeveloperOrEmployer::Developer, &app_data.auth_keys.encoding_key, "/v1/developers", parameters.clone(), Some(60*2));

        let result = get_all_developers(app_data, Json(parameters), req).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }
}