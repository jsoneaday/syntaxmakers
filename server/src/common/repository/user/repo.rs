use sqlx::{Postgres, query_as, Pool, Error};
use crate::common::repository::{user::models::AuthenticateResult, base::{DbRepo, ConnGetter}};
use async_trait::async_trait;
use crate::common::repository::user::models::DeveloperOrEmployer;
use super::models::ChangePassword;

mod internal {  
    use sqlx::query;

    use crate::common::{authentication::password_hash::{hash_password, verify_password}, repository::{developers::models::Developer, employers::models::Employer, error::SqlxError, user::models::{ChangePassword, Password}}};
    use super::*;    

    pub async fn authenticate_db(conn: &Pool<Postgres>, is_dev_or_emp: DeveloperOrEmployer, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error> {
        if is_dev_or_emp == DeveloperOrEmployer::Developer {            
            let result = query_as::<_, Developer>(r"
                select d.id, d.created_at, d.updated_at, d.user_name, d.full_name, d.email, d.password, d.primary_lang_id, sl.secondary_lang_id
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