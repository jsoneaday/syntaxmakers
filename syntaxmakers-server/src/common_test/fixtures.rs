use std::sync::OnceLock;

use fake::Fake;
use fake::faker::name::en::{FirstName, LastName};

use crate::app_state::AppState;
use crate::common::repository::base::{DbRepo};

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
            "Crypto/Blockchain" ,
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

pub async fn get_app_data(repo: DbRepo) -> actix_web::web::Data<AppState<DbRepo>> {
    actix_web::web::Data::new(AppState { repo })
}

pub fn get_fake_fullname() -> String {
    format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>())
}