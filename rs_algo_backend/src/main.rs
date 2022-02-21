use actix_web::dev::Service;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;

use std::io::Result;

#[macro_use]
extern crate lazy_static;

mod db;
mod error;
mod handlers;
mod helpers;
mod models;

use db::mongo;
use error::CustomError;
use handlers::index::index;
use handlers::instrument::instrument;
use models::app_state::AppState;
use std::env;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let port = env::var("BACKEND_PORT").expect("BACKEND_PORT not found");
    let app_name = env::var("BACKEND_NAME").expect("BACKEND_NAME not found");
    let db_name = env::var("BACKEND_DATABASE").expect("BACKEND_DATABASE not found");

    let mongodb: mongodb::Client = mongo::connect()
        .await
        .map_err(|_e| CustomError::NoDbConnection)
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
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .service(web::scope("/api").route("/instruments", web::post().to(instrument)))
    })
    .bind(["0.0.0.0:", &port].concat())
    .expect("Can't launch server")
    .run()
    .await
}
