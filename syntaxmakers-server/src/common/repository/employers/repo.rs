use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::DbRepo;
use crate::common::repository::employers::models::{NewEmployer, Employer};
use crate::common::repository::base::EntityId;

mod internal {
    use super::*;    

    pub async fn create_employer(conn: &Pool<Postgres>, new_employer: NewEmployer) -> Result<EntityId, Error> {
        let result = query_as::<_, EntityId>("insert into employer (user_name, full_name, email, company_id) values ($1, $2, $3, $4) returning id")
            .bind(new_employer.user_name)
            .bind(new_employer.full_name)
            .bind(new_employer.email)
            .bind(new_employer.company_id)
            .fetch_one(conn)
            .await;

        match result {
            Ok(row) => Ok(row),
            Err(e) => {
                println!("create employer error: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn get_employer(conn: &Pool<Postgres>, id: i64) -> Result<Option<Employer>, Error> {
        query_as::<_, Employer>("select * from employer where id = $1")
            .bind(id)
            .fetch_optional(conn).await
    }

    pub async fn get_all_employers(conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Employer>, Error> {
        query_as::<_, Employer>(
            r"
            select * from employer
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
pub trait CreateEmployerFn {
    async fn create_employer(&self, conn: &Pool<Postgres>, new_developer: NewEmployer) -> Result<EntityId, Error>;
}

#[async_trait]
impl CreateEmployerFn for DbRepo {
    async fn create_employer(&self, conn: &Pool<Postgres>, new_developer: NewEmployer) -> Result<EntityId, Error> {
        internal::create_employer(conn, new_developer).await
    }
}

#[async_trait]
pub trait GetEmployerFn {
    async fn get_employer(&self, conn: &Pool<Postgres>, id: i64) -> Result<Option<Employer>, Error>;
}

#[async_trait]
impl GetEmployerFn for DbRepo {
    async fn get_employer(&self, conn: &Pool<Postgres>, id: i64) -> Result<Option<Employer>, Error> {
        internal::get_employer(conn, id).await
    }
}

#[async_trait]
pub trait GetAllEmployersFn {
    async fn get_all_employers(&self, conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Employer>, Error>;
}

#[async_trait]
impl GetAllEmployersFn for DbRepo {
    async fn get_all_employers(&self, conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Employer>, Error> {
        internal::get_all_employers(conn, page_size, last_offset).await
    }
}