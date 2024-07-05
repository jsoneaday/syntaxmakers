use actix_http::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


/// If company_id is none new_company_name must have a value,
/// as that situation is considered a request to create a new company
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewEmployerForRoute {
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub company_id: Option<i64>,
    pub new_company_name: Option<String>
}

/// If company_id is none new_company_name must have a value,
/// as that situation is considered a request to create a new company
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmployerForRoute {
    pub id: i64,
    pub full_name: String,
    pub email: String,
    pub company_id: Option<i64>,
    pub new_company_name: Option<String>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployerResponder {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub company_id: i64
}

impl Responder for EmployerResponder {
    type Body = BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body),
            Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("Failed to serialize EmployerResponder")
        }
    }
}

#[derive(Serialize)]
pub struct EmployerResponders(pub Vec<EmployerResponder>);

impl Responder for EmployerResponders {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body),
            Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("Failed to serialize EmployerResponder")
        }
    }
}