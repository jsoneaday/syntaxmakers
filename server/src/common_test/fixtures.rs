use std::sync::OnceLock;
use actix_http::header::HeaderValue;
use actix_web::cookie::Cookie;
use actix_web::http::header;
use actix_web::web::Bytes;
use actix_web::{HttpRequest, test};
use fake::Fake;
use fake::faker::company::en::CompanyName;
use fake::faker::internet::en::{Username, FreeEmail};
use fake::faker::name::en::{FirstName, LastName};
use jsonwebtoken::{EncodingKey, DecodingKey};
use serde::Serialize;
use crate::app_state::AppState;
use crate::common::authentication::auth_service::{init_auth_keys, get_token, Authenticator, AuthenticationError};
use crate::common::fs_utils::get_file_buffer;
use crate::common::rand_utils::get_random_no_from_range;
use crate::common::repository::base::{Repository, DbRepo};
use crate::common::repository::companies::models::NewCompany;
use crate::common::repository::companies::repo::InsertCompanyFn;
use crate::common::repository::countries::models::Country;
use crate::common::repository::countries::repo::QueryAllCountriesFn;
use crate::common::repository::employers::models::NewEmployer;
use crate::common::repository::employers::repo::InsertEmployerFn;
use crate::common::repository::industries::models::Industry;
use crate::common::repository::industries::repo::QueryAllIndustriesFn;
use crate::common::repository::jobs::models::NewJob;
use crate::common::repository::jobs::repo::{InsertJobFn, QueryJobsCountFn};
use crate::common::repository::languages::models::Language;
use crate::common::repository::languages::repo::QueryAllLanguagesFn;
use crate::common::repository::salaries::repo::QueryAllSalariesFn;
use crate::common::repository::salaries::models::Salary;
use crate::common::repository::user::models::DeveloperOrEmployer;
use async_trait::async_trait;

pub static COUNTRIES: OnceLock<Vec<Country>> = OnceLock::new();
pub static INDUSTRIES: OnceLock<Vec<Industry>> = OnceLock::new();
pub static LANGUAGES: OnceLock<Vec<Language>> = OnceLock::new();
pub static SALARY_BASE: OnceLock<Vec<Salary>> = OnceLock::new();
pub static FAKE_JOB_TITLES: OnceLock<Vec<&str>> = OnceLock::new();
pub static FAKE_JOB_DESC: OnceLock<Vec<&str>> = OnceLock::new(); 
use actix_http::body::BoxBody;

pub async fn init_fixtures() {    
    let repo = DbRepo::init().await;
    
    if let None = COUNTRIES.get() {
        let countries = repo.query_all_countries().await.unwrap();
        COUNTRIES.get_or_init(|| countries);
    }
    if let None = INDUSTRIES.get() {
        let industries = repo.query_all_industries().await.unwrap();
        INDUSTRIES.get_or_init(|| industries);
    }
    if let None = LANGUAGES.get() {
        let languages = repo.query_all_languages().await.unwrap();
        LANGUAGES.get_or_init(|| languages);
    }
    if let None = SALARY_BASE.get() {
        let salaries = repo.query_all_salaries().await.unwrap();
        SALARY_BASE.get_or_init(|| salaries);   
    }
    if let None = FAKE_JOB_TITLES.get() {
        FAKE_JOB_TITLES.get_or_init(|| {
            vec![
                "Senior Web Developer",
                "Senior Full-Stack Developer",
                "Senior Java Developer",
                "Senior TypeScript Engineer",
                "Senior NodeJS Developer",
                "Full-Stack Python Developer",
                "Senior C# Developer",
                "Full-Stack Engineer"
            ]
        });
    }
    if let None = FAKE_JOB_DESC.get() {
        FAKE_JOB_DESC.get_or_init(|| {
            vec![
                r"
                About the job
                As a Senior Mobile Engineer, you will be one of the first 5 engineers at our client, responsible for building ambitious new features end-to-end while ensuring the client's app remains performant and bug-free. Your contributions will have an enormous and long-lasting impact. Users love our client's mobile app (4.9 stars from 350+ App Store reviews), and you will be relentless to continue raising the bar for field sales software.
            
            
                Location (Hybrid)
            
                New York, NY
            
            
                Qualifications (Must):
            
                4-10 years of professional engineering experience
                Fluent in React and Typescript
                Proven track record of high-performance
                Able to work 3 days in person in NYC
            
            
                Qualifications (Preferred):
            
                Worked at a company with < 300 employees
                Founding Engineer Experience
                Mobile Engineering experience
            
            
                Responsibilities
            
                Architect and assemble core features, including novel interfaces powered by LLMs on the way to building a world-class AI coach and sales co-pilot
                Write great code quickly and be counted on to deploy with minimal code review
                Monitor analytics and communicate directly with customer-facing teams to Identify, communicate, and resolve bugs and performance issues
                Iterate and improve DevOps in preparation for the rapid scaling of both our engineering team and our user base
                Work directly with the founders, Joe and Jake, daily and be counted on to tell them when they’re wrong
            
            
                Keywords
            
                Frontend, ReactJS, TypeScript, AI, Artificial Intelligence, Mobile Engineering, DevOps, Front-End.
                ",
                r"
                About the job
                How is this unique data platform about to scale to the next Billion Dollars Unicorn out of SF?
            
            
            
                Could you be the Frontend Tech Lead to dictate the success of the frontend strategy of this data privacy and governance platform?
            
            
            
                If so, this is the perfect Frontend Tech Lead role for you.
            
            
                This company is at the helm of an emerging market in data privacy and governance for businesses. The platform covers all regulations in this ever-growingly data sensitive environment where fundamentally, privacy matters.
            
            
                You will be joining an emerging market at a company that has achieved 0-1 and now has huge growth ambitions for the future. The CEO & CTO have huge success in scaling companies, with their last ventures being acquired by FAANG for $100 M's.
            
            
                You will be in charge of the Frontend Strategy of this platform as it massively upscales. Your role will consist of a mix of team leadership, Frontend strategy and architecture and diving in hands on too. 
            
            
            
                Get ready to role your sleeves up.
                "
            ]
        });
    }

    setup_data().await;
}

async fn setup_data() {
    let repo = DbRepo::init().await;
    let jobs = repo.query_all_jobs_count().await.unwrap().count;
    
    if jobs == 0 {
        let logo = get_company_logo_randomly();
        let company_create_result = repo.insert_company(NewCompany{ 
            name: CompanyName().fake::<String>(), 
            logo: Some(logo), 
            headquarters_country_id: 1 
        }).await.unwrap();
        let company_id = company_create_result.id;

        let email_prefix = get_random_no_from_range(0, 100);
        let create_employer_result = repo.insert_employer(NewEmployer {
            user_name: Username().fake::<String>(),
            full_name: get_fake_fullname(),
            email: format!("{}{}", email_prefix, FreeEmail().fake::<String>()),
            password: "test123".to_string(),
            company_id
        }).await.unwrap();

        for _ in 1..40 {          
            repo.insert_job(NewJob {
                employer_id: create_employer_result.id,
                title: get_fake_title().to_string(),
                description: get_fake_desc().to_string(),
                is_remote: true,
                country_id: None,
                primary_lang_id: get_random_language().await.id,
                secondary_lang_id: Some(get_random_language().await.id),
                industry_id: get_random_industry().await.id,
                salary_id: get_random_salary().await.id
            }).await.unwrap();
        }
    }
}

pub async fn get_app_data<T: Repository, U: Authenticator>(repo: T, auth_service: U) -> actix_web::web::Data<AppState<T, U>> {
    actix_web::web::Data::new(AppState { repo, auth_service, auth_keys: init_auth_keys().await })
}

pub fn get_fake_fullname() -> String {
    format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>())
}

pub fn get_fake_title() -> &'static str {
    let index = get_random_no_from_range(0, FAKE_JOB_TITLES.get().unwrap().len()-1);
    FAKE_JOB_TITLES.get().unwrap()[index]
}

pub fn get_fake_desc() -> &'static str {
    let index = get_random_no_from_range(0, FAKE_JOB_DESC.get().unwrap().len()-1);
    FAKE_JOB_DESC.get().unwrap()[index]
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

pub fn get_company_logo_randomly() -> Vec<u8> {
    let file_no = get_random_no_from_range(1, 7);
    let file_path = format!("src/common_test/files/office-cl-{}.png", file_no);
    
    get_file_buffer(&file_path)
}

pub async fn get_random_salary() -> Salary {    
    let index = get_random_no_from_range(0, 3);
    SALARY_BASE.get().unwrap().get(index).unwrap().clone()
}

pub async fn get_random_language() -> Language {
    let index = get_random_no_from_range(0, 8);
    LANGUAGES.get().unwrap().get(index).unwrap().clone()
}

pub async fn get_random_industry() -> Industry {
    let index = get_random_no_from_range(0, 4);
    INDUSTRIES.get().unwrap().get(index).unwrap().clone()
}

pub async fn get_random_country() -> Country {
    COUNTRIES.get().unwrap().get(0).unwrap().clone()
}