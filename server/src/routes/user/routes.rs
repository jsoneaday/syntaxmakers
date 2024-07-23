use actix_web::{web::{Data, Json, Query}, HttpRequest};
use crate::{
    app_state::AppState, 
    common::{
        authentication::auth_keys_service::Authenticator, 
        emailer::emailer::{EmailerReceiveService, EmailerSendService}, 
        repository::{
            base::Repository, 
            developers::repo::{InsertDevForgotPasswordConfirmFn, QueryDeveloperFn}, 
            employers::repo::{InsertEmpForgotPasswordConfirmFn, QueryEmployerFn}, 
            user::{models::{ChangePassword, RepoDeveloperOrEmployer, RepoResetPassword}, repo::{ChangePasswordFn, ResetPasswordFn, SendEmailFn}}
        }
    }, 
    routes::{auth_helper::check_is_authenticated, authentication::models::{ForgotPassword, RouteDeveloperOrEmployer, RouteResetPassword}, base_model::OutputBool, user_error::UserError}
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

/// User changes password after logged in
pub async fn change_password<T: QueryDeveloperFn + QueryEmployerFn + ChangePasswordFn + Repository, E: EmailerSendService, U: Authenticator>(
    app_data: Data<AppState<T, E, U>>, 
    json: Json<ChangePasswordRoute>,
    req: HttpRequest
) -> Result<OutputBool, UserError> {
    let is_auth = check_is_authenticated(app_data.clone(), json.id, if json.dev_or_emp == RouteDeveloperOrEmployer::Developer {
        RouteDeveloperOrEmployer::Developer
    } else {
        RouteDeveloperOrEmployer::Employer
    }, req).await;
    if !is_auth {
        error!("Authorization failed");
        return Err(UserError::AuthenticationFailed);
    }
    
    let result = app_data.repo.change_password(ChangePassword {
        id: json.id,
        old_password: json.old_password.to_owned(),
        new_password: json.new_password.to_owned(),
        dev_or_emp: if json.dev_or_emp == RouteDeveloperOrEmployer::Developer {
            RepoDeveloperOrEmployer::Developer
        } else {
            RepoDeveloperOrEmployer::Employer
        }
    }).await;

    match result {
        Ok(_) => Ok(OutputBool { result: true }),
        Err(e) => Err(e.into())
    }
}

pub async fn forgot_password<T: InsertEmpForgotPasswordConfirmFn<E> + InsertDevForgotPasswordConfirmFn<E> + Repository, E: EmailerSendService + Send + Sync, U: Authenticator>(
    app_data: Data<AppState<T, E, U>>, json: Json<ForgotPassword>
) -> Result<OutputBool, UserError> {
    println!("start forgot_password {:?}", json);
    if json.dev_or_emp == RouteDeveloperOrEmployer::Developer {
        match app_data.repo.insert_dev_forgot_password_confirm(json.email.to_owned(), &app_data.emailer).await {
            Ok(_) => Ok(OutputBool { result: true }),
            Err(e) => Err(e.into())
        }
    } else {
        println!("emp");
        match app_data.repo.insert_emp_forgot_password_confirm(json.email.to_owned(), &app_data.emailer).await {
            Ok(_) => Ok(OutputBool { result: true }),
            Err(e) => Err(e.into())
        }
    }
}

/// User resets password due to forgotten password
pub async fn reset_password<T: ResetPasswordFn + Repository, E: EmailerSendService + Send + Sync, U: Authenticator>(
    app_data: Data<AppState<T, E, U>>, json: Json<RouteResetPassword>
) -> Result<OutputBool, UserError> {
    println!("start reset_password {:?}", json);

    let result = app_data.repo.reset_password(RepoResetPassword {
        user_id: json.user_id,
        new_password: json.new_password.to_owned(),
        dev_or_emp: if json.dev_or_emp == RouteDeveloperOrEmployer::Developer {
            RepoDeveloperOrEmployer::Developer
        } else {
            RepoDeveloperOrEmployer::Employer
        },
        unique_key: json.unique_key
    }).await;

    match result {
        Ok(_) => Ok(OutputBool { result: true }),
        Err(e) => Err(e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{authentication::auth_keys_service::AuthenticationError, repository::user::{models::AuthenticateResult, repo::AuthenticateDbFn}}, 
        common_test::fixtures::{get_app_data, MockEmailer}
    };
    use jsonwebtoken::DecodingKey;
    use super::*;
    use async_trait::async_trait;

    struct MockDbRepo;
    struct MockAuthService;
    #[async_trait]
    impl Authenticator for MockAuthService {
        async fn is_authenticated(&self, _: String, _: Vec<(&str, &str)>, _: &DecodingKey) -> Result<bool, AuthenticationError> {
            Ok(true)
        }
    }

    #[async_trait]
    impl Repository for MockDbRepo {
        async fn init() -> Self {
            MockDbRepo
        }
    }

    #[async_trait]
    impl AuthenticateDbFn for MockDbRepo {
        async fn authenticate_db(&self, _: RepoDeveloperOrEmployer, _: String, _: String) -> Result<AuthenticateResult, sqlx::Error> {
            Ok(AuthenticateResult::Success{ id: 1 })
        }
    }

    #[async_trait]
    impl ResetPasswordFn for MockDbRepo {
        async fn reset_password(&self, _: RepoResetPassword) -> Result<(), sqlx::Error> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_reset_password_route() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let emailer = MockEmailer;
        let app_data = get_app_data(repo, emailer, auth_service).await; 

        let result = reset_password(app_data, Json(RouteResetPassword {
            user_id: 1,
            new_password: "test1234".to_string(),
            dev_or_emp: RouteDeveloperOrEmployer::Developer,
            unique_key: uuid::Uuid::now_v7()
        })).await;
        
        assert!(result.is_ok());        
    }
}