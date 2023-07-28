use actix_web::web::{Data, Json, Path};
use crate::{
    app_state::AppState, 
    common::repository::{developers::{repo::{InsertDeveloperFn, QueryDeveloperFn, QueryAllDevelopersFn}, models::NewDeveloper}, base::Repository}, 
    routes::{base_model::OutputId, user_error::UserError}
};
use super::models::{NewDeveloperForRoute, DeveloperResponder, QueryAllDevelopers, DeveloperResponders};

pub async fn create_developer<T: InsertDeveloperFn + Repository>(
    app_data: Data<AppState<T>>, 
    json: Json<NewDeveloperForRoute>
) -> Result<OutputId, UserError> {
    let result = app_data.repo.insert_developer(NewDeveloper {
        user_name: json.user_name.to_owned(),
        full_name: json.full_name.to_owned(),
        email: json.email.to_owned(),
        primary_lang_id: json.primary_lang_id
    }).await;

    match result {
        Ok(entity) => Ok(OutputId { id: entity.id }),
        Err(e) => Err(e.into())
    }
}

pub async fn get_developer<T: QueryDeveloperFn + Repository>(
    app_data: Data<AppState<T>>, 
    path: Path<i64>
) -> Result<Option<DeveloperResponder>, UserError> {
    let id = path.into_inner();
    let result = app_data.repo.query_developer(id).await;

    match result {
        Ok(optional_dev) => match optional_dev {
            Some(dev) => Ok(Some(DeveloperResponder { 
                id: dev.id, 
                updated_at: dev.updated_at, 
                user_name: dev.user_name.to_owned(), 
                full_name: dev.full_name.to_owned(), 
                email: dev.email.to_owned(), 
                primary_lang_id: dev.primary_lang_id
            })),
            None => Ok(None)
        },
        Err(e) => Err(e.into())
    }
}

pub async fn get_all_developers<T: QueryAllDevelopersFn + Repository>(
    app_data: Data<AppState<T>>, 
    json: Json<QueryAllDevelopers>
) -> Result<DeveloperResponders, UserError> {
    let result = app_data.repo.query_all_developers(json.page_size, json.last_offset).await;

    match result {
        Ok(developers) => {
            let devs = developers.iter().map(|dev| {
                DeveloperResponder { 
                    id: dev.id, 
                    updated_at: dev.updated_at, 
                    user_name: dev.user_name.to_owned(), 
                    full_name: dev.full_name.to_owned(), 
                    email: dev.email.to_owned(), 
                    primary_lang_id: dev.primary_lang_id
                }
            })
            .collect::<Vec<DeveloperResponder>>();
            Ok(DeveloperResponders(devs))
        },
        Err(e) => Err(e.into())
    }
}