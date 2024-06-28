use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewApplicationForRoute {
    pub job_id: i64,
    pub developer_id: i64
}

