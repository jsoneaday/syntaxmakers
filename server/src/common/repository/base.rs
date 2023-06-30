use log::info;
use sqlx::{Pool, Postgres, migrate, FromRow};
use std::env;
use dotenv::dotenv;

#[derive(FromRow)]
pub struct EntityId {
    pub id: i64
}

pub trait Repository{}

pub struct DbRepo {
    conn: Pool<Postgres>
}

impl DbRepo {
    pub async fn init() -> Self {
        DbRepo {
            conn: get_conn().await
        }
    }
}

impl Repository for DbRepo{}

pub trait ConnGetter {
    type Output;

    fn get_conn(&self) -> &Self::Output;
}

impl ConnGetter for DbRepo {
    type Output = Pool<Postgres>;

    fn get_conn(&self) -> &Self::Output {
        &self.conn
    }
}

async fn get_conn() -> Pool<Postgres> {
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
            info!("testing log");
            println!("migration complete");
        },
        Err(e) => println!("failed to migrate {}", e)
    };

    conn
}