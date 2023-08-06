use actix_http::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewJobForRoute {
    pub employer_id: i64,
    pub title: String,
    pub description: String,
    pub is_remote: bool,
    pub country_id: Option<i64>,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>,
    pub industry_id: i64,
    pub salary_id: i64
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobResponder {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub employer_id: i64,
    pub employer_name: String,
    pub company_id: i64,
    pub company_name: String,
    pub company_logo: Option<Vec<u8>>,
    pub title: String,
    pub description: String,
    pub is_remote: bool,
    pub country_id: Option<i64>,
    pub country_name: Option<String>,
    pub primary_lang_id: i64,
    pub primary_lang_name: String,
    pub secondary_lang_id: i64,
    pub secondary_lang_name: String,
    pub industry_id: i64,
    pub industry_name: String,
    pub salary_id: i64,
    pub salary: i32
}

impl Responder for JobResponder {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);
        match json_result {
            Ok(body) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body),
            Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("Failed to serialize JobResponder")
        }
    }
}

#[derive(Serialize)]
pub struct JobResponders(pub Vec<JobResponder>);

impl Responder for JobResponders {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);
        match json_result {
            Ok(body) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body),
            Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("Failed to serialize JobResponders")
        }
    }
}