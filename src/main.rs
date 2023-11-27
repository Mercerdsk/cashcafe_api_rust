mod api;
mod models;
mod repository;
mod logger;
use actix_web::{web, App, HttpServer,http::header,web::Data};
use actix_cors::Cors;
use crate::api::routes::*;
use crate::models::request_models::GlobalConfigModel;
use crate::api::endpoints::get_version_handler;
use fern;
use chrono::Utc;
use actix_web::middleware::Logger;
use env_logger::Env;
use std::path::Path;
use std::fs::File;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};



//log build
fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {

    let json_file_path= Path::new("./json_files/database_config.json");
    let file = File::open(json_file_path)?;
    let games:GlobalConfigModel=serde_json::from_reader(file)?;
    let path = Path::new(&games.log_file_path);

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
        .level_for("tiberius", log::LevelFilter::Off)
        .level_for("actix_web", log::LevelFilter::Off)
        .chain(fern::DateBased::new(path, "%Y-%m-%d-hour-%H-api.log"))
        .apply()?;

    Ok(())
}

#[derive(Debug)]
struct AppState {
    app_name: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("-------------------Starting Actix-Web Server-----------------");
    println!("-------------------Reading config file --------------------");
    let json_file_path= Path::new("./json_files/database_config.json");
    let file = File::open(json_file_path)?;
    let web_config:GlobalConfigModel=serde_json::from_reader(file)?;
    let toggle_log = web_config.toggle_log;
    let api_port = web_config.api_port;

    if toggle_log==0{
        setup_logging().expect("failed to initialize logging.");
        println!("file logging activated");
    }
    else {
        env_logger::init_from_env(Env::default().default_filter_or("debug"));
        println!("console logging activated");
    }
    //  let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("/home/sasikumar/Analytics_RND/rust_files/cashcafe_api/src/certicficates/key.key", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("/home/sasikumar/Analytics_RND/rust_files/cashcafe_api/src/certicficates/cert.crt").unwrap();
    HttpServer::new(move|| {
        // let cors = Cors::default().allow_any_origin().send_wildcard();
        let json_cfg = web::JsonConfig::default().limit(10_097_152);
        let cors = Cors::permissive()
            .allowed_methods(vec!["GET", "POST","OPTIONS"])
            // .allowed_headers(vec![
            //     header::CONTENT_TYPE,
            //     header::AUTHORIZATION,
            //     header::ACCEPT,
            // ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(web_config.clone()))
            
        .wrap(cors)
        .wrap(Logger::default())
        .service(get_version_handler)
        .service(web::scope("/v1").configure(init_routes_v1).app_data(json_cfg))
    })
    .bind(("0.0.0.0", api_port))?
    .run()
    .await
}
