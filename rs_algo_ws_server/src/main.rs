use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpServer, Responder};

mod services;
mod message;
mod server;
mod session;

use services::ws::chat_ws;

use std::{env, sync::{Arc, atomic::{AtomicUsize, Ordering}}};
use actix::Actor;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        // env_logger::Builder::from_default_env()
    // .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
    // .filter(None, LevelFilter::Info)
    // .init();
    
    let port = env::var("WS_SERVER_PORT").expect("WS_SERVER_PORT not found");
    let app_name = env::var("WS_SERVER_NAME").expect("WS_SERVER_NAME not found");
    let app_state = Arc::new(AtomicUsize::new(0));

    let server = server::ChatServer::new(app_state.clone()).start();

    log::info!("Starting {} on port {} !", app_name, port.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .service(web::resource("/").to(chat_ws))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(["0.0.0.0:", &port].concat())
    .expect("[WS SERVER ERROR] Can't launch server!")
    .run()
    .await
}