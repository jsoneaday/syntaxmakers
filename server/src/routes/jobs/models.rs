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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateJobForRoute {
    pub id: i64,
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

#[derive(Serialize, Clone)]
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
    pub secondary_lang_id: Option<i64>,
    pub secondary_lang_name: Option<String>,
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

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobAppliedResponder {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    /// this is the Application's created_at value
    pub dev_applied_at: DateTime<Utc>,
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
    pub secondary_lang_id: Option<i64>,
    pub secondary_lang_name: Option<String>,
    pub industry_id: i64,
    pub industry_name: String,
    pub salary_id: i64,
    pub salary: i32
}

impl Responder for JobAppliedResponder {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);
        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize JobAppliedResponder")
        }
    }
}

#[derive(Serialize)]
pub struct JobAppliedResponders(pub Vec<JobAppliedResponder>);

impl Responder for JobAppliedResponders {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);
        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize JobAppliedResponders")
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobAndApplicantResponder {
    pub job_id: i64,
    pub job_updated_at: DateTime<Utc>,
    pub applied_at: DateTime<Utc>,
    pub dev_id: i64,
    pub dev_full_name: String,
    pub dev_description: String,
    pub job_title: String,
    pub dev_primary_lang_id: i64,
    pub dev_primary_lang_name: String,
    pub dev_secondary_lang_id: i64,
    pub dev_secondary_lang_name: String
}

impl Responder for JobAndApplicantResponder {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);
        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize JobAndApplicantResponder")
        }
    }
}

#[derive(Serialize)]
pub struct JobAndApplicantResponders(pub Vec<JobAndApplicantResponder>);

impl Responder for JobAndApplicantResponders {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);
        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize JobAndApplicantResponders")
        }
    }
}