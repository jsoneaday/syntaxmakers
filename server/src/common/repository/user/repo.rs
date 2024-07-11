use sqlx::{Postgres, query_as, Pool, Error};
use crate::common::{emailer::emailer::EmailerSendService, repository::{base::{ConnGetter, DbRepo}, user::models::AuthenticateResult}};
use async_trait::async_trait;
use crate::common::repository::user::models::DeveloperOrEmployer;
use super::models::ChangePassword;

mod internal {  
    use sqlx::query;

    use crate::common::{authentication::password_hash::{hash_password, verify_password}, emailer::emailer::EmailerSendService, repository::{developers::models::Developer, employers::models::Employer, error::SqlxError, user::models::{ChangePassword, Password}}};
    use super::*;    

    pub async fn authenticate_db(conn: &Pool<Postgres>, is_dev_or_emp: DeveloperOrEmployer, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error> {
        if is_dev_or_emp == DeveloperOrEmployer::Developer {            
            let result = query_as::<_, Developer>(r"
                select d.id, d.created_at, d.updated_at, d.user_name, d.full_name, d.email, d.description, d.password, d.primary_lang_id, sl.secondary_lang_id
                from developer d left join developers_secondary_langs sl on d.id = sl.developer_id where email = $1
            ")
            .bind(email)
            .fetch_optional(conn)
            .await;

            match result {
                Ok(opt_entity) => match opt_entity {
                    Some(entity) => {
                        match entity.verify_password(&password) {
                            Ok(is_same) => {
                                if is_same {
                                    Ok(AuthenticateResult::Success { id: entity.id })
                                } else {
                                    Ok(AuthenticateResult::Failure)
                                }                                
                            },
                            Err(_) => {
                                Ok(AuthenticateResult::Failure)
                            }
                        }                        
                    },
                    None => Ok(AuthenticateResult::Failure)
                },
                Err(e) => {
                    Err(e.into())
                }
            }
        } else {            
            let result = query_as::<_, Employer>("select * from employer where email = $1")
            .bind(email)
            .fetch_optional(conn)
            .await;

            match result {
                Ok(opt_entity) => match opt_entity {
                    Some(entity) => {
                        match entity.verify_password(&password) {
                            Ok(is_same) => {
                                if is_same {
                                    Ok(AuthenticateResult::Success { id: entity.id })
                                } else {
                                    Ok(AuthenticateResult::Failure)
                                }                                
                            },
                            Err(_) => Ok(AuthenticateResult::Failure)
                        }                        
                    },
                    None => Ok(AuthenticateResult::Failure)
                },
                Err(e) => Err(e.into())
            }
        }           
    }

    pub async fn change_password(conn: &Pool<Postgres>, change_password: ChangePassword) -> Result<(), sqlx::Error> {
        let query_str = if change_password.dev_or_emp == DeveloperOrEmployer::Developer {
            "select password from developer where id = $1"
        } else {
            "select password from employer where id = $1"
        };
        let password_result = query_as::<_, Password>(query_str)
        .bind(change_password.id)
        .fetch_one(conn)
        .await;

        match password_result {
            Ok(current_password) => {
                if !verify_password(&change_password.old_password, &current_password.password).unwrap() {
                    return Err(SqlxError::PasswordChangeFailed.into());
                }
                if change_password.new_password.len() < 8 || change_password.new_password.len() > 50 { 
                    return Err(SqlxError::PasswordChangeFailed.into());
                }
            },
            Err(e) => return Err(e)
        };

        let query_str = if change_password.dev_or_emp == DeveloperOrEmployer::Developer {
            "update developer set password = $2 where id = $1"
        } else {
            "update employer set password = $2 where id = $1"
        };
        match query::<_>(query_str)
            .bind(change_password.id)
            .bind(hash_password(&change_password.new_password).unwrap())
            .execute(conn)
            .await {
                Ok(row) => {
                    println!("change password result {:?}", row);
                    if row.rows_affected() == 0 {
                        return Err(SqlxError::PasswordChangeFailed.into());
                    }
                    Ok(())
                },
                Err(e) => Err(e)
        }
    }

    pub async fn send_email<E: EmailerSendService>(conn: &Pool<Postgres>, sender_emp_id: i64, receiver_dev_id: i64, subject: String, body: String, emailer: &E) -> Result<(), sqlx::Error> {
        match query_as::<_, Employer>("select * from employer where id = $1")
            .bind(sender_emp_id)
            .fetch_one(conn)
            .await {
                Ok(emp) => {
                    match query_as::<_, Developer>(r"
                        select d.id, d.created_at, d.updated_at, d.user_name, d.full_name, d.email, d.description, d.password, d.primary_lang_id, dsl.secondary_lang_id
                        from developer d join developers_secondary_langs dsl on d.id = dsl.developer_id
                        where d.id = $1
                    ")
                        .bind(receiver_dev_id)
                        .fetch_one(conn)
                        .await {
                            Ok(dev) => {
                                let _subject = format!("SyntaxMakers: {}", subject);
                                let _subject = &_subject[0.. if _subject.len() > 50 {
                                    50
                                } else {
                                    _subject.len()
                                }];
                                match emailer.send_email(emp.full_name, emp.email, dev.full_name, dev.email, _subject.to_string(), body).await {
                                    Ok(()) => {
                                        println!("success");
                                        Ok(())
                                    },
                                    Err(e) => {
                                        println!("route: {}", e);
                                        Err(e.into())          
                                    }
                                }
                            },
                            Err(e) => {
                                println!("route: {}", e);
                                Err(e)
                            }
                        }                    
                },
                Err(e) => {
                    println!("route: {}", e);
                    Err(e)
                }
            }        
    }
}

#[async_trait]
pub trait AuthenticateDbFn {
    async fn authenticate_db(&self, is_dev_or_emp: DeveloperOrEmployer, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error>;
}

#[async_trait]
impl AuthenticateDbFn for DbRepo {
    async fn authenticate_db(&self, is_dev_or_emp: DeveloperOrEmployer, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error> {
        internal::authenticate_db(self.get_conn(), is_dev_or_emp, email, password).await
    }
}

#[async_trait]
pub trait ChangePasswordFn {
    async fn change_password(&self, change_password: ChangePassword) -> Result<(), Error>;
}

#[async_trait]
impl ChangePasswordFn for DbRepo {
    async fn change_password(&self, change_password: ChangePassword) -> Result<(), Error> {
        internal::change_password(self.get_conn(), change_password).await
    }
}

#[async_trait]
pub trait SendEmailFn<E: EmailerSendService + Send + Sync> {
    async fn send_email(&self, send_emp_id: i64, receive_dev_id: i64, subject: String, body: String, emailer: &E) -> Result<(), Error>;
}

#[async_trait]
impl<E: EmailerSendService + Send + Sync>  SendEmailFn<E> for DbRepo {
    async fn send_email(&self, send_emp_id: i64, receive_dev_id: i64, subject: String, body: String, emailer: &E) -> Result<(), Error> {
        internal::send_email(self.get_conn(), send_emp_id, receive_dev_id, subject, body, emailer).await
    }
}