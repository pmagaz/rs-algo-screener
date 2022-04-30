use super::instrument;
use crate::db;
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::models::backtest_instrument::BackTestInstrument;

use actix_web::{web, HttpResponse};
use bson::doc;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct ApiResponse {
    result: String,
}

pub async fn find_all(state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();

    let back_test_symbols: Vec<BackTestInstrument> = db::back_test::find_all(&state).await.unwrap();

    println!(
        "[BACK TEST LIST] {:?} {:?} {:?}",
        Local::now(),
        back_test_symbols,
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(back_test_symbols))
}

pub async fn find_instruments(state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let back_test_symbols: Vec<String> = db::back_test::find_all(&state)
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.symbol)
        .collect();

    let query = doc! {"symbol": { "$in": &back_test_symbols }};

    let instruments = instrument::find_detail(query.to_string(), state)
        .await
        .unwrap();

    println!(
        "[BACK TEST INSTRUMENTS] {:?} {:?} {:?}",
        Local::now(),
        back_test_symbols,
        now.elapsed()
    );

    Ok(instruments)
}
