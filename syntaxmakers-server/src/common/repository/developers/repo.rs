use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::DbRepo;
use crate::common::repository::developers::models::Developer;
use crate::common::repository::base::EntityId;
use crate::common::repository::developers::models::NewDeveloper;

mod internal {
    use super::*;    

    pub async fn create_developer(conn: &Pool<Postgres>, new_developer: NewDeveloper) -> Result<EntityId, Error> {
        let result = query_as::<_, EntityId>("insert into developer (user_name, full_name, primary_lang_id) values ($1, $2, $3) returning id")
            .bind(new_developer.user_name)
            .bind(new_developer.full_name)
            .bind(new_developer.primary_lang_id)
            .fetch_one(conn)
            .await;

        match result {
            Ok(row) => Ok(row),
            Err(e) => {
                println!("create developer error: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn get_developer(conn: &Pool<Postgres>, id: i64) -> Result<Option<Developer>, Error> {
        query_as::<_, Developer>("select * from developer where id = $1")
            .bind(id)
            .fetch_optional(conn).await
    }

    pub async fn get_all_developers(conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Developer>, Error> {
        query_as::<_, Developer>(
            r"
            select * from developer
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
pub trait CreateDeveloperFn {
    async fn create_developer(&self, conn: &Pool<Postgres>, new_developer: NewDeveloper) -> Result<EntityId, Error>;
}

#[async_trait]
impl CreateDeveloperFn for DbRepo {
    async fn create_developer(&self, conn: &Pool<Postgres>, new_developer: NewDeveloper) -> Result<EntityId, Error> {
        internal::create_developer(conn, new_developer).await
    }
}

#[async_trait]
pub trait GetDeveloperFn {
    async fn get_developer(&self, conn: &Pool<Postgres>, id: i64) -> Result<Option<Developer>, Error>;
}

#[async_trait]
impl GetDeveloperFn for DbRepo {
    async fn get_developer(&self, conn: &Pool<Postgres>, id: i64) -> Result<Option<Developer>, Error> {
        internal::get_developer(conn, id).await
    }
}

#[async_trait]
pub trait GetAllDevelopersFn {
    async fn get_all_developers(&self, conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Developer>, Error>;
}

#[async_trait]
impl GetAllDevelopersFn for DbRepo {
    async fn get_all_developers(&self, conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Developer>, Error> {
        internal::get_all_developers(conn, page_size, last_offset).await
    }
}