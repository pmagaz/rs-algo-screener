use super::instrument;
use crate::db;
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::models::backtest_instrument::BackTestInstrumentResult;
use crate::models::backtest_strategy::BackTestStrategyResult;
use crate::models::instrument::Instrument;

use actix_web::{web, HttpResponse};
use bson::doc;
use rs_algo_shared::helpers::date::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Instant;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct ApiResponse {
    result: String,
}

pub async fn find_instruments(state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();

    println!("[BACK TEST INSTRUMENTS] Request at {:?}", Local::now());

    let growth_srt = env::var("GROWTH_STOCK_SYMBOLS").unwrap();
    let growth_symbols: Vec<&str> = growth_srt.split(",").collect();

    let value_srt = env::var("VALUE_STOCK_SYMBOLS").unwrap();
    let value_symbols: Vec<&str> = value_srt.split(",").collect();
    let arr = [growth_symbols, value_symbols].concat();
    let query = doc! {"symbol": { "$in": arr }};

    let backtest_instruments: Vec<Instrument> = db::back_test::find_instruments(query, &state)
        .await
        .unwrap();

    println!(
        "[BACK TEST INSTRUMENTS] {:?} {:?}",
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(backtest_instruments))
}

pub async fn find_strategies(state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();

    println!("[BACK TEST STRATEGIES] Request at {:?}", Local::now());
    let query = doc! {};
    let backtest_instruments_result: Vec<BackTestInstrumentResult> =
        db::back_test::find_backtest_instruments_result(query, &state)
            .await
            .unwrap();

    println!(
        "[BACK TEST INSTRUMENTS] {:?} {:?}",
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(backtest_instruments_result))
}

pub async fn upsert(
    //backtested_result: String,
    backtested_result: web::Json<BackTestInstrumentResult>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    println!(
        "[BACKTEST INSTRUMENT] Received at {:?} in {:?}",
        Local::now(),
        now
    );

    let now = Instant::now();
    // let backtested_result: BackTestInstrumentResult =
    //     serde_json::from_str(&backtested_result).unwrap();

    let symbol = backtested_result.instrument.symbol.clone();

    let now = Instant::now();
    let _upsert = db::back_test::upsert(&backtested_result, &state)
        .await
        .unwrap();

    println!(
        "[BACKTEST RESULT UPSERTED] {:?} at {:?} in {:?}",
        symbol,
        Local::now(),
        now.elapsed()
    );
    Ok(HttpResponse::Ok().json(backtested_result))
}

pub async fn upsert_strategies(
    backtested_strategy_result: web::Json<BackTestStrategyResult>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    println!(
        "[BACKTEST INSTRUMENT] Received at {:?} in {:?}",
        Local::now(),
        now
    );

    let now = Instant::now();
    // let backtested_result: BackTestInstrumentResult =
    //     serde_json::from_str(&backtested_result).unwrap();

    //let symbol = backtested_strategy_result.instrument.symbol.clone();

    let now = Instant::now();
    let _upsert = db::back_test::upsert_strategies(&backtested_strategy_result, &state)
        .await
        .unwrap();

    println!(
        "[BACKTEST STRATEGY UPSERTED] {:?} at {:?} in {:?}",
        backtested_strategy_result.strategy,
        Local::now(),
        now.elapsed()
    );
    Ok(HttpResponse::Ok().json(backtested_strategy_result))
}
