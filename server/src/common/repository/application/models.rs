use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Application {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub job_id: i64,
    pub developer_id: i64
}

pub struct NewApplication {
    pub job_id: i64,
    pub developer_id: i64
}