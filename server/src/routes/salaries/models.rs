use actix_http::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SalaryResponder {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub base: i32
}

impl Responder for SalaryResponder {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body),
            Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("Failed to serialize SalaryResponder")
        }
    }
}

#[derive(Serialize)]
pub struct SalaryResponders(pub Vec<SalaryResponder>);

impl Responder for SalaryResponders {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body),
            Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("Failed to serialize SalaryResponders")
        }
    }
}