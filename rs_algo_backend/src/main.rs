use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;

use std::io::Result;

mod db;
mod error;
mod middleware;
mod models;
mod render_image;
mod services;
mod strategies;

use db::mongo;
use error::RsAlgoError;
use middleware::cors::cors_middleware;
use models::app_state::AppState;
use models::db::Db;
use services::back_test;
use services::index::index;
use services::instrument;
use services::watch_list;
use std::env;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let port = env::var("BACKEND_PORT").expect("BACKEND_PORT not found");
    let app_name = env::var("BACKEND_NAME").expect("BACKEND_NAME not found");
    //let db_name = env::var("MONGO_MEM_DB_NAME").expect("MONGO_MEM_DB_NAME not found");

    let username = env::var("DB_USERNAME").expect("DB_USERNAME not found");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD not found");
    let db_mem_name = env::var("MONGO_MEM_DB_NAME").expect("MONGO_MEM_DB_NAME not found");
    let db_mem_uri = env::var("MONGO_MEM_DB_URI").expect("MONGO_MEM_DB_URI not found");

    let db_hdd_name = env::var("MONGO_HDD_DB_NAME").expect("MONGO_HD_DB_NAME not found");
    let db_hdd_uri = env::var("MONGO_HDD_DB_URI").expect("MONGO_HD_DB_URI not found");

    let mongodb_mem_client: mongodb::Client =
        mongo::connect(&username, &password, &db_mem_name, &db_mem_uri)
            .await
            .map_err(|_e| RsAlgoError::NoDbConnection)
            .unwrap();

    let mongodb_hdd_client: mongodb::Client =
        mongo::connect(&username, &password, &db_hdd_name, &db_hdd_uri)
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
            .wrap(cors_middleware())
            //.wrap(Cors::permissive())
            .data(AppState {
                app_name: String::from(&app_name),
                db_mem: Db {
                    client: mongodb_mem_client.clone(),
                    name: db_mem_name.to_owned(),
                },
                db_hdd: Db {
                    client: mongodb_hdd_client.clone(),
                    name: db_hdd_name.to_owned(),
                },
            })
            .app_data(web::PayloadConfig::new(10000000))
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .service(
                web::scope("/api")
                    .route("/instruments", web::post().to(instrument::find))
                    .route("/instruments", web::put().to(instrument::upsert))
                    .route("/instruments/{symbol}", web::get().to(instrument::find_one))
                    .route(
                        "/instruments/chart/{symbol}",
                        web::get().to(instrument::chart),
                    )
                    .route("/watchlist", web::get().to(watch_list::find))
                    .route("/watchlist", web::put().to(watch_list::upsert))
                    .route("/backtest", web::put().to(back_test::upsert))
                    .route("/backtest/instruments", web::get().to(back_test::find_all)),
            )
    })
    .bind(["0.0.0.0:", &port].concat())
    .expect("[Error] Can't launch server!")
    .run()
    .await
}
