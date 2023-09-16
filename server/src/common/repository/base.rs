use log::{info, error};
use sqlx::{Pool, Postgres, migrate, FromRow};
use std::env;
use dotenv::dotenv;
use async_trait::async_trait;

#[derive(FromRow)]
pub struct CountResult {
    pub count: i64,
}

#[derive(FromRow, Clone)]
pub struct EntityId {
    pub id: i64
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
    let postgres_host = env::var("POSTGRES_HOST").unwrap();
    let postgres_port = env::var("POSTGRES_PORT").unwrap().parse::<u16>().unwrap();
    let postgres_password = env::var("POSTGRES_PASSWORD").unwrap();
    let postgres_user = env::var("POSTGRES_USER").unwrap();
    let postgres_db = env::var("POSTGRES_DB").unwrap();

    let postgres_url = format!(
        "postgres://{postgres_user}:{postgres_password}@{postgres_host}:{postgres_port}/{postgres_db}"
    );
    info!("connection string {}", postgres_url);

    let conn = sqlx::postgres::PgPool::connect(&postgres_url).await.unwrap();
    let migrate_result = migrate!("./migrations").run(&conn).await;
    match migrate_result {
        Ok(()) => {
            info!("migration complete");
        },
        Err(e) => error!("failed to migrate {}", e)
    };

    conn
}