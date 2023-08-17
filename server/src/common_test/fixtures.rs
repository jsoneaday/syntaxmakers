use std::sync::OnceLock;
use actix_http::header::HeaderValue;
use actix_web::cookie::Cookie;
use actix_web::http::header;
use actix_web::web::Bytes;
use actix_web::{HttpRequest, test};
use fake::Fake;
use fake::faker::lorem::en::Sentence;
use fake::faker::name::en::{FirstName, LastName};
use jsonwebtoken::{EncodingKey, DecodingKey};
use serde::Serialize;
use crate::app_state::AppState;
use crate::common::authentication::auth_service::{init_auth_keys, get_token, Authenticator, AuthenticationError};
use crate::common::fs_utils::get_file_buffer;
use crate::common::repository::base::Repository;
use crate::common::repository::user::models::DeveloperOrEmployer;
use async_trait::async_trait;
pub static COUNTRY_NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();
pub static INDUSTRY_NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();
pub static LANGUAGE_NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();
pub static SALARY_BASE: OnceLock<Vec<i32>> = OnceLock::new();
use actix_http::body::BoxBody;

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
            "C#",
            "Java",
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

pub async fn get_app_data<T: Repository, U: Authenticator>(repo: T, auth_service: U) -> actix_web::web::Data<AppState<T, U>> {
    actix_web::web::Data::new(AppState { repo, auth_service, auth_keys: init_auth_keys().await })
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

pub struct MockAuthService;
#[async_trait]
impl Authenticator for MockAuthService {
    async fn is_authenticated(&self, _: String, _: Vec<(&str, &str)>, _: &DecodingKey) -> Result<bool, AuthenticationError> {
        Ok(true)
    }
}

pub struct MockDbRepo;

#[async_trait]
impl Repository for MockDbRepo {
    async fn init() -> Self {
        MockDbRepo
    }
}

pub fn get_fake_httprequest_with_bearer_token(
    user_name: String, 
    dev_or_emp: DeveloperOrEmployer, 
    encoding_key: &EncodingKey, 
    url: &str, 
    parameter_data: impl Serialize, 
    token_expiration_duration: Option<i64>,
    cookie: Option<Cookie>
) -> HttpRequest {
    let header_value_string = format!("Bearer {}", get_token(user_name, dev_or_emp, encoding_key, token_expiration_duration));
    let header_value = HeaderValue::from_str(&header_value_string).unwrap();
    let req = test::TestRequest
        ::post()
        .append_header((header::AUTHORIZATION, header_value.clone()))
        .uri(url)
        .set_json(parameter_data);     
        
    if let Some(cookie) = cookie {
        let req = req.cookie(cookie)
            .to_http_request();
        req
    } else {
        let req = req.to_http_request();
        req
    }
}

pub fn get_httpresponse_body_as_string(body_bytes_result: Result<Bytes, BoxBody>) -> String {    
    match body_bytes_result {
        Ok(body_bytes) => {
            let body_str = String::from_utf8(body_bytes.to_vec());
            match body_str {
                Ok(token) => {
                    println!("token {}", token);
                    token
                },
                Err(_) => "".to_string()
            } 
        },
        Err(_) => "".to_string()
    }
}

pub fn get_company_log_randomly() -> Vec<u8> {
    use rand::Rng;
    
    let file_no = rand::thread_rng().gen_range(1..7);
    let file_path = format!("src/common_test/files/office-cl-{}.png", file_no);
    println!("file_path for logo {}", file_path);
    get_file_buffer(&file_path)
}