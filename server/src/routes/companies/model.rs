use actix_http::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct NewCompany {
    pub name: String
}

#[derive(Serialize)]
pub struct CompanyResponder {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub name: String
}

impl Responder for CompanyResponder {
    type Body = BoxBody;
    
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(json) => HttpResponse::Ok().content_type(ContentType::json()).body(json),
            Err(_) => HttpResponse::InternalServerError().content_type(ContentType::json()).body("Failed to serialize CompanyResponder")
        }
    }
}

#[derive(Serialize)]
pub struct CompanyResponders(pub Vec<CompanyResponder>);

impl Responder for CompanyResponders {
    type Body = BoxBody;
    
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(json) => HttpResponse::Ok().content_type(ContentType::json()).body(json),
            Err(_) => HttpResponse::InternalServerError().content_type(ContentType::json()).body("Failed to serialize CompanyResponders")
        }
    }
}