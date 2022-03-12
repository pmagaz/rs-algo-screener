use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;

use std::io::Result;

mod db;
mod error;
mod helpers;
mod models;
mod services;

use db::mongo;
use error::RsAlgoError;
use models::app_state::AppState;
use services::index::index;
use services::instrument;
use std::env;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let port = env::var("BACKEND_PORT").expect("BACKEND_PORT not found");
    let app_name = env::var("BACKEND_NAME").expect("BACKEND_NAME not found");
    let db_name = env::var("BACKEND_DATABASE").expect("BACKEND_DATABASE not found");

    let mongodb: mongodb::Client = mongo::connect()
        .await
        .map_err(|_e| RsAlgoError::NoDbConnection)
        .unwrap();

    println!(
        "[Server] Launching {:} on port {:?}",
        app_name,
        port.clone()
    );

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from(&app_name),
                db: mongodb.clone(),
                db_name: db_name.to_owned(),
            })
            .app_data(web::PayloadConfig::new(10000000))
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .service(
                web::scope("/api")
                    .route("/instruments", web::get().to(instrument::get))
                    .route("/instruments", web::post().to(instrument::post)),
            )
    })
    .bind(["0.0.0.0:", &port].concat())
    .expect("[Error] Can't launch server!")
    .run()
    .await
}
