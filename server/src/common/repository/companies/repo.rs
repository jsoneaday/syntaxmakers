use async_trait::async_trait;
use mockall::automock;
use sqlx::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::{base::EntityId, companies::models::{NewCompany, Company}};

mod internal {
    use super::*;    

    pub async fn insert_company(conn: &Pool<Postgres>, new_company: NewCompany) -> Result<EntityId, Error> {
        query_as::<_, EntityId>(
            r"
            insert into company 
            (name, logo, headquarters_country_id) 
            values 
            ($1, $2, $3) 
            returning id
            ")
            .bind(new_company.name)
            .bind(new_company.logo)
            .bind(new_company.headquarters_country_id)
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
pub trait InsertCompanyFn {
    async fn insert_company(&self, new_company: NewCompany) -> Result<EntityId, Error>;
}

#[async_trait]
impl InsertCompanyFn for DbRepo {
    async fn insert_company(&self, new_company: NewCompany) -> Result<EntityId, Error> {
        internal::insert_company(self.get_conn(), new_company).await
    }
}

#[automock]
#[async_trait]
pub trait QueryAllCompaniesFn {
    async fn query_all_companies(&self) -> Result<Vec<Company>, Error>;
}

#[async_trait]
impl QueryAllCompaniesFn for DbRepo {
    async fn query_all_companies(&self) -> Result<Vec<Company>, Error> {
        internal::get_all_companies(self.get_conn()).await
    }
}
