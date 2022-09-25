use super::instrument;
use crate::db;
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;

use actix_web::{web, HttpResponse};
use bson::doc;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::watch_instrument::*;

use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct ApiResponse {
    result: String,
}

pub async fn find(state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let portfolio_symbols: Vec<String> = db::portfolio::find_all(&state)
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.symbol)
        .collect();

    let query = doc! {"symbol": { "$in": &portfolio_symbols }};

    let instruments = instrument::find(query.to_string(), state).await.unwrap();
    log::info!(
        "[FIND PORTFOLIO] {:?} {:?} {:?}",
        Local::now(),
        portfolio_symbols,
        now.elapsed()
    );

    Ok(instruments)
}

pub async fn upsert(
    watch_instrument: String,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    log::info!(
        "[PORTFOLIO] Received at {:?} in {:?}",
        Local::now(),
        now.elapsed()
    );

    let now = Instant::now();
    let watch_instrument: WatchInstrument = serde_json::from_str(&watch_instrument).unwrap();

    let symbol = watch_instrument.symbol.clone();

    log::info!(
        "[PORTFOLIO] Parsed {:?} at {:?} in {:?}",
        symbol,
        Local::now(),
        now.elapsed()
    );

    let now = Instant::now();
    let _upsert = db::portfolio::upsert(watch_instrument, &state)
        .await
        .unwrap();

    log::info!(
        "[PORTFOLIO UPSERTED] {:?} at {:?} in {:?}",
        symbol,
        Local::now(),
        now.elapsed()
    );
    Ok(HttpResponse::Ok().json(ApiResponse {
        result: "ok".to_owned(),
    }))
}


pub async fn delete(watch_instrument: String, state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let watch_instrument: WatchInstrument = serde_json::from_str(&watch_instrument).unwrap();
    let symbol = watch_instrument.symbol.clone();

    let _result = db::portfolio::delete(&watch_instrument, &state).await.unwrap();

    log::info!(
        "[DELETED PORTFOLIO] {:?} {:?} {:?}",
        Local::now(),
        symbol,
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(ApiResponse {
        result: "ok".to_owned(),
    }))
}