pub mod common {
    pub mod datetime_utils;
    pub mod fs_utils;
    pub mod rand_utils;
    pub mod repository {
        pub mod base;
        pub mod error;
        pub mod application {
            pub mod models;
            pub mod repo;
        }
        pub mod countries {
            pub mod models;
            pub mod repo;
        }
        pub mod industries {
            pub mod models;
            pub mod repo;
        }
        pub mod languages {
            pub mod models;
            pub mod repo;
        }
        pub mod salaries {
            pub mod models;
            pub mod repo;
        }
        pub mod companies {
            pub mod models;
            pub mod repo;
        }
        pub mod developers {
            pub mod models;
            pub mod repo;
        }
        pub mod employers {
            pub mod models;
            pub mod repo;
        }
        pub mod jobs {
            pub mod models;
            pub mod repo;
        }
        pub mod user {
            pub mod models;
            pub mod repo;
        }        
    }
    pub mod authentication {
        pub mod password_hash;
        pub mod auth_keys_service;
    }
    pub mod emailer {
        pub mod model;
        pub mod emailer;
    }
}
pub mod common_test {
    pub mod fixtures;
}
pub mod app_state;
pub mod routes {
    pub mod base_model;
    pub mod user_error;
    pub mod route_utils;
    pub mod auth_helper;
    pub mod application {
        pub mod models;
        pub mod routes;
    }
    pub mod authentication {
        pub mod models;
        pub mod routes;
    }
    pub mod companies {
        pub mod models;
        pub mod routes;
    }
    pub mod countries {
        pub mod models;
        pub mod routes;
    }
    pub mod developers {
        pub mod models;
        pub mod routes;
    }
    pub mod employers {
        pub mod models;
        pub mod routes;
    }
    pub mod industries {
        pub mod models;
        pub mod routes;
    }
    pub mod jobs {
        pub mod models;
        pub mod routes;
    }
    pub mod languages {
        pub mod models;
        pub mod routes;
    }
    pub mod salaries {
        pub mod models;
        pub mod routes;
    }
    pub mod user {
        pub mod models;
        pub mod routes;
    }
}

use actix_cors::Cors;
use actix_web::{HttpServer, http::header, App, middleware::Logger, web};
use common::authentication::auth_keys_service::{init_auth_keys, AuthService};
use common::emailer::emailer::Emailer;
use common::repository::base::{DbRepo, Repository};
use routes::application::routes::{create_application, developer_applied};
use routes::authentication::routes::{login, refresh_access_token};
use routes::developers::routes::{get_developer_by_email, update_developer};
use routes::employers::routes::{get_employer_by_email, update_employer};
use routes::user::routes::{change_password, confirm_email, send_email};
use routes::jobs::routes::{get_jobs_and_appliers, get_jobs_by_applier, get_jobs_by_employer, get_jobs_by_search_terms, get_jobs_by_search_terms_for_emp, update_job};
use routes::{
    salaries::routes::get_all_salaries, 
    languages::routes::get_all_languages, 
    jobs::routes::{get_job, create_job, get_jobs_by_developer}, 
    industries::routes::get_all_industries, 
    employers::routes::{get_employer, create_employer, get_all_employers}, 
    developers::routes::{get_developer, create_developer, get_all_developers}, 
    countries::routes::get_all_countries, 
    companies::routes::{get_all_companies, create_company}
};
use crate::app_state::AppState;
use std::env;
use dotenv::dotenv;
use openssl::ssl::{SslAcceptorBuilder, SslAcceptor, SslMethod, SslFiletype};

#[allow(unused)]
fn ssl_builder() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("ssl/key.pem", SslFiletype::PEM)
        .expect("failed to open/read key.pem");
    builder.set_certificate_chain_file("ssl/cert.pem")
        .expect("failed to open/read cert.pem");
    builder
}

pub async fn run() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    
    dotenv().ok();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    
    let app_data = actix_web::web::Data::new(AppState{
        repo: DbRepo::init().await,
        emailer: Emailer::new(),
        auth_service: AuthService,
        auth_keys: init_auth_keys().await
    });    

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())     
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        header::CONTENT_TYPE,
                        header::AUTHORIZATION,
                        header::ACCEPT,
                    ])
                    .supports_credentials()
                    .max_age(3600)
            )                              
            .service(
                web::scope("/v1")
                    .service(web::resource("/login")
                        .route(web::post().to(login::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/apply_job")
                        .route(web::post().to(create_application::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/developer_applied")
                        .route(web::post().to(developer_applied::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/refreshtoken")
                        .route(web::post().to(refresh_access_token::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/salaries")
                        .route(web::get().to(get_all_salaries::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/languages")
                        .route(web::get().to(get_all_languages::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/job/{id}")
                        .route(web::get().to(get_job::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/job_update")
                        .route(web::post().to(update_job::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/job")
                        .route(web::post().to(create_job::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/jobs_dev")
                        .route(web::post().to(get_jobs_by_developer::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/jobs_emp")
                        .route(web::post().to(get_jobs_by_employer::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/jobs_search")
                        .route(web::post().to(get_jobs_by_search_terms::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/jobs_emp_search")
                        .route(web::post().to(get_jobs_by_search_terms_for_emp::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/jobs_applied")
                        .route(web::post().to(get_jobs_by_applier::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/job_applicants")
                        .route(web::post().to(get_jobs_and_appliers::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/industries")
                        .route(web::get().to(get_all_industries::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/employer/{id}")
                        .route(web::get().to(get_employer::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/employer")
                        .route(web::post().to(create_employer::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/employer_update")
                        .route(web::post().to(update_employer::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/employer_email/{email}")
                        .route(web::get().to(get_employer_by_email::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/employers")
                        .route(web::get().to(get_all_employers::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/developer_email/{email}")
                        .route(web::get().to(get_developer_by_email::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/developer/{id}")
                        .route(web::get().to(get_developer::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/developer")
                        .route(web::post().to(create_developer::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/developer_update")
                        .route(web::post().to(update_developer::<DbRepo, Emailer, AuthService>)))                    
                    .service(web::resource("/developers")
                        .route(web::get().to(get_all_developers::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/user_change_password")
                        .route(web::post().to(change_password::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/countries")
                        .route(web::get().to(get_all_countries::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/company")
                        .route(web::post().to(create_company::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/companies")
                        .route(web::get().to(get_all_companies::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/confirm_email")
                        .route(web::get().to(confirm_email::<DbRepo, Emailer, AuthService>)))
                    .service(web::resource("/send_email")
                        .route(web::post().to(send_email::<DbRepo, Emailer, AuthService>)))
            )            
    })
    .bind((host, port)).expect("")
    // note: cannot use this for dev as client must also be on https,
    // enable at production
    // .bind_openssl((host, port), ssl_builder()).expect("SSL not working") 
    .run()
    .await
}