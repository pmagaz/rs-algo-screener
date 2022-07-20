use crate::db;
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::render_image::Backend;

use actix_files as fs;
use actix_web::{web, HttpResponse};
use bson::doc;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::BackTestStrategyResult;
use rs_algo_shared::models::instrument::Instrument;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct ApiResponse {
    result: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SymbolQuery {
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Params {
    pub offset: u64,
    pub limit: i64,
}

pub async fn find_instruments(
    market: web::Path<String>,
    query: web::Query<Params>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let env = env::var("ENV").unwrap();

    let market = market.into_inner();

    let offset = query.offset;
    let limit = query.limit;

    // let market = match market.as_ref() {
    //     "crytpo"
    // }

    let query = match env.as_ref() {
        "development" => doc! {"market": &market, "symbol": "HOLX.US"},
        _ => doc! { "market": &market},
    };

    println!("[BACK TEST INSTRUMENTS] Request for {:?}", market);

    let backtest_instruments: Vec<Instrument> =
        db::back_test::find_instruments(query, offset, limit, &state)
            .await
            .unwrap();

    println!(
        "[BACK TEST INSTRUMENTS] {:?} instruments returned at {:?} {:?}",
        backtest_instruments.len(),
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(backtest_instruments))
}

pub async fn find_instruments_result(
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();

    println!("[BACK TEST STRATEGIES] Request at {:?}", Local::now());
    let query = doc! {};
    let backtest_instruments_result: Vec<BackTestInstrumentResult> =
        db::back_test::find_backtest_instruments_result(query, 50, &state)
            .await
            .unwrap();

    println!(
        "[BACK TEST INSTRUMENTS] {:?} {:?}",
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(backtest_instruments_result))
}

pub async fn find_instruments_result_by_strategy(
    params: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();

    let (market, strategy) = params.into_inner();

    println!(
        "[BACK TEST STRATEGIES] For {} Request at {:?}",
        market,
        Local::now()
    );

    let query = doc! {"market": market, "strategy": strategy.to_string()};

    let backtest_instruments_result: Vec<BackTestInstrumentResult> =
        db::back_test::find_backtest_instruments_result(query, 500, &state)
            .await
            .unwrap();

    println!(
        "[BACK TEST INSTRUMENTS] {:?} {:?}",
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(backtest_instruments_result))
}

//FIXME IMPROVE ERROR HANDLING
pub async fn upsert_instruments_result(
    backtested_result: web::Json<BackTestInstrumentResult>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();

    let symbol = backtested_result.instrument.symbol.clone();

    println!(
        "[BACKTEST INSTRUMENT] instrument {} received at {:?} in {:?}",
        &symbol,
        Local::now(),
        now
    );

    let now = Instant::now();
    let _upsert = db::back_test::upsert_instruments_result(&backtested_result, &state)
        .await
        .unwrap();

    println!(
        "[BACKTEST RESULT UPSERTED] instrument {} at {:?} in {:?}",
        symbol,
        Local::now(),
        now.elapsed()
    );
    Ok(HttpResponse::Ok().json(backtested_result))
}

pub async fn upsert_strategies_result(
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
    let _upsert = db::back_test::upsert_strategies_result(&backtested_strategy_result, &state)
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

pub async fn find_strategies_result(
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();

    println!("[BACK TEST STRATEGIES] Request at {:?}", Local::now());

    let query = doc! {};

    let backtest_instruments: Vec<BackTestStrategyResult> =
        db::back_test::find_strategies_result(query, &state)
            .await
            .unwrap();

    println!(
        "[BACK TEST STRATEGIES] {:?} {:?}",
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(backtest_instruments))
}

pub async fn chart(
    params: web::Path<(String, String)>,
    query: web::Query<SymbolQuery>,
    state: web::Data<AppState>,
) -> Result<fs::NamedFile, RsAlgoError> {
    let now = Instant::now();
    let symbol = &query.symbol;
    let (market, strategy) = params.into_inner();

    println!(
        "[BACKTEST CHART] for {:?} / {:?} at {:?}",
        strategy,
        symbol,
        Local::now(),
    );

    let query =
        doc! {"market": market, "strategy": strategy.to_string(),  "instrument.symbol": symbol};

    let backtest_result: BackTestInstrumentResult =
        db::back_test::find_strategy_instrument_result(query, &state)
            .await
            .unwrap()
            .unwrap();

    let trades_in: Vec<TradeIn> = backtest_result.instrument.trades_in;
    let trades_out: Vec<TradeOut> = backtest_result.instrument.trades_out;
    let trades = &(&trades_in, &trades_out);

    let instrument = db::back_test::find_backtest_instrument_by_symbol(&*symbol, &state)
        .await
        .unwrap()
        .unwrap();

    let output_file = [
        &env::var("BACKEND_PLOTTER_OUTPUT_FOLDER").unwrap(),
        &strategy,
        "_",
        symbol,
        ".png",
    ]
    .concat();

    Backend::new().render(&instrument, trades, &output_file);

    let mut image_path = PathBuf::new();
    image_path.push(output_file);

    let file = fs::NamedFile::open(image_path).unwrap();

    println!(
        "[BACKTEST CHART RENDER] {:?} {:?} {:?}",
        strategy,
        Local::now(),
        now.elapsed()
    );

    Ok(file.use_last_modified(true))
}
