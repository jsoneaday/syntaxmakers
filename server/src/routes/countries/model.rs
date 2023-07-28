use actix_http::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use chrono::{Utc, DateTime};
use serde::Serialize;

#[derive(Serialize)]
pub struct CountryResponder {
    pub id: i64,    
    pub updated_at: DateTime<Utc>,
    pub name: String
}

impl Responder for CountryResponder {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);
        match json_result {
            Ok(body) => HttpResponse::Ok().content_type(ContentType::json()).body(body),
            Err(_) => HttpResponse::InternalServerError().content_type(ContentType::json()).body("Failed to serialize CountryResponder")
        }
    }
}

#[derive(Serialize)]
pub struct CountryResponders(pub Vec<CountryResponder>);

impl Responder for CountryResponders {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);
        match json_result {
            Ok(body) => HttpResponse::Ok().content_type(ContentType::json()).body(body),
            Err(_) => HttpResponse::InternalServerError().content_type(ContentType::json()).body("Failed to serialize CountryResponders")
        }
    }
}