use actix_web::{web::{Data, Json, Query}, HttpRequest};
use crate::{
    app_state::AppState, 
    common::{
        authentication::auth_keys_service::Authenticator, 
        emailer::emailer::{EmailerReceiveService, EmailerSendService}, 
        repository::{
            base::Repository, 
            developers::repo::QueryDeveloperFn, 
            employers::repo::QueryEmployerFn, 
            user::{models::{ChangePassword, DeveloperOrEmployer}, repo::{ChangePasswordFn, SendEmailFn}}
        }
    }, 
    routes::{auth_helper::check_is_authenticated, authentication::models::DeveloperOrEmployer as AuthDeveloperOrEmployer, base_model::OutputBool, user_error::UserError}
};
use crate::common::repository::developers::repo::ConfirmDevEmailFn as ConfirmDevEmailFn;
use crate::common::repository::employers::repo::ConfirmEmpEmailFn as ConfirmEmpEmailFn;
use super::models::{ChangePasswordRoute, ConfirmEmailQuery, SendEmail};
use log::error;

pub async fn confirm_email<T: ConfirmDevEmailFn + ConfirmEmpEmailFn + Repository + Send + Sync, E: EmailerSendService + EmailerReceiveService<T>, U: Authenticator>(
    app_data: Data<AppState<T, E, U>>,
    query: Query<ConfirmEmailQuery>
) -> Result<OutputBool, UserError> {
    match app_data.emailer.receive_email_confirm(&app_data.repo, query.is_dev, query.profile_id, query.new_email.to_owned(), query.unique_key.unwrap()).await {
        Ok(()) => {
            println!("success");
            Ok(OutputBool { result: true })
        },
        Err(e) => {
            println!("route: {}", e);
            match e {
                _ => Err(e.into())
            }            
        }
    }
}

pub async fn send_email<T: SendEmailFn<E> + Repository + Send + Sync, E: EmailerSendService + Send + Sync, U: Authenticator>(
    app_data: Data<AppState<T, E, U>>,
    json: Json<SendEmail>
) -> Result<OutputBool, UserError> {
    match app_data.repo.send_email(json.sender_emp_id, json.receiver_dev_id, json.subject.to_owned(), json.body.to_owned(), &app_data.emailer).await {
        Ok(()) => Ok(OutputBool { result: true }),
        Err(e) => Err(e.into())
    }
}

pub async fn change_password<T: QueryDeveloperFn + QueryEmployerFn + ChangePasswordFn + Repository, E: EmailerSendService, U: Authenticator>(
    app_data: Data<AppState<T, E, U>>, 
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