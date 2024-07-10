use actix_http::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OutputId {
    pub id: i64
}

impl Responder for OutputId {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize OutputId")
        }
    }
}

#[derive(Serialize)]
pub struct OutputBool {
    pub result: bool
}

impl Responder for OutputBool {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);
        match json_result {
            Ok(body) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body),
            Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("Failed to serialize OutputBool")
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PagingModel {
    pub page_size: i32,
    pub last_offset: i64
}


#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdAndPagingModel {
    pub id: i64,
    pub page_size: i32,
    pub last_offset: i64
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchAndPagingModel {
    pub search_terms: Vec<String>,
    pub page_size: i32,
    pub last_offset: i64
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchForEmpAndPagingModel {
    pub emp_id: i64,
    pub search_terms: Vec<String>,
    pub page_size: i32,
    pub last_offset: i64
}