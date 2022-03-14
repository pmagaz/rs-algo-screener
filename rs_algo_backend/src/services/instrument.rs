use crate::db;
use crate::db::helpers::{compact_instrument, get_collection};
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::models::instrument::Instrument;
use std::time::Instant;

use actix_web::{web, HttpResponse};
use rs_algo_shared::helpers::date::Local;
use std::env;

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
    let symbol = instrument.symbol.clone();
    instrument.updated = Local::now().to_string();

    let insert_instruments = env::var("INSERT_INSTRUMENTS")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    if insert_instruments {
        let now = Instant::now();

        let _insert_result = db::instrument::insert(&instrument, &state).await.unwrap();

        println!(
            "[INSTRUMENT INSERTED] {:?} at {:?} in {:?}",
            symbol,
            Local::now(),
            now.elapsed()
        );
    }

    let insert_compact_instruments = env::var("INSERT_COMPACT_INSTRUMENTS")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    if insert_compact_instruments {
        let now = Instant::now();
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
    }
    Ok(HttpResponse::Ok().json("ok"))
}
