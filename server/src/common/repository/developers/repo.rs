use async_trait::async_trait;
use sqlx::error::Error;
use sqlx::{Postgres, Pool, query_as};
use crate::common::repository::base::{DbRepo, ConnGetter};
use crate::common::repository::developers::models::{Developer, NewDeveloper, UpdateDeveloper};
use crate::common::repository::base::EntityId;
use log::error;
use crate::common::{authentication::password_hash::hash_password, repository::{base::EmailConfirm, developers::models::DevEmailConfirm, error::SqlxError}};
use crate::common::emailer::emailer::EmailerSendService;

mod internal {
    use chrono::Utc;
    use sqlx::{query, PgConnection};
    use uuid::Uuid;      
    use crate::common::emailer::emailer::{CHANGE_EMAIL_CONFIRMATION_BODY, CHANGE_PASSWORD_CONFIRMATION_BODY, CHANGE_PASSWORD_SUBJECT, EMAIL_CONFIRMATION_SUBJECT, SIGNUP_EMAIL_CONFIRMATION_BODY};

    use super::*;    

    pub async fn insert_developer<E: EmailerSendService + Send + Sync>(conn: &Pool<Postgres>, new_developer: NewDeveloper, emailer: &E) -> Result<EntityId, Error> {
        let mut tx = conn.begin().await.unwrap();
        
        // todo: need to test min password length 8 and max 200
        let hashed_password = hash_password(&new_developer.password).unwrap();
        let insert_result = query_as::<_, EntityId>(
            r"
            insert into developer 
            (user_name, full_name, email, description, password, primary_lang_id) 
            values 
            ($1, $2, $3, $4, $5, $6)
            returning id
            ")
            .bind(new_developer.user_name)
            .bind(new_developer.full_name.clone())
            .bind(new_developer.email.clone())
            .bind(new_developer.description)
            .bind(hashed_password)
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

        match insert_email_confirm(&mut *tx, dev_id, EMAIL_CONFIRMATION_SUBJECT.to_string(), SIGNUP_EMAIL_CONFIRMATION_BODY.to_string(), new_developer.full_name, new_developer.email, emailer).await {
            Ok(_) => (),
            Err(e) => return Err(e)
        };

        _ = tx.commit().await;

        Ok(EntityId { id: dev_id })
    }
    
    /// note: Does NOT change password!
    pub async fn update_developer<E: EmailerSendService + Send + Sync>(conn: &Pool<Postgres>, update_developer: UpdateDeveloper, emailer: &E) -> Result<(), Error> {
        // need later to confirm does request change email?
        let existing_developer = query_developer(conn, update_developer.id).await.unwrap();

        let mut tx = conn.begin().await.unwrap();
                   
        // note: NOT updating email here
        // requires email confirmation by user first
        let update_result = query::<_>(
            r"
                update developer
                set updated_at = $2, full_name = $3, description = $4, primary_lang_id = $5
                where id = $1
            ")
            .bind(update_developer.id)
            .bind(Utc::now())
            .bind(update_developer.full_name.clone())
            .bind(update_developer.description)
            .bind(update_developer.primary_lang_id)
            .execute(&mut *tx)
            .await;

        let update_result = match update_result {
            Ok(row) => {
                if row.rows_affected() > 0 {
                    Ok(row)
                } else {
                    error!("update developer has failed");
                    Err(SqlxError::DatabaseQueryFailed)
                }
            },
            Err(e) => {
                error!("update developer error: {:?}", e);
                Err(SqlxError::DatabaseQueryFailed)
            }
        };
        if let Err(e) = update_result {
            return Err(e.into());
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
                            let _ = query::<_>(
                                r"
                                    delete from developers_secondary_langs
                                    where developer_id = $1
                                "
                            )
                            .bind(update_developer.id)
                            .execute(&mut *tx)
                            .await;
                        } else {                            
                            let _ = query::<_>(
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
                        }
                    }
                }
            },
            Err(e) => return Err(e)
        }

        if existing_developer.unwrap().email != update_developer.email {
            match insert_email_confirm(&mut *tx, update_developer.id, EMAIL_CONFIRMATION_SUBJECT.to_string(), CHANGE_EMAIL_CONFIRMATION_BODY.to_string(), update_developer.full_name, update_developer.email, emailer).await {
                Ok(_) => (),
                Err(e) => return Err(e)
            };
        }
        
        _ = tx.commit().await;

        Ok(())
    }

    pub async fn query_developer(conn: &Pool<Postgres>, id: i64) -> Result<Option<Developer>, Error> {
        query_as::<_, Developer>(
            r"
                select d.id, d.created_at, d.updated_at, d.user_name, d.full_name, d.email, d.description, d.password, d.primary_lang_id, dsl.secondary_lang_id
                from developer d left join developers_secondary_langs dsl on d.id = dsl.developer_id
                where d.id = $1
            ")
            .bind(id)
            .fetch_optional(conn).await
    }

    pub async fn query_developer_by_email(conn: &Pool<Postgres>, email: String) -> Result<Option<Developer>, Error> {
        query_as::<_, Developer>(
            r"
            select d.id, d.created_at, d.updated_at, d.user_name, d.full_name, d.email, d.description, d.password, d.primary_lang_id, dsl.secondary_lang_id
            from developer d left join developers_secondary_langs dsl on d.id = dsl.developer_id
            where d.email = $1
            ")
            .bind(email)
            .fetch_optional(conn).await
    }

    pub async fn query_developer_by_user_name(conn: &Pool<Postgres>, user_name: String) -> Result<Option<Developer>, Error> {
        query_as::<_, Developer>(
            r"
            select d.id, d.created_at, d.updated_at, d.user_name, d.full_name, d.email, d.description, d.password, d.primary_lang_id, dsl.secondary_lang_id
            from developer d left join developers_secondary_langs dsl on d.id = dsl.developer_id
            where d.user_name = $1
            ")
            .bind(user_name)
            .fetch_optional(conn).await
    }

    pub async fn query_all_developers(conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Developer>, Error> {
        query_as::<_, Developer>(
            r"
            select d.id, d.created_at, d.updated_at, d.user_name, d.full_name, d.email, d.description, d.password, d.primary_lang_id, dsl.secondary_lang_id
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

    pub async fn has_unconfirmed_email(conn: &Pool<Postgres>, email: String) -> Result<bool, Error> {
        match query_as::<_, EntityId>(r"
            select id from dev_email_confirmation where is_valid = true and is_confirmed = false and new_email = $1
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

    pub async fn query_latest_valid_email_confirm(conn: &Pool<Postgres>, dev_id: i64) -> Result<Option<DevEmailConfirm>, Error> {
        match query_as::<_, DevEmailConfirm>(r"
            select *
            from dev_email_confirmation 
            where 
                is_valid = true 
                and is_confirmed = false 
                and developer_id = $1
            order by updated_at desc
            limit 1
        ")
        .bind(dev_id)
        .fetch_optional(conn)
        .await {
            Ok(confirm) => Ok(confirm),
            Err(e) => Err(e)
        }
    }

    // whether the user is attempting to use their old email or their new email
    async fn insert_email_confirm<E: EmailerSendService + Send + Sync>(tx: &mut PgConnection, dev_id: i64, email_subject: String, email_body: String, dev_full_name: String, new_email: String, emailer: &E) -> Result<EmailConfirm, Error> {
        let uuid = Uuid::now_v7();
        match query_as::<_, EntityId>(r"
            insert into dev_email_confirmation
            (developer_id, is_confirmed, is_valid, new_email, unique_key)
            values
            ($1, false, true, $2, $3)
            returning id
        ")
        .bind(dev_id)
        .bind(new_email.clone())
        .bind(uuid)
        .fetch_one(tx)
        .await {
            Ok(entity) => {
                match emailer.send_email_confirm_requirement(true, true, dev_id, email_subject, email_body, dev_full_name, new_email, uuid).await {
                    Ok(_) => Ok(EmailConfirm {
                        entity,
                        unique_key: uuid
                    }),
                    Err(e) => Err(e.into())
                }
            },
            Err(e) => Err(e)
        }
    }

    pub async fn confirm_email(conn: &Pool<Postgres>, email: String, dev_id: i64, unique_key: String) -> Result<(), Error> {
        let mut tx = conn.begin().await.unwrap();

        #[allow(unused)]
        let mut uuid: Option<Uuid> = None;
        match Uuid::parse_str(&unique_key) {
            Ok(_uuid) => uuid = Some(_uuid),
            Err(_) => return Err(SqlxError::EmailConfirmInvalidUniqueKey.into())
        }
        let uuid = uuid.unwrap();

        // first see if email already confirmed
        match query_as::<_, DevEmailConfirm>(
            r"
                select * 
                from dev_email_confirmation 
                where 
                is_confirmed = true 
                and is_valid = false 
                and developer_id = $1 
                and new_email = $2 
                and unique_key = $3
                order by updated_at desc 
                limit 1
            "
        )
        .bind(dev_id)
        .bind(email.clone())
        .bind(uuid)
        .fetch_optional(&mut *tx)
        .await {
            Ok(row) => if row.is_some() {
                return Err(SqlxError::EmailAlreadyConfirmed.into())
            },
            Err(_) => ()
        };

        #[allow(unused)]
        let mut current_email_confirm: Option<DevEmailConfirm> = None;
        match query_as::<_, DevEmailConfirm>(
            r"
                select * 
                from dev_email_confirmation 
                where 
                is_confirmed = false 
                and is_valid = true 
                and developer_id = $1 
                and new_email = $2 
                and unique_key = $3
                order by updated_at desc 
                limit 1
            "
        )
        .bind(dev_id)
        .bind(email)
        .bind(uuid)
        .fetch_one(&mut *tx)
        .await {
            Ok(confirm) => if confirm.is_valid {
                current_email_confirm = Some(confirm.clone());
                // see if later email confirm exists
                match query_as::<_, DevEmailConfirm>(
                    "select * from dev_email_confirmation where developer_id = $1 and is_valid = true and updated_at > $2"                
                )
                .bind(dev_id)
                .bind(confirm.updated_at)
                .fetch_all(&mut *tx)
                .await {
                    Ok(found_newer_confirms) => if found_newer_confirms.len() > 0 {
                        return Err(SqlxError::EmailConfirmFailedNewerExists.into());
                    },
                    Err(e) => return Err(e)
                }                
            } else {
                return Err(SqlxError::EmailConfirmFailed.into());
            },
            Err(e) => return Err(e)
        };

        let current_email_confirm_id = current_email_confirm.clone().unwrap().id;
        // get all email change attempts prior to this one and invalidate them
        match query::<_>(
            r"
            update dev_email_confirmation 
            set is_valid = false, updated_at = $3
            where developer_id = $2 and id <> $1
            "
        )
        .bind(current_email_confirm_id)
        .bind(dev_id)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await {
            Ok(_) => (),
            Err(e) => return Err(e)
        };
        
        match query::<_>(r"
            update dev_email_confirmation
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

        match query::<_>("update developer set email = $2, updated_at = $3 where id = $1")
            .bind(dev_id)
            .bind(current_email_confirm.unwrap().new_email)
            .bind(Utc::now())
            .execute(&mut *tx)
            .await {
                Ok(row) => if row.rows_affected() > 0 {
                    _ = tx.commit().await;
                    Ok(())
                } else {
                    Err(SqlxError::EmailConfirmForProfileUpdateFailed.into())
                },
                Err(e) => Err(e)
            }
    }

    pub async fn insert_forgot_password_confirm<E: EmailerSendService + Send + Sync>(
        conn: &Pool<Postgres>, email: String, emailer: &E
    ) -> Result<EmailConfirm, Error> {
        match query_developer_by_email(conn, email.clone()).await {
            Ok(opt_dev) => match opt_dev {
                Some(dev) => {
                    let uuid = Uuid::now_v7();
                    match query_as::<_, EntityId>(r"
                        insert into dev_forgot_password_confirmation
                        (developer_id, is_confirmed, is_valid, unique_key)
                        values
                        ($1, false, true, $2)
                        returning id
                    ")
                    .bind(dev.id)
                    .bind(uuid)
                    .fetch_one(conn)
                    .await {
                        Ok(entity) => {
                            match emailer.send_email_confirm_requirement(
                                false, true, dev.id, CHANGE_PASSWORD_SUBJECT.to_string(), CHANGE_PASSWORD_CONFIRMATION_BODY.to_string(), dev.full_name, email, uuid
                            ).await {
                                Ok(_) => Ok(EmailConfirm {
                                    entity,
                                    unique_key: uuid
                                }),
                                Err(e) => Err(e.into())
                            }                
                        },
                        Err(e) => Err(e)
                    }
                },
                None => Err(SqlxError::UserNotFoundByEmail.into())
            },
            Err(e) => Err(e)
        }
    }
}

#[async_trait]
pub trait InsertDeveloperFn<E: EmailerSendService + Send + Sync> {
    async fn insert_developer(&self, new_developer: NewDeveloper, emailer: &E) -> Result<EntityId, Error>;
}

#[async_trait]
impl<E: EmailerSendService + Send + Sync> InsertDeveloperFn<E> for DbRepo {
    async fn insert_developer(&self, new_developer: NewDeveloper, emailer: &E) -> Result<EntityId, Error> {
        internal::insert_developer(self.get_conn(), new_developer, emailer).await
    }
}

#[async_trait]
pub trait UpdateDeveloperFn<E: EmailerSendService + Send + Sync> {
    async fn update_developer(&self, update_developer: UpdateDeveloper, emailer: &E) -> Result<(), Error>;
}

#[async_trait]
impl<E: EmailerSendService + Send + Sync> UpdateDeveloperFn<E> for DbRepo {
    async fn update_developer(&self, update_developer: UpdateDeveloper, emailer: &E) -> Result<(), Error> {
        internal::update_developer(self.get_conn(), update_developer, emailer).await
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
pub trait QueryDeveloperByUserNameFn {
    async fn query_developer_by_user_name(&self, user_name: String) -> Result<Option<Developer>, Error>;
}

#[async_trait]
impl QueryDeveloperByUserNameFn for DbRepo {
    async fn query_developer_by_user_name(&self, user_name: String) -> Result<Option<Developer>, Error> {
        internal::query_developer_by_user_name(self.get_conn(), user_name).await
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

#[async_trait]
pub trait HasUnconfirmedDevEmailFn {
    async fn has_unconfirmed_dev_email(&self, email: String) -> Result<bool, Error>;
}

#[async_trait]
impl HasUnconfirmedDevEmailFn for DbRepo {
    async fn has_unconfirmed_dev_email(&self, email: String) -> Result<bool, Error> {
        internal::has_unconfirmed_email(self.get_conn(), email).await
    }
}

#[async_trait]
pub trait QueryLatestValidEmailConfirmFn {
    async fn query_latest_valid_email_confirm(&self, dev_id: i64) -> Result<Option<DevEmailConfirm>, Error>;
}

#[async_trait]
impl QueryLatestValidEmailConfirmFn for DbRepo {
    async fn query_latest_valid_email_confirm(&self, dev_id: i64) -> Result<Option<DevEmailConfirm>, Error> {
        internal::query_latest_valid_email_confirm(self.get_conn(), dev_id).await
    }
}

#[async_trait]
pub trait ConfirmDevEmailFn {
    async fn confirm_dev_email(&self, email: String, dev_id: i64, unique_key: String) -> Result<(), Error>;
}

#[async_trait]
impl ConfirmDevEmailFn for DbRepo {
    async fn confirm_dev_email(&self, email: String, dev_id: i64, unique_key: String) -> Result<(), Error> {
        internal::confirm_email(self.get_conn(), email, dev_id, unique_key).await
    }
}

#[async_trait]
pub trait InsertDevForgotPasswordConfirmFn<E: EmailerSendService + Send + Sync> {
    async fn insert_dev_forgot_password_confirm(&self, email: String, emailer: &E) -> Result<EmailConfirm, Error>;
}

#[async_trait]
impl<E: EmailerSendService + Send + Sync> InsertDevForgotPasswordConfirmFn<E> for DbRepo {
    async fn insert_dev_forgot_password_confirm(&self, email: String, emailer: &E) -> Result<EmailConfirm, Error> {
        internal::insert_forgot_password_confirm(self.get_conn(), email, emailer).await
    }
}