use actix_http::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OutputId {
    pub id: i64
}

impl Responder for OutputId {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize OutputId")
        }
    }
}

#[derive(Deserialize)]
pub struct PagingModel {
    pub page_size: i32,
    pub last_offset: i64
}


#[derive(Deserialize)]
pub struct IdAndPagingModel {
    pub id: i64,
    pub page_size: i32,
    pub last_offset: i64
}