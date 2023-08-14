mod api;
mod models;
mod repository;
mod logger;
use actix_web::{web, App, HttpServer,http::header};
use actix_cors::Cors;
use crate::api::routes::*;
use fern;
use chrono::Utc;
use actix_web::middleware::Logger;
use env_logger::Env;



//log build


struct AppState {
    app_name: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // setup_logging().expect("failed to initialize logging.");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    HttpServer::new(|| {
        // let cors = Cors::default().allow_any_origin().send_wildcard();
        let cors = Cors::permissive()
            .allowed_methods(vec!["GET", "POST","OPTIONS"])
            // .allowed_headers(vec![
            //     header::CONTENT_TYPE,
            //     header::AUTHORIZATION,
            //     header::ACCEPT,
            // ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
        .wrap(cors)
        .wrap(Logger::default())
        .service(web::scope("/v1").configure(init_routes_v1))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
