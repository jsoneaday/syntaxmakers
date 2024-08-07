use log::{info, error};
use sqlx::{Pool, Postgres, migrate, FromRow};
use std::env;
use dotenv::dotenv;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(FromRow)]
pub struct CountResult {
    pub count: i64,
}

#[derive(FromRow, Clone, Debug)]
pub struct EntityId {
    pub id: i64
}

#[derive(Clone, Debug)]
pub struct EmailConfirm {
    pub entity: EntityId,
    pub unique_key: Uuid
}

/// Migration version
#[derive(FromRow, Clone, Debug)]
pub struct Version {
    pub version: i64
}

#[async_trait]
pub trait Repository{
    async fn init() -> Self;
}

#[derive(Debug, Clone)]
pub struct DbRepo {
    conn: Pool<Postgres>
}

#[async_trait]
impl Repository for DbRepo {
    async fn init() -> Self {
        DbRepo {
            conn: get_conn_pool().await
        }
    }
}

pub trait ConnGetter: Repository {
    type Output;

    fn get_conn(&self) -> &Self::Output;
}

impl ConnGetter for DbRepo {
    type Output = Pool<Postgres>;

    fn get_conn(&self) -> &Self::Output {
        &self.conn
    }
}

async fn get_conn_pool() -> Pool<Postgres> {
    dotenv().ok();
    
    let postgres_port = env::var("POSTGRES_PORT").unwrap().parse::<u16>().unwrap();
    let postgres_password = env::var("POSTGRES_PASSWORD").unwrap();
    let postgres_user = env::var("POSTGRES_USER").unwrap();
    let postgres_db = env::var("POSTGRES_DB").unwrap();
    let env = env::var("ENV").unwrap();
    let postgres_host = if env == "production" {
        env::var("POSTGRES_HOST").unwrap()
    } else {
        format!("{}:{}", env::var("POSTGRES_HOST").unwrap(), postgres_port)
    };
    let ssl_mode = if env == "production" {
        "?sslmode=require"
    } else {
        ""
    };

    let postgres_url = format!(
        "postgres://{postgres_user}:{postgres_password}@{postgres_host}/{postgres_db}{ssl_mode}"
    );
    // info!("DB connection string: {}", postgres_url);

    let conn = sqlx::postgres::PgPool::connect(&postgres_url).await.unwrap();
    let result = sqlx::query_as::<_, Version>(r"
        select version
        from _sqlx_migrations
        where version = $1
    ")
    .bind(20230623210040 as i64) // todo: need to improve this
    .fetch_optional(&conn)
    .await;

    if let Ok(version) = result {
        if let Some(_) = version {
            info!("migration was already completed");
            return conn;
        }
    }

    info!("migration started");
    let migrate_result = migrate!("./migrations").run(&conn).await;
    info!("end migration {:?}", migrate_result);
    match migrate_result {
        Ok(()) => {
            info!("migration complete");
        },
        Err(e) => error!("failed to migrate {}", e)
    };    

    conn
}