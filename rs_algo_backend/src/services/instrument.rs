use crate::db;
use crate::db::helpers::{compact_instrument, get_collection};
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::models::instrument::Instrument;
use std::time::Instant;

use actix_web::{web, HttpResponse};
use rs_algo_shared::helpers::date::Local;

pub async fn post(params: String, state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let compact_instruments = db::instrument::find_by_params(&state, params)
        .await
        .unwrap();

    println!("[POST] {:?} {:?}", Local::now(), now.elapsed());

    Ok(HttpResponse::Ok().json(compact_instruments))
}

pub async fn put(
    instrument: String,
    //instrument: web::Json<Instrument>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let mut instrument: Instrument = serde_json::from_str(&instrument).unwrap();
    let now = Instant::now();
    let symbol = instrument.symbol.clone();
    instrument.updated = Local::now().to_string();

    let _insert_result = db::instrument::insert(&instrument, &state).await.unwrap();

    println!(
        "[INSTRUMENT INSERTED] {:?} at {:?} in {:?}",
        symbol,
        Local::now(),
        now.elapsed()
    );

    let _insert_compact =
        db::instrument::insert_compact(compact_instrument(instrument).unwrap(), &state)
            .await
            .unwrap();

    println!(
        "[COMPACT INSTRUMENT INSERTED] {:?} at {:?} in {:?}",
        symbol,
        Local::now(),
        now.elapsed()
    );
    let now = Instant::now();

    Ok(HttpResponse::Ok().json("ok"))
}
