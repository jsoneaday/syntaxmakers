use sqlx::{Postgres, query_as, Pool};
use crate::common::repository::{user::models::AuthenticateResult, base::{DbRepo, ConnGetter}};
use async_trait::async_trait;
use crate::common::repository::user::models::DeveloperOrEmployer;

mod internal {  
    use crate::common::repository::{developers::models::Developer, employers::models::Employer};
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