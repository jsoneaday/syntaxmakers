use actix_http::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use chrono::{Utc, DateTime};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryResponder {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub name: String
}

impl Responder for IndustryResponder {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_body = serde_json::to_string(&self);

        match json_body {
            Ok(body) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body),
            Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("Failed to serialize IndustryResponder")
        }
    }
}

#[derive(Serialize)]
pub struct IndustryResponders(pub Vec<IndustryResponder>);

impl Responder for IndustryResponders {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_body = serde_json::to_string(&self);

        match json_body {
            Ok(body) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body),
            Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("Failed to serialize IndustryResponder")
        }
    }
}