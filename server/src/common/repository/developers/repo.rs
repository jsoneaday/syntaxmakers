use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::developers::models::Developer;
use crate::common::repository::base::EntityId;
use crate::common::repository::developers::models::NewDeveloper;
use log::error;

mod internal {
    use super::*;    

    pub async fn insert_developer(conn: &Pool<Postgres>, new_developer: NewDeveloper) -> Result<EntityId, Error> {
        let mut tx = conn.begin().await.unwrap();
        
        let insert_result = query_as::<_, EntityId>(
            r"
            insert into developer 
            (user_name, full_name, email, password, primary_lang_id) 
            values 
            ($1, $2, $3, $4, $5)
            returning id
            ")
            .bind(new_developer.user_name)
            .bind(new_developer.full_name)
            .bind(new_developer.email)
            .bind(new_developer.password)
            .bind(new_developer.primary_lang_id)
            .fetch_one(&mut *tx)
            .await;

        let inserted_entity = match insert_result {
            Ok(row) => Ok(row),
            Err(e) => {
                error!("create developer error: {:?}", e);
                Err(e)
            }
        };
        if let Err(e) = inserted_entity {
            return Err(e);
        }

        let dev_id = inserted_entity.unwrap().id;
        if let Some(secondary_lang_id) = new_developer.secondary_lang_id {
            _ = query_as::<_, EntityId>(
                r"
                insert into developers_secondary_langs
                (developer_id, secondary_lang_id)
                values
                ($1, $2)
                returning id
                "
            )
            .bind(dev_id)
            .bind(secondary_lang_id)
            .fetch_one(&mut *tx)
            .await;
        }

        _ = tx.commit().await;

        Ok(EntityId { id: dev_id })
    }

    pub async fn query_developer(conn: &Pool<Postgres>, id: i64) -> Result<Option<Developer>, Error> {
        query_as::<_, Developer>(
            r"
            select d.id, d.created_at, d.updated_at, d.user_name, d.full_name, d.email, d.primary_lang_id, dsl.secondary_lang_id
            from developer d left join developers_secondary_langs dsl on d.id = dsl.developer_id
            where d.id = $1
            ")
            .bind(id)
            .fetch_optional(conn).await
    }

    pub async fn query_developer_by_email(conn: &Pool<Postgres>, email: String) -> Result<Option<Developer>, Error> {
        query_as::<_, Developer>(
            r"
            select d.id, d.created_at, d.updated_at, d.user_name, d.full_name, d.email, d.primary_lang_id, dsl.secondary_lang_id
            from developer d left join developers_secondary_langs dsl on d.id = dsl.developer_id
            where d.email = $1
            ")
            .bind(email)
            .fetch_optional(conn).await
    }

    pub async fn query_all_developers(conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Developer>, Error> {
        query_as::<_, Developer>(
            r"
            select d.id, d.created_at, d.updated_at, d.user_name, d.full_name, d.email, d.primary_lang_id, dsl.secondary_lang_id
            from developer d left join developers_secondary_langs dsl on d.id = dsl.developer_id
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
pub trait InsertDeveloperFn {
    async fn insert_developer(&self, new_developer: NewDeveloper) -> Result<EntityId, Error>;
}

#[async_trait]
impl InsertDeveloperFn for DbRepo {
    async fn insert_developer(&self, new_developer: NewDeveloper) -> Result<EntityId, Error> {
        internal::insert_developer(self.get_conn(), new_developer).await
    }
}

#[async_trait]
pub trait QueryDeveloperFn {
    async fn query_developer(&self, id: i64) -> Result<Option<Developer>, Error>;
}

#[async_trait]
impl QueryDeveloperFn for DbRepo {
    async fn query_developer(&self, id: i64) -> Result<Option<Developer>, Error> {
        internal::query_developer(self.get_conn(), id).await
    }
}

#[async_trait]
pub trait QueryDeveloperByEmailFn {
    async fn query_developer_by_email(&self, email: String) -> Result<Option<Developer>, Error>;
}

#[async_trait]
impl QueryDeveloperByEmailFn for DbRepo {
    async fn query_developer_by_email(&self, email: String) -> Result<Option<Developer>, Error> {
        internal::query_developer_by_email(self.get_conn(), email).await
    }
}

#[async_trait]
pub trait QueryAllDevelopersFn {
    async fn query_all_developers(&self, page_size: i32, last_offset: i64) -> Result<Vec<Developer>, Error>;
}

#[async_trait]
impl QueryAllDevelopersFn for DbRepo {
    async fn query_all_developers(&self, page_size: i32, last_offset: i64) -> Result<Vec<Developer>, Error> {
        internal::query_all_developers(self.get_conn(), page_size, last_offset).await
    }
}