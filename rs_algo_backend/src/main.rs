use actix_web::dev::Service;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use futures::future::FutureExt;

use std::io::Result;

#[macro_use]
extern crate lazy_static;

mod error;
mod handlers;
mod models;

use handlers::index::index;
use models::app_state::AppState;
use std::env;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let port = env::var("BACKEND_PORT").unwrap();
    let app_name = env::var("BACKEND_NAME").unwrap();
    println!(
        "[Server] Launching {:} on port {:?}",
        app_name,
        port.clone()
    );
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from(&app_name),
            })
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .service(web::scope("/data").route("/", web::get().to(index)))
    })
    .bind(["0.0.0.0:", &port].concat())
    .expect("Can't launch server")
    .run()
    .await
}
