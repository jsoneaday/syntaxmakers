pub mod common {
    pub mod repository {
        pub mod base;
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

use actix_web::{HttpServer, App, middleware::Logger};
use common::repository::base::DbRepo;
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
            
    })
    .bind((host, port))?
    .run()
    .await
}