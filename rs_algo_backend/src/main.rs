use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;

use std::io::Result;

mod db;
mod error;
mod models;
mod render_image;
mod services;
mod strategies;

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
        "[Server] Launching {:} on port2 {:?}",
        app_name,
        port.clone()
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .data(AppState {
                app_name: String::from(&app_name),
                db: mongodb.clone(),
                db_name: db_name.to_owned(),
            })
            .app_data(web::PayloadConfig::new(10000000))
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/instruments", web::get().to(instrument::render))
            .route("/instruments", web::post().to(instrument::find))
            .route("/instruments", web::put().to(instrument::upsert))
    })
    .bind(["0.0.0.0:", &port].concat())
    .expect("[Error] Can't launch server!")
    .run()
    .await
}
