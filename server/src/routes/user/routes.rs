use actix_web::{web::{Data, Json}, HttpRequest};
use crate::{
    app_state::AppState, 
    common::{
        authentication::auth_service::Authenticator, 
        repository::{base::Repository, developers::repo::QueryDeveloperFn, employers::repo::QueryEmployerFn, user::{models::{ChangePassword, DeveloperOrEmployer}, repo::ChangePasswordFn}}
    }, 
    routes::{auth_helper::check_is_authenticated, authentication::models::DeveloperOrEmployer as AuthDeveloperOrEmployer, base_model::OutputBool, user_error::UserError}
};
use super::models::ChangePasswordRoute;
use log::error;

pub async fn change_password<T: QueryDeveloperFn + QueryEmployerFn + ChangePasswordFn + Repository, U: Authenticator>(
    app_data: Data<AppState<T, U>>, 
    json: Json<ChangePasswordRoute>,
    req: HttpRequest
) -> Result<OutputBool, UserError> {
    let is_auth = check_is_authenticated(app_data.clone(), json.id, if json.dev_or_emp == AuthDeveloperOrEmployer::Developer {
        AuthDeveloperOrEmployer::Developer
    } else {
        AuthDeveloperOrEmployer::Employer
    }, req).await;
    if !is_auth {
        error!("Authorization failed");
        return Err(UserError::AuthenticationFailed);
    }
    
    let result = app_data.repo.change_password(ChangePassword {
        id: json.id,
        old_password: json.old_password.to_owned(),
        new_password: json.new_password.to_owned(),
        dev_or_emp: if json.dev_or_emp == AuthDeveloperOrEmployer::Developer {
            DeveloperOrEmployer::Developer
        } else {
            DeveloperOrEmployer::Employer
        }
    }).await;

    match result {
        Ok(_) => Ok(OutputBool { result: true }),
        Err(e) => Err(e.into())
    }
}