mod api;
mod models;
mod repository;
mod logger;
use log::{debug, error, info, trace, warn};
use log4rs;
use actix_web::{web, App, HttpServer,http::header};
use actix_cors::Cors;
use crate::api::endpoints::*;

struct AppState {
    app_name: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().send_wildcard();
        // let cors = Cors::permissive()
        //     .allowed_methods(vec!["GET", "POST","OPTIONS"])
        //     .allowed_headers(vec![
        //         header::CONTENT_TYPE,
        //         header::AUTHORIZATION,
        //         header::ACCEPT,
        //     ])
        //     .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
        .wrap(cors)
        .service(player_creation_handler)
        .service(player_login_handler)
        .service(get_balance_handler)
        .service(available_games_handler)
        .service(payment_init_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
