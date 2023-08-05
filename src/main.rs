mod api;
mod models;
mod repository;
mod logger;
use actix_web::{web, App, HttpServer,http::header};
use actix_cors::Cors;
use crate::api::routes::*;
use fern;
use chrono::Utc;



//log build
fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{:?} {} {}] {}",
                Utc::now(),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("hyper", log::LevelFilter::Info)
        .chain(fern::DateBased::new("/home/sasikumar/Analytics_RND/rust_files/cashcafe_api/src/log/", "%Y-%m-%d--api.log"))
        .apply()?;

    Ok(())
}

struct AppState {
    app_name: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // setup_logging().expect("failed to initialize logging.");
    
    HttpServer::new(|| {
        // let cors = Cors::default().allow_any_origin().send_wildcard();
        let cors = Cors::permissive()
            .allowed_methods(vec!["GET", "POST","OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
        .wrap(cors)
        .service(web::scope("/v1").configure(init_routes_v1))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
