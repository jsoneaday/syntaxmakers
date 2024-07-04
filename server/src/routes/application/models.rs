use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewApplicationForRoute {
    pub job_id: i64,
    pub developer_id: i64
}

