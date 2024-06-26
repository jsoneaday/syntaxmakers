use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::developers::models::{Developer, NewDeveloper, UpdateDeveloper, DevPassword};
use crate::common::repository::base::EntityId;
use log::error;

mod internal {
    use sqlx::query;
    use crate::common::repository::error::SqlxError;
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

    pub async fn update_developer(conn: &Pool<Postgres>, update_developer: UpdateDeveloper) -> Result<(), Error> {
        let mut tx = conn.begin().await.unwrap();

        let password_result = query_as::<_, DevPassword>(r"
            select password from developer where id = $1
        ")
        .bind(update_developer.id)
        .fetch_one(&mut *tx)
        .await;
        match password_result {
            Ok(current_password) => {
                if current_password.password != update_developer.old_password {
                    return Err(SqlxError::PasswordChangeFailed.into());
                }
                if update_developer.new_password.len() < 8 {
                    return Err(SqlxError::PasswordChangeFailed.into());
                }
            },
            Err(e) => return Err(e)
        }
        
        let update_result = query::<_>(
            r"
                update developer
                set full_name = $2, email = $3, password = $4, primary_lang_id = $5
                where id = $1
            ")
            .bind(update_developer.id)
            .bind(update_developer.full_name)
            .bind(update_developer.email)
            .bind(update_developer.new_password)
            .bind(update_developer.primary_lang_id)
            .execute(&mut *tx)
            .await;

        let update_result = match update_result {
            Ok(row) => {
                println!("update developer {:?}", row);
                Ok(row)
            },
            Err(e) => {
                error!("update developer error: {:?}", e);
                Err(e)
            }
        };
        if let Err(e) = update_result {
            return Err(e);
        }

        match query_as::<_, EntityId>(
                r"select id from developers_secondary_langs where developer_id = $1 order by updated_at desc limit 1"
            )
            .bind(update_developer.id)
            .fetch_optional(&mut *tx)
            .await {
            Ok(entity) => {
                match entity {
                    Some(_) => {
                        if let None = update_developer.secondary_lang_id {
                            let test = query::<_>(
                                r"
                                    delete from developers_secondary_langs
                                    where developer_id = $1
                                "
                            )
                            .bind(update_developer.id)
                            .execute(&mut *tx)
                            .await;
                            println!("delete secondary lang {:?}", test);
                        } else {                            
                            let test = query::<_>(
                                r"
                                    update developers_secondary_langs
                                    set secondary_lang_id = $2
                                    where developer_id = $1
                                "
                            )
                            .bind(update_developer.id)
                            .bind(update_developer.secondary_lang_id.unwrap())
                            .execute(&mut *tx)
                            .await;
                            println!("update existing secondary {:?}", test);
                        }
                    },
                    None => {
                        if let Some(secondary_lang_id) = update_developer.secondary_lang_id {
                            _ = query_as::<_, EntityId>(
                                r"
                                    insert into developers_secondary_langs
                                    (developer_id, secondary_lang_id)
                                    values
                                    ($1, $2)
                                    returning id
                                "
                            )
                            .bind(update_developer.id)
                            .bind(secondary_lang_id)
                            .fetch_one(&mut *tx)
                            .await;
                            println!("insert new secondary lang")
                        }
                    }
                }
            },
            Err(e) => return Err(e)
        }
        
        _ = tx.commit().await;

        Ok(())
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
pub trait UpdateDeveloperFn {
    async fn update_developer(&self, update_developer: UpdateDeveloper) -> Result<(), Error>;
}

#[async_trait]
impl UpdateDeveloperFn for DbRepo {
    async fn update_developer(&self, update_developer: UpdateDeveloper) -> Result<(), Error> {
        internal::update_developer(self.get_conn(), update_developer).await
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