use fern;
use actix_web::web;
use chrono::Utc;
use log::info;
use crate::models::request_models::*;

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