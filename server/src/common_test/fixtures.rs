use std::sync::OnceLock;
use actix_http::header::HeaderValue;
use actix_web::http::header;
use actix_web::{HttpRequest, test};
use fake::Fake;
use fake::faker::lorem::en::Sentence;
use fake::faker::name::en::{FirstName, LastName};
use jsonwebtoken::EncodingKey;
use serde::Serialize;
use crate::app_state::AppState;
use crate::common::authentication::auth_service::{init_auth_keys, get_token};
use crate::common::repository::base::Repository;
use async_trait::async_trait;
pub static COUNTRY_NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();
pub static INDUSTRY_NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();
pub static LANGUAGE_NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();
pub static SALARY_BASE: OnceLock<Vec<i32>> = OnceLock::new();

pub fn init_fixtures() {
    COUNTRY_NAMES.get_or_init(|| {
        vec![
            "United States" 
        ]
    });

    INDUSTRY_NAMES.get_or_init(|| {
        vec![
            "Finance",
            "Blockchain" ,
            "AI/ML",
            "Games"
        ]
    });

    LANGUAGE_NAMES.get_or_init(|| {
        vec![
            "Rust",
            "Go" ,
            "Ruby",
            "Swift",
            "Kotlin",
            "Scala",
            "Elixir"
        ]
    });
    SALARY_BASE.get_or_init(|| {
        vec![
            200_000,
            300_000,
            400_000,
            500_000
        ]
    });
}

pub async fn get_app_data<T: Repository>(repo: T) -> actix_web::web::Data<AppState<T>> {
    actix_web::web::Data::new(AppState { repo, auth_keys: init_auth_keys().await })
}

pub fn get_fake_fullname() -> String {
    format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>())
}

pub fn get_fake_title() -> String {
    Sentence(5..6).fake::<String>()
}

pub fn get_fake_desc() -> String {
    Sentence(9..10).fake::<String>()
}

pub struct MockDbRepo;

#[async_trait]
impl Repository for MockDbRepo {
    async fn init() -> Self {
        MockDbRepo
    }
}

pub fn get_fake_request_with_bearer_token(user_name: String, encoding_key: &EncodingKey, url: &str, data: impl Serialize) -> HttpRequest {
    let header_value_string = format!("Bearer {}", get_token(user_name, encoding_key, None));
    let header_value = HeaderValue::from_str(&header_value_string).unwrap();
    test::TestRequest
        ::post()
        .append_header((header::AUTHORIZATION, header_value.clone()))
        .uri(url)
        .set_json(data)
        .to_http_request()
}