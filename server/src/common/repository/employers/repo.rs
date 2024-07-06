use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as, query};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::employers::models::{NewEmployer, Employer};
use crate::common::repository::base::EntityId;
use crate::common::{authentication::password_hash::hash_password, repository::{employers::models::UpdateEmployer, error::SqlxError}};
use log::error;

mod internal {
    use chrono::Utc;
    use sqlx::PgConnection;
    use crate::common::repository::employers::models::EmpEmailConfirm;
    use super::*;    

    pub async fn insert_employer(conn: &Pool<Postgres>, new_employer: NewEmployer) -> Result<EntityId, Error> {
        let mut tx = conn.begin().await.unwrap();

        let employer = query_as::<_, EntityId>("insert into employer (user_name, full_name, email, password, company_id) values ($1, $2, $3, $4, $5) returning id")
            .bind(new_employer.user_name)
            .bind(new_employer.full_name)
            .bind(new_employer.email.clone())
            .bind(hash_password(&new_employer.password).unwrap())
            .bind(new_employer.company_id)
            .fetch_one(&mut *tx)
            .await;

        match employer {
            Ok(entity) => {
                match insert_email_confirm(&mut *tx, entity.id, new_employer.email).await {
                    Ok(_) => (),
                    Err(e) => return Err(e)
                };

                _ = tx.commit().await;
                Ok(entity)
            },
            Err(e) => {
                error!("create employer error: {:?}", e);
                Err(e)
            }
        }
    }

    /// note: Does NOT change password!
    pub async fn update_employer(conn: &Pool<Postgres>, update_employer: UpdateEmployer) -> Result<(), Error> {
        // need later to confirm does request change email?
        let existing_employer = query_employer(conn, update_employer.id).await.unwrap();

        let mut tx = conn.begin().await.unwrap();
        let update_result = query::<_>(
            r"
                update employer
                set updated_at = $2, full_name = $3, email = $4, company_id = $5
                where id = $1
            ")
            .bind(update_employer.id)
            .bind(Utc::now())
            .bind(update_employer.full_name)
            .bind(update_employer.email.clone())
            .bind(update_employer.company_id)
            .execute(&mut *tx)
            .await;

        let update_result = match update_result {
            Ok(row) => {
                if row.rows_affected() > 0 {
                    Ok(row)
                } else {
                    error!("update employer has failed");
                    Err(SqlxError::QueryFailed)
                }
            },
            Err(e) => {
                error!("update employer error: {:?}", e);
                Err(SqlxError::QueryFailed)
            }
        };
        if let Err(e) = update_result {
            return Err(e.into());
        };

        if existing_employer.unwrap().email != update_employer.email {
            match insert_email_confirm(&mut *tx, update_employer.id, update_employer.email).await {
                Ok(_) => (),
                Err(e) => return Err(e)
            }
        }

        _ = tx.commit().await;
        Ok(())
    }

    pub async fn query_employer(conn: &Pool<Postgres>, id: i64) -> Result<Option<Employer>, Error> {
        query_as::<_, Employer>(
            r"
            select id, created_at, updated_at, user_name, full_name, email, password, company_id 
            from employer where id = $1
            ")
            .bind(id)
            .fetch_optional(conn).await
    }

    pub async fn query_employer_by_email(conn: &Pool<Postgres>, email: String) -> Result<Option<Employer>, Error> {
        query_as::<_, Employer>(
            r"
            select id, created_at, updated_at, user_name, full_name, email, password, company_id 
            from employer where email = $1
            ")
            .bind(email)
            .fetch_optional(conn).await
    }

    pub async fn query_employer_by_user_name(conn: &Pool<Postgres>, user_name: String) -> Result<Option<Employer>, Error> {
        query_as::<_, Employer>(
            r"
            select id, created_at, updated_at, user_name, full_name, email, password, company_id 
            from employer where user_name = $1
            ")
            .bind(user_name)
            .fetch_optional(conn).await
    }

    pub async fn query_all_employers(conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Employer>, Error> {
        query_as::<_, Employer>(
            r"
            select id, created_at, updated_at, user_name, full_name, email, password, company_id 
            from employer
            order by updated_at desc
            limit $1
            offset $2
            "
        )
        .bind(page_size)
        .bind(last_offset)
        .fetch_all(conn).await
    }

    pub async fn has_unconfirmed_email(conn: &Pool<Postgres>, email: String) -> Result<bool, Error> {
        match query_as::<_, EntityId>(r"
            select id from emp_email_confirmation where is_valid = true and is_confirmed = false and new_email = $1
        ")
        .bind(email)
        .fetch_all(conn)
        .await {
            Ok(rows) => if rows.len() > 0 {
                Ok(true)
            } else {
                Ok(false)
            },
            Err(e) => Err(e)
        }
    }

    // whether the user is attempting to use their old email or their new email
    pub async fn insert_email_confirm(tx: &mut PgConnection, emp_id: i64, new_email: String) -> Result<EntityId, Error> {
        query_as::<_, EntityId>(r"
            insert into emp_email_confirmation
            (employer_id, is_confirmed, is_valid, new_email)
            values
            ($1, false, true, $2)
            returning id
        ")
        .bind(emp_id)
        .bind(new_email)
        .fetch_one(tx)
        .await
    }

    pub async fn confirm_email(conn: &Pool<Postgres>, email: String, emp_id: i64) -> Result<(), Error> {
        let mut tx = conn.begin().await.unwrap();

        #[allow(unused)]
        let mut current_email_confirm: Option<EmpEmailConfirm> = None;
        match query_as::<_, EmpEmailConfirm>(
            "select * from emp_email_confirmation where is_confirmed = false and is_valid = true and employer_id = $1 and new_email = $2 order by updated_at desc limit 1"
        )
        .bind(emp_id)
        .bind(email)
        .fetch_one(&mut *tx)
        .await {
            Ok(confirm) => if confirm.is_valid {
                current_email_confirm = Some(confirm.clone());
                // see if later email confirm exists
                match query_as::<_, EmpEmailConfirm>(
                    "select * from emp_email_confirmation where employer_id = $1 and updated_at > $2"                
                )
                .bind(emp_id)
                .bind(confirm.updated_at)
                .fetch_all(&mut *tx)
                .await {
                    Ok(found_newer_confirms) => if found_newer_confirms.len() > 0 {
                        println!("found newer confirms");
                        return Err(SqlxError::NewerEmailConfirmExist.into());
                    },
                    Err(e) => {
                        println!("{}", e);
                        return Err(e);
                    }
                }                
            } else {
                println!("email confirm invalid");
                return Err(SqlxError::EmailConfirmInvalid.into());
            },
            Err(e) => {
                println!("{}", e);
                return Err(e);
            }
        };

        let current_email_confirm_id = current_email_confirm.clone().unwrap().id;
        // get all email change attempts prior to this one and invalidate them
        match query::<_>(
            r"
            update emp_email_confirmation 
            set is_valid = false, updated_at = $3
            where employer_id = $2 and id <> $1
            "
        )
        .bind(current_email_confirm_id)
        .bind(emp_id)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await {
            Ok(_) => (),
            Err(e) => return Err(e)
        };
        
        match query::<_>(r"
            update emp_email_confirmation
            set is_confirmed = true, is_valid = false, updated_at = $2
            where id = $1
        ")
        .bind(current_email_confirm_id)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await {
            Ok(row) => {
                if row.rows_affected() == 0 {                                        
                    return Err(SqlxError::EmailConfirmFailed.into())
                }
            },
            Err(e) => return Err(e)
        };

        match query::<_>("update employer set email = $2, updated_at = $3 where id = $1")
            .bind(emp_id)
            .bind(current_email_confirm.unwrap().new_email)
            .bind(Utc::now())
            .execute(&mut *tx)
            .await {
                Ok(row) => if row.rows_affected() > 0 {
                    _ = tx.commit().await;
                    Ok(())
                } else {
                    Err(SqlxError::UpdateProfileEmailFailed.into())
                },
                Err(e) => Err(e)
            }
    }
}

#[async_trait]
pub trait InsertEmployerFn {
    async fn insert_employer(&self, new_employer: NewEmployer) -> Result<EntityId, Error>;
}

#[async_trait]
impl InsertEmployerFn for DbRepo {
    async fn insert_employer(&self, new_employer: NewEmployer) -> Result<EntityId, Error> {
        internal::insert_employer(self.get_conn(), new_employer).await
    }
}

#[async_trait]
pub trait UpdateEmployerFn {
    async fn update_employer(&self, update_employer: UpdateEmployer) -> Result<(), Error>;
}

#[async_trait]
impl UpdateEmployerFn for DbRepo {
    async fn update_employer(&self, update_employer: UpdateEmployer) -> Result<(), Error> {
        internal::update_employer(self.get_conn(), update_employer).await
    }
}

#[async_trait]
pub trait QueryEmployerFn {
    async fn query_employer(&self, id: i64) -> Result<Option<Employer>, Error>;
}

#[async_trait]
impl QueryEmployerFn for DbRepo {
    async fn query_employer(&self, id: i64) -> Result<Option<Employer>, Error> {
        internal::query_employer(self.get_conn(), id).await
    }
}

#[async_trait]
pub trait QueryEmployerByEmailFn {
    async fn query_employer_by_email(&self, email: String) -> Result<Option<Employer>, Error>;
}

#[async_trait]
impl QueryEmployerByEmailFn for DbRepo {
    async fn query_employer_by_email(&self, email: String) -> Result<Option<Employer>, Error> {
        internal::query_employer_by_email(self.get_conn(), email).await
    }
}

#[async_trait]
pub trait QueryEmployerByUsernameFn {
    async fn query_employer_by_user_name(&self, user_name: String) -> Result<Option<Employer>, Error>;
}

#[async_trait]
impl QueryEmployerByUsernameFn for DbRepo {
    async fn query_employer_by_user_name(&self, user_name: String) -> Result<Option<Employer>, Error> {
        internal::query_employer_by_user_name(self.get_conn(), user_name).await
    }
}

#[async_trait]
pub trait QueryAllEmployersFn {
    async fn query_all_employers(&self, page_size: i32, last_offset: i64) -> Result<Vec<Employer>, Error>;
}

#[async_trait]
impl QueryAllEmployersFn for DbRepo {
    async fn query_all_employers(&self, page_size: i32, last_offset: i64) -> Result<Vec<Employer>, Error> {
        internal::query_all_employers(self.get_conn(), page_size, last_offset).await
    }
}

#[async_trait]
pub trait HasUnconfirmedEmailFn {
    async fn has_unconfirmed_email(&self, email: String) -> Result<bool, Error>;
}

#[async_trait]
impl HasUnconfirmedEmailFn for DbRepo {
    async fn has_unconfirmed_email(&self, email: String) -> Result<bool, Error> {
        internal::has_unconfirmed_email(self.get_conn(), email).await
    }
}

#[async_trait]
pub trait ConfirmEmailFn {
    async fn confirm_email(&self, email: String, emp_id: i64) -> Result<(), Error>;
}

#[async_trait]
impl ConfirmEmailFn for DbRepo {
    async fn confirm_email(&self, email: String, emp_id: i64) -> Result<(), Error> {
        internal::confirm_email(self.get_conn(), email, emp_id).await
    }
}