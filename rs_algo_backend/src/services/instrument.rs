use crate::db;
use crate::error::CustomError;
use crate::models::app_state::AppState;
use crate::models::instrument::Instrument;
use std::time::Instant;

use actix_web::{web, HttpResponse, Responder};
use rs_algo_shared::helpers::date::Local;

pub async fn get(state: web::Data<AppState>) -> Result<HttpResponse, CustomError> {
    let now = Instant::now();
    println!("[GET] {:?} {:?}", Local::now(), now.elapsed());

    let instruments = db::instrument::find_all(&state).await.unwrap();

    println!("[GET] {:?} {:?}", Local::now(), now.elapsed());

    Ok(HttpResponse::Ok().json(instruments))
}

pub async fn post(data: String, state: web::Data<AppState>) -> Result<HttpResponse, CustomError> {
    let now = Instant::now();
    let response: Instrument = serde_json::from_str(&data).unwrap();
    let symbol = response.symbol.clone();

    let _insert_result = db::instrument::insert(response, &state).await.unwrap();

    println!(
        "[INSERTED] {:?} at {:?} in {:?}",
        symbol,
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json("ok"))
}
