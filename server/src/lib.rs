pub mod common {
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
    }
}
pub mod common_test {
    pub mod fixtures;
}
pub mod app_state;
pub mod routes {
    pub mod base_model;
    pub mod user_error;
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

use actix_web::{HttpServer, App, middleware::Logger, web};
use common::repository::base::{DbRepo, Repository};
use routes::{salaries::routes::get_all_salaries, languages::routes::get_all_languages, jobs::routes::{get_job, create_job}, industries::routes::get_all_industries, employers::routes::{get_employer, create_employer, get_all_employers}, developers::routes::{get_developer, create_developer, get_all_developers}, countries::routes::get_all_countries, companies::routes::{get_all_companies, create_company}};
use crate::app_state::AppState;
use std::env;
use dotenv::dotenv;

pub async fn run() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    
    dotenv().ok();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    let repo = DbRepo::init().await;
    let app_data = actix_web::web::Data::new(AppState{
        repo
    });    

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .service(
                web::scope("/v1")
                    .service(web::resource("/salaries")
                        .route(web::get().to(get_all_salaries::<DbRepo>)))
                    .service(web::resource("/languages")
                        .route(web::get().to(get_all_languages::<DbRepo>)))
                    .service(web::resource("/job/{id}")
                        .route(web::get().to(get_job::<DbRepo>)))
                    .service(web::resource("/job")
                        .route(web::post().to(create_job::<DbRepo>)))
                    .service(web::resource("/industries")
                        .route(web::get().to(get_all_industries::<DbRepo>)))
                    .service(web::resource("/employer/{id}")
                        .route(web::get().to(get_employer::<DbRepo>)))
                    .service(web::resource("/employer")
                        .route(web::post().to(create_employer::<DbRepo>)))
                    .service(web::resource("/employers")
                        .route(web::get().to(get_all_employers::<DbRepo>)))
                    .service(web::resource("/developer/{id}")
                        .route(web::get().to(get_developer::<DbRepo>)))
                    .service(web::resource("/developer")
                        .route(web::post().to(create_developer::<DbRepo>)))
                    .service(web::resource("/developers")
                        .route(web::get().to(get_all_developers::<DbRepo>)))
                    .service(web::resource("/countries")
                        .route(web::get().to(get_all_countries::<DbRepo>)))
                    .service(web::resource("/company")
                        .route(web::post().to(create_company::<DbRepo>)))
                    .service(web::resource("/companies")
                        .route(web::get().to(get_all_companies::<DbRepo>)))
            )
    })
    .bind((host, port))?
    .run()
    .await
}