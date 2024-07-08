use actix_web::{web::{Data, Json, Path}, HttpRequest};
use crate::{
    app_state::AppState, common::{
        authentication::auth_keys_service::Authenticator, emailer::emailer::EmailerService, repository::{
            base::Repository, 
            developers::{
                models::{NewDeveloper, UpdateDeveloper}, 
                repo::{InsertDeveloperFn, QueryAllDevelopersFn, QueryDeveloperByEmailFn, QueryDeveloperByUserNameFn, QueryDeveloperFn, UpdateDeveloperFn}
            }, 
            employers::repo::QueryEmployerFn
        }
    }, 
    routes::{auth_helper::check_is_authenticated, base_model::{IdAndPagingModel, OutputBool, OutputId}, route_utils::get_header_strings, user_error::UserError}
};
use super::models::{DeveloperResponder, DeveloperResponders, NewDeveloperForRoute, UpdateDeveloperForRoute};
use crate::routes::authentication::models::DeveloperOrEmployer as AuthDeveloperOrEmployer;
use log::error;

/// register a new developer profile
pub async fn create_developer<
    T: QueryDeveloperByUserNameFn + QueryDeveloperByEmailFn + InsertDeveloperFn<E> + Repository, 
    E: EmailerService + Send + Sync, 
    U: Authenticator
    >(
    app_data: Data<AppState<T, E, U>>, 
    json: Json<NewDeveloperForRoute>
) -> Result<OutputId, UserError> {
    match app_data.repo.query_developer_by_email(json.email.clone()).await {
        Ok(result) => match result {
            Some(_) => {
                return Err(UserError::EmailAlreadyInUse);
            },
            None => ()
        },
        Err(_) => ()
    };
    match app_data.repo.query_developer_by_user_name(json.user_name.clone()).await {
        Ok(result) => match result {
            Some(_) => {
                return Err(UserError::UsernameAlreadyInUse);
            },
            None => ()
        },
        Err(_) => ()
    };

    let result = app_data.repo.insert_developer(NewDeveloper {
        user_name: json.user_name.to_owned(),
        full_name: json.full_name.to_owned(),
        email: json.email.to_owned(),
        description: json.description.to_owned(),
        password: json.password.to_owned(),
        primary_lang_id: json.primary_lang_id,
        secondary_lang_id: json.secondary_lang_id
    }, &app_data.emailer).await;

    match result {
        Ok(entity) => Ok(OutputId { id: entity.id }),
        Err(e) => Err(e.into())
    }
}

pub async fn update_developer<
    T: QueryDeveloperByEmailFn + QueryDeveloperFn + QueryEmployerFn + UpdateDeveloperFn<E> + Repository, 
    E: EmailerService + Send + Sync, 
    U: Authenticator
    >(
    app_data: Data<AppState<T, E, U>>, 
    json: Json<UpdateDeveloperForRoute>,
    req: HttpRequest
) -> Result<OutputBool, UserError> {
    let is_auth = check_is_authenticated(app_data.clone(), json.id, AuthDeveloperOrEmployer::Developer, req).await;
    if !is_auth {
        error!("Authorization failed");
        return Err(UserError::AuthenticationFailed);
    }
    
    let result = app_data.repo.update_developer(UpdateDeveloper {
        id: json.id,
        full_name: json.full_name.to_owned(),
        email: json.email.to_owned(),
        primary_lang_id: json.primary_lang_id,
        secondary_lang_id: json.secondary_lang_id,
        description: json.description.to_owned()
    }, &app_data.emailer).await;

    match result {
        Ok(_) => {
            // todo: if email changed need to create a confirm email record
            Ok(OutputBool { result: true })
        },
        Err(e) => Err(e.into())
    }
}

pub async fn get_developer<T: QueryDeveloperFn + Repository, E: EmailerService, U: Authenticator>(
    app_data: Data<AppState<T, E, U>>, 
    path: Path<i64>,
    req: HttpRequest
) -> Result<Option<DeveloperResponder>, UserError> {    
    let id = path.into_inner();
    let result = app_data.repo.query_developer(id).await;

    match result {
        Ok(optional_dev) => match optional_dev {
            Some(dev) => {
                let headers = get_header_strings(req.headers());
                let authenticated = app_data.auth_service.is_authenticated(dev.user_name.clone(), headers, &app_data.auth_keys.decoding_key).await;
                match authenticated {
                    Ok(auth) => {
                        if auth {
                            Ok(Some(DeveloperResponder { 
                                id: dev.id, 
                                updated_at: dev.updated_at, 
                                user_name: dev.user_name.to_owned(), 
                                full_name: dev.full_name.to_owned(), 
                                email: dev.email.to_owned(), 
                                description: dev.description.to_owned(),
                                primary_lang_id: dev.primary_lang_id,
                                secondary_lang_id: dev.secondary_lang_id
                            }))
                        } else {
                            Err(UserError::AuthenticationFailed)
                        }
                    },
                    Err(e) => {
                        println!("get dev auth error {}", e);
                        Err(UserError::InternalError)
                    }
                }
            },
            None => {
                println!("dev not found by id {}", id);
                Ok(None)
            }
        },
        Err(e) => {
            println!("get dev error {}", e);
            Err(e.into())
        }
    }
}

pub async fn get_developer_by_email<T: QueryDeveloperByEmailFn + Repository, E: EmailerService, U: Authenticator>(
    app_data: Data<AppState<T, E, U>>, 
    path: Path<String>,
    req: HttpRequest
) -> Result<Option<DeveloperResponder>, UserError> {
    let email = path.into_inner();
    let result = app_data.repo.query_developer_by_email(email).await;

    match result {
        Ok(optional_dev) => match optional_dev {
            Some(dev) => {
                let headers = get_header_strings(req.headers());
                let authenticated = app_data.auth_service.is_authenticated(dev.user_name.clone(), headers, &app_data.auth_keys.decoding_key).await;
                match authenticated {
                    Ok(auth) => {
                        if auth {
                            Ok(Some(DeveloperResponder { 
                                id: dev.id, 
                                updated_at: dev.updated_at, 
                                user_name: dev.user_name.to_owned(), 
                                full_name: dev.full_name.to_owned(), 
                                email: dev.email.to_owned(), 
                                description: dev.description.to_owned(),
                                primary_lang_id: dev.primary_lang_id,
                                secondary_lang_id: dev.secondary_lang_id
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

pub async fn get_all_developers<T: QueryAllDevelopersFn + QueryDeveloperFn + Repository, E: EmailerService, U: Authenticator>(
    app_data: Data<AppState<T, E, U>>, 
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
                        let headers = get_header_strings(req.headers());
                        let is_auth_result = app_data.auth_service.is_authenticated(requestor_dev.user_name, headers, &app_data.auth_keys.decoding_key).await;
                        
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
                                            description: dev.description.to_owned(),
                                            primary_lang_id: dev.primary_lang_id,
                                            secondary_lang_id: dev.secondary_lang_id
                                        }
                                    })
                                    .collect::<Vec<DeveloperResponder>>();
                                    Ok(DeveloperResponders(devs))
                                } else {
                                    Err(UserError::AuthenticationFailed)
                                }
                            },
                            Err(_) => Err(UserError::InternalError)
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
    use crate::{
        common::{
            authentication::auth_keys_service::AuthenticationError, 
            repository::{base::EntityId, developers::models::Developer, user::{models::{ChangePassword, DeveloperOrEmployer}, repo::ChangePasswordFn}}
        }, 
        common_test::fixtures::{
            get_app_data, get_fake_dev_desc, get_fake_email, get_fake_fullname, get_fake_httprequest_with_bearer_token, init_fixtures, get_fake_user_name,
            MockEmailer, MockDbRepo, LANGUAGES
        }, 
        routes::user::{models::ChangePasswordRoute, routes::change_password}
    };
    use async_trait::async_trait;
    use chrono::Utc;
    use fake::{faker::internet::en::Username, Fake};
    use jsonwebtoken::DecodingKey;
    use super::*;

    const DEV_USERNAME: &str = "tester";
    struct MockAuthService;
    #[async_trait]
    impl Authenticator for MockAuthService {
        async fn is_authenticated(&self, _: String, _: Vec<(&str, &str)>, _: &DecodingKey) -> Result<bool, AuthenticationError> {
            Ok(true)
        }
    }

    #[async_trait]
    impl QueryDeveloperFn for MockDbRepo {
        async fn query_developer(&self, _: i64) -> Result<Option<Developer>, sqlx::Error> {
            Ok(Some(Developer::new( 
                1,                     
                Utc::now(), 
                Utc::now(), 
                DEV_USERNAME.to_string(), 
                get_fake_fullname(), 
                "".to_string(),
                get_fake_email(),
                get_fake_dev_desc(),
                1,
                Some(2)
            )))
        }
    }

    #[async_trait]
    impl QueryDeveloperByEmailFn for MockDbRepo {
        async fn query_developer_by_email(&self, _: String) -> Result<Option<Developer>, sqlx::Error> {
            Ok(Some(Developer::new( 
                1,                     
                Utc::now(), 
                Utc::now(), 
                DEV_USERNAME.to_string(), 
                get_fake_fullname(), 
                "".to_string(),
                get_fake_email(), 
                get_fake_dev_desc(),
                1,
                Some(2)
            )))
        }
    }

    #[async_trait]
    impl QueryAllDevelopersFn for MockDbRepo {
        async fn query_all_developers(&self, _: i32, _: i64) -> Result<Vec<Developer>, sqlx::Error> {
            Ok(vec![
                Developer::new( 
                    1,                     
                    Utc::now(), 
                    Utc::now(), 
                    Username().fake::<String>(), 
                    get_fake_fullname(), 
                    "".to_string(),
                    get_fake_email(), 
                    get_fake_dev_desc(),
                    1,
                    Some(2)
                )
            ])
        }
    }

    mod mod_create_developer_route {        
        use super::*;

        pub struct CreateDevMockDbRepo;

        #[async_trait]
        impl Repository for CreateDevMockDbRepo {
            async fn init() -> Self {
                CreateDevMockDbRepo
            }
        }

        #[async_trait]
        impl<E: EmailerService + Send + Sync> InsertDeveloperFn<E> for CreateDevMockDbRepo {
            async fn insert_developer(&self, _: NewDeveloper, _emailer: &E) -> Result<EntityId, sqlx::Error> {
                Ok(EntityId { id: 1 })
            }
        }

        #[async_trait]
        impl QueryDeveloperByEmailFn for CreateDevMockDbRepo {
            async fn query_developer_by_email(&self, _: String) -> Result<Option<Developer>, sqlx::Error> {
                Ok(None)
            }
        }

        #[async_trait]
        impl QueryDeveloperByUserNameFn for CreateDevMockDbRepo {
            async fn query_developer_by_user_name(&self, _: String) -> Result<Option<Developer>, sqlx::Error> {
                Ok(None)
            }
        }

        #[tokio::test]
        async fn test_create_developer_route() {
            let repo = CreateDevMockDbRepo::init().await;
            init_fixtures().await;
            let auth_service = MockAuthService;
            let emailer = MockEmailer;
            let app_data = get_app_data(repo, emailer, auth_service).await; 

            let result = create_developer(app_data, Json(NewDeveloperForRoute { 
                user_name: get_fake_user_name(), 
                full_name: get_fake_fullname(), 
                email: get_fake_email(), 
                password: "test1234".to_string(),
                primary_lang_id: LANGUAGES.get().unwrap()[0].id, 
                secondary_lang_id: None,
                description: get_fake_dev_desc()
            })).await;

            assert!(result.unwrap().id == 1);
        }
    }


    #[async_trait]
    impl ChangePasswordFn for MockDbRepo {
        async fn change_password(&self, _: ChangePassword) -> Result<(), sqlx::Error> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_change_dev_password_route() {
        let repo = MockDbRepo::init().await;
        init_fixtures().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await; 

        let req = get_fake_httprequest_with_bearer_token(
            DEV_USERNAME.to_string(), DeveloperOrEmployer::Developer, &app_data.auth_keys.encoding_key, "/v1/developer", 1, Some(60*2), None
        );

        let result = change_password(app_data, Json(ChangePasswordRoute { 
            id: 1,
            old_password: "test1234".to_string(),
            new_password: "test1234".to_string(),
            dev_or_emp: AuthDeveloperOrEmployer::Developer
        }), req).await;

        assert!(result.is_ok());
    }        

    #[tokio::test]
    async fn test_get_developer_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await; 

        let req = get_fake_httprequest_with_bearer_token(
            DEV_USERNAME.to_string(), DeveloperOrEmployer::Developer, &app_data.auth_keys.encoding_key, "/v1/developer", 1, Some(60*2), None
        );

        let result = get_developer(app_data, Path::from(1), req).await.unwrap();

        assert!(result.unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_developer_by_email_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await; 

        let req = get_fake_httprequest_with_bearer_token(
            DEV_USERNAME.to_string(), DeveloperOrEmployer::Developer, &app_data.auth_keys.encoding_key, "/v1/developer", 1, Some(60*2), None
        );

        let result = get_developer_by_email(app_data, Path::from("jon@jon.com".to_string()), req).await.unwrap();

        assert!(result.unwrap().id == 1);
    }

    #[tokio::test]
    async fn test_get_all_developers_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await; 

        let parameters = IdAndPagingModel { id: 1, page_size: 10, last_offset: 1 };
        let req = get_fake_httprequest_with_bearer_token(
            DEV_USERNAME.to_string(), 
            DeveloperOrEmployer::Developer, 
            &app_data.auth_keys.encoding_key, 
            "/v1/developers", 
            parameters.clone(), 
            Some(60*2),
            None
        );

        let result = get_all_developers(app_data, Json(parameters), req).await.unwrap();

        assert!(result.0.get(0).unwrap().id == 1);
    }
}