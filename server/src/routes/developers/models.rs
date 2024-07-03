use actix_http::body::BoxBody;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use actix_web::{Responder, HttpResponse, http::header::ContentType};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDeveloperForRoute {
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRoute {
    /// developer id
    pub id: i64,
    pub old_password: String,
    pub new_password: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDeveloperForRoute {
    pub id: i64,
    pub full_name: String,
    pub email: String,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperResponder {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub primary_lang_id: i64,
    pub secondary_lang_id: Option<i64>
}

impl Responder for DeveloperResponder {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize DeveloperResponder")
        }
    }
}

#[derive(Serialize)]
pub struct DeveloperResponders(pub Vec<DeveloperResponder>);

impl Responder for DeveloperResponders {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize DeveloperResponders")
        }
    }
}