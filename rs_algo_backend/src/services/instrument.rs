use crate::backend::Backend;
use crate::db;
use crate::db::helpers::{compact_instrument, get_collection};
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::models::Instrument;
use std::time::Instant;

use actix_web::{web, HttpResponse};
use rs_algo_shared::helpers::date::Local;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct SymbolQuery {
    pub symbol: String,
}

pub async fn render(
    query: web::Query<SymbolQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();

    let instrument = db::instrument::find_by_symbol(&query.symbol, &state)
        .await
        .unwrap()
        .unwrap();
    let backend = Backend::new();
    let _output = backend.render(&instrument);

    println!(
        "[RENDER] {:?} {:?} {:?}",
        instrument.symbol,
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(instrument))
}

pub async fn find(params: String, state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let instruments = db::instrument::find_by_params(&state, params)
        .await
        .unwrap();

    println!("[FIND] {:?} {:?}", Local::now(), now.elapsed());

    Ok(HttpResponse::Ok().json(instruments))
}

pub async fn upsert(
    instrument: String,
    //instrument: web::Json<Instrument>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let mut instrument: Instrument = serde_json::from_str(&instrument).unwrap();
    let symbol = instrument.symbol.clone();
    instrument.updated = Local::now().to_string();

    let insert_instruments_detail = env::var("INSERT_INSTRUMENTS_DETAIL")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    if insert_instruments_detail {
        let now = Instant::now();

        let _insert_result = db::instrument::insert_detail(&instrument, &state)
            .await
            .unwrap();

        println!(
            "[INSTRUMENT UPSERTED] {:?} at {:?} in {:?}",
            symbol,
            Local::now(),
            now.elapsed()
        );
    }

    let insert_instruments = env::var("INSERT_INSTRUMENTS")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    if insert_instruments {
        let now = Instant::now();
        let _insert_compact =
            db::instrument::insert(compact_instrument(instrument).unwrap(), &state)
                .await
                .unwrap();

        println!(
            "[INSTRUMENT DETAIL UPSERTED] {:?} at {:?} in {:?}",
            symbol,
            Local::now(),
            now.elapsed()
        );
    }
    Ok(HttpResponse::Ok().json("ok"))
}
