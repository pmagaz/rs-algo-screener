use crate::db;
use crate::db::helpers::{compact_instrument, get_collection};
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::models::instrument::Instrument;
use std::time::Instant;

use actix_web::{web, HttpResponse};
use rs_algo_shared::helpers::date::Local;

pub async fn get(state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    println!("[GET] {:?} {:?}", Local::now(), now.elapsed());

    let compact_instruments = db::instrument::find_all(&state).await.unwrap();

    println!("[GET] {:?} {:?}", Local::now(), now.elapsed());

    Ok(HttpResponse::Ok().json(compact_instruments))
}

pub async fn post(data: String, state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let instrument: Instrument = serde_json::from_str(&data).unwrap();
    let symbol = instrument.symbol.clone();

    let now = Instant::now();
    let _insert_compact =
        db::instrument::insert_compact(compact_instrument(instrument.clone()).unwrap(), &state)
            .await
            .unwrap();

    println!(
        "[INSERTED COMPACT] {:?} at {:?} in {:?}",
        symbol,
        Local::now(),
        now.elapsed()
    );

    let now = Instant::now();

    let _insert_result = db::instrument::insert(instrument, &state).await.unwrap();

    println!(
        "[INSERTED] {:?} at {:?} in {:?}",
        symbol,
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json("ok"))
}
