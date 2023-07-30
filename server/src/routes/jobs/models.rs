use actix_http::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewJobForRoute {
    pub employer_id: i64,
    pub title: String,
    pub description: String,
    pub is_remote: bool,
    pub headquarters_country_id: Option<i64>,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>,
    pub industry_id: i64,
    pub salary_id: i64
}

#[derive(Serialize)]
pub struct JobResponder {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub employer_id: i64,
    pub title: String,
    pub description: String,
    pub is_remote: bool,
    pub headquarters_country_id: i64,
    pub primary_lang_id: i64,
    pub secondary_lang_id: i64,
    pub industry_id: i64,
    pub salary_id: i64
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