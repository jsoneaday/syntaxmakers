use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewApplicationForRoute {
    pub job_id: i64,
    pub developer_id: i64
}