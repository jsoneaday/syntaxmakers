use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as, query};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::employers::models::{NewEmployer, Employer};
use crate::common::repository::base::EntityId;
use crate::common::{authentication::password_hash::hash_password, repository::{employers::models::UpdateEmployer, error::SqlxError}};
use log::error;

mod internal {
    use super::*;    

    pub async fn insert_employer(conn: &Pool<Postgres>, new_employer: NewEmployer) -> Result<EntityId, Error> {
        let result = query_as::<_, EntityId>("insert into employer (user_name, full_name, email, password, company_id) values ($1, $2, $3, $4, $5) returning id")
            .bind(new_employer.user_name)
            .bind(new_employer.full_name)
            .bind(new_employer.email)
            .bind(hash_password(&new_employer.password).unwrap())
            .bind(new_employer.company_id)
            .fetch_one(conn)
            .await;

        match result {
            Ok(entity) => Ok(entity),
            Err(e) => {
                error!("create employer error: {:?}", e);
                Err(e)
            }
        }
    }

    /// note: Does NOT change password!
    pub async fn update_employer(conn: &Pool<Postgres>, update_employer: UpdateEmployer) -> Result<(), Error> {
        let update_result = query::<_>(
            r"
                update employer
                set full_name = $2, email = $3, company_id = $4
                where id = $1
            ")
            .bind(update_employer.id)
            .bind(update_employer.full_name)
            .bind(update_employer.email)
            .bind(update_employer.company_id)
            .execute(conn)
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
pub trait QueryAllEmployersFn {
    async fn query_all_employers(&self, page_size: i32, last_offset: i64) -> Result<Vec<Employer>, Error>;
}

#[async_trait]
impl QueryAllEmployersFn for DbRepo {
    async fn query_all_employers(&self, page_size: i32, last_offset: i64) -> Result<Vec<Employer>, Error> {
        internal::query_all_employers(self.get_conn(), page_size, last_offset).await
    }
}