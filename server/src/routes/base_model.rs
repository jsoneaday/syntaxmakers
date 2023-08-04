use serde::Deserialize;

pub struct OutputId {
    pub id: i64
}

#[derive(Deserialize)]
pub struct PagingModel {
    pub page_size: i32,
    pub last_offset: i64
}


#[derive(Deserialize)]
pub struct IdAndPagingModel {
    pub id: i64,
    pub page_size: i32,
    pub last_offset: i64
}