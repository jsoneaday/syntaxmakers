use async_trait::async_trait;
use mockall::automock;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::DbRepo;
use crate::common::repository::{base::EntityId, companies::models::{NewCompany, Company}};

mod internal {
    use super::*;    

    pub async fn create_company(conn: &Pool<Postgres>, new_company: NewCompany) -> Result<EntityId, Error> {
        query_as::<_, EntityId>("insert into company (name) values ($1) returning id")
            .bind(new_company.name)
            .fetch_one(conn).await
    }

    pub async fn get_all_companies(conn: &Pool<Postgres>) -> Result<Vec<Company>, Error> {
        query_as::<_, Company>(
            r"
            select * from company 
            order by updated_at desc 
            "
        )
        .fetch_all(conn).await
    }
}

#[automock]
#[async_trait]
pub trait CreateCompanyFn {
    async fn create_company(&self, conn: &Pool<Postgres>, new_company: NewCompany) -> Result<EntityId, Error>;
}

#[async_trait]
impl CreateCompanyFn for DbRepo {
    async fn create_company(&self, conn: &Pool<Postgres>, new_company: NewCompany) -> Result<EntityId, Error> {
        internal::create_company(conn, new_company).await
    }
}

#[automock]
#[async_trait]
pub trait GetAllCompaniesFn {
    async fn get_all_companies(&self, conn: &Pool<Postgres>) -> Result<Vec<Company>, Error>;
}

#[async_trait]
impl GetAllCompaniesFn for DbRepo {
    async fn get_all_companies(&self, conn: &Pool<Postgres>) -> Result<Vec<Company>, Error> {
        internal::get_all_companies(conn).await
    }
}
