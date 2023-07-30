use actix_http::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use chrono::{DateTime, Utc};
use serde::Serialize;


#[derive(Serialize)]
pub struct LanguageResponder {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub name: String
}

impl Responder for LanguageResponder {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body),
            Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("Failed to serialize LanguageResponder")
        }
    }
}

#[derive(Serialize)]
pub struct LanguageResponders(pub Vec<LanguageResponder>);

impl Responder for LanguageResponders {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body),
            Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("Failed to serialize LanguageResponder")
        }
    }
}