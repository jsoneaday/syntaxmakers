pub mod common {
    pub mod datetime_utils;
    pub mod repository {
        pub mod base;
        pub mod error;
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
        pub mod auth_service;
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
}

use actix_cors::Cors;
use actix_web::{HttpServer, http::header, App, middleware::Logger, web};
use common::authentication::auth_service::{init_auth_keys, AuthService};
use common::repository::base::{DbRepo, Repository};
use routes::authentication::routes::{login, refresh_access_token};
use routes::developers::routes::get_developer_by_email;
use routes::{
    salaries::routes::get_all_salaries, 
    languages::routes::get_all_languages, 
    jobs::routes::{get_job, create_job, get_jobs_by_dev_profile}, 
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
                        .route(web::post().to(login::<DbRepo, AuthService>)))
                    .service(web::resource("/refreshtoken")
                        .route(web::post().to(refresh_access_token::<DbRepo, AuthService>)))
                    .service(web::resource("/salaries")
                        .route(web::get().to(get_all_salaries::<DbRepo, AuthService>)))
                    .service(web::resource("/languages")
                        .route(web::get().to(get_all_languages::<DbRepo, AuthService>)))
                    .service(web::resource("/job/{id}")
                        .route(web::get().to(get_job::<DbRepo, AuthService>)))
                    .service(web::resource("/job")
                        .route(web::post().to(create_job::<DbRepo, AuthService>)))
                    .service(web::resource("/jobs")
                        .route(web::post().to(get_jobs_by_dev_profile::<DbRepo, AuthService>)))
                    .service(web::resource("/industries")
                        .route(web::get().to(get_all_industries::<DbRepo, AuthService>)))
                    .service(web::resource("/employer/{id}")
                        .route(web::get().to(get_employer::<DbRepo, AuthService>)))
                    .service(web::resource("/employer")
                        .route(web::post().to(create_employer::<DbRepo, AuthService>)))
                    .service(web::resource("/employers")
                        .route(web::get().to(get_all_employers::<DbRepo, AuthService>)))
                    .service(web::resource("/developer_email/{email}")
                        .route(web::get().to(get_developer_by_email::<DbRepo, AuthService>)))
                    .service(web::resource("/developer/{id}")
                        .route(web::get().to(get_developer::<DbRepo, AuthService>)))
                    .service(web::resource("/developer")
                        .route(web::post().to(create_developer::<DbRepo, AuthService>)))
                    .service(web::resource("/developers")
                        .route(web::get().to(get_all_developers::<DbRepo, AuthService>)))
                    .service(web::resource("/countries")
                        .route(web::get().to(get_all_countries::<DbRepo, AuthService>)))
                    .service(web::resource("/company")
                        .route(web::post().to(create_company::<DbRepo, AuthService>)))
                    .service(web::resource("/companies")
                        .route(web::get().to(get_all_companies::<DbRepo, AuthService>)))
            )            
    })
    .bind((host, port)).expect("")
    .run()
    .await
}