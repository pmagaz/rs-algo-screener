use crate::db;
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::render_image::{Backend, BackendMode};

use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::BackTestStrategyResult;
use rs_algo_shared::models::trade::{TradeIn, TradeOut};
use rs_algo_shared::scanner::instrument::*;

use actix_files as fs;
use actix_web::{web, HttpResponse};
use bson::doc;
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

pub async fn find_one(
    path: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let (symbol, time_frame) = path.into_inner();

    let instrument = db::back_test::find_one(&symbol, &time_frame, &state)
        .await
        .unwrap();

    log::info!(
        "[FINDONE] {} {} {} {:?}",
        &symbol,
        &time_frame,
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(instrument))
}

pub async fn find_instruments(
    path: web::Path<(String, String)>,
    query: web::Query<Params>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let env = env::var("ENV").unwrap();
    let (market, time_frame) = path.into_inner();

    let offset = query.offset;
    let limit = query.limit;

    let query = match env.as_ref() {
        "development" => doc! {"market": &market, "symbol": "BITCOIN"},
        _ => doc! { "market": &market},
    };

    log::info!(
        "[BACK TEST INSTRUMENTS] Request {:} for {}",
        time_frame,
        market,
    );

    let backtest_instruments: Vec<Instrument> =
        db::back_test::find_instruments(query, offset, limit, time_frame, &state)
            .await
            .unwrap();

    log::info!(
        "[BACK TEST INSTRUMENTS] {:?} instruments returned at {:?} {:?}",
        backtest_instruments.len(),
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(backtest_instruments))
}

pub async fn find_compact_instruments(
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let _env = env::var("ENV").unwrap();

    let query = doc! {};

    log::info!("[BACK TEST INSTRUMENTS] All");

    let backtest_instruments: Vec<CompactInstrument> =
        db::back_test::find_backtest_compact_instruments(query, 0, 5000, &state)
            .await
            .unwrap();

    log::info!(
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

    log::info!("[BACK TEST STRATEGIES] Request at {:?}", Local::now());
    let query = doc! {};
    let backtest_instruments_result: Vec<BackTestInstrumentResult> =
        db::back_test::find_backtest_instruments_result(query, 50, &state)
            .await
            .unwrap();

    log::info!(
        "[BACK TEST INSTRUMENTS] {:?} {:?}",
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(backtest_instruments_result))
}

pub async fn find_instruments_result_by_strategy(
    params: web::Path<(String, String, String)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();

    let (market, strategy, strategy_type) = params.into_inner();

    log::info!(
        "[BACK TEST STRATEGIES] For {} Request at {:?}",
        market,
        Local::now()
    );

    let query = doc! {"market": market, "strategy": strategy, "strategy_type": strategy_type};

    let backtest_instruments_result: Vec<BackTestInstrumentResult> =
        db::back_test::find_backtest_instruments_result(query, 500, &state)
            .await
            .unwrap();

    log::info!(
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

    log::info!(
        "[BACKTEST INSTRUMENT] instrument {} received at {:?} in {:?}",
        &symbol,
        Local::now(),
        now
    );

    let now = Instant::now();
    let _upsert = db::back_test::upsert_instruments_result(&backtested_result, &state)
        .await
        .unwrap();

    log::info!(
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
    log::info!(
        "[BACKTEST INSTRUMENT] Received at {:?} in {:?}",
        Local::now(),
        now
    );

    let now = Instant::now();
    let _upsert = db::back_test::upsert_strategies_result(&backtested_strategy_result, &state)
        .await
        .unwrap();

    log::info!(
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

    log::info!("[BACK TEST STRATEGIES] Request at {:?}", Local::now());

    let query = doc! {};

    let backtest_instruments: Vec<BackTestStrategyResult> =
        db::back_test::find_strategies_result(query, &state)
            .await
            .unwrap();

    log::info!(
        "[BACK TEST STRATEGIES] {:?} {:?}",
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(backtest_instruments))
}

pub async fn find_strategies_result_instruments(
    instrument: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();

    let instrument = instrument.into_inner();

    log::info!(
        "[BACK TEST STRATEGIES INSTRUMENT] Request for {} at {:?}",
        instrument,
        Local::now()
    );

    let query = doc! { "instrument.symbol": instrument};

    let backtest_instruments: Vec<BackTestInstrumentResult> =
        db::back_test::find_backtest_instruments_result(query, 100, &state)
            .await
            .unwrap();

    log::info!(
        "[BACK TEST STRATEGIES] {:?} {:?}",
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(backtest_instruments))
}

pub async fn find_spreads(state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();

    log::info!("[BACK TEST SPREADS] Request for at {:?}", Local::now());

    let spreads: Vec<BackTestSpread> = db::back_test::find_spreads(&state).await.unwrap();

    Ok(HttpResponse::Ok().json(spreads))
}

pub async fn chart(
    params: web::Path<(String, String, String)>,
    query: web::Query<SymbolQuery>,
    state: web::Data<AppState>,
) -> Result<fs::NamedFile, RsAlgoError> {
    let now = Instant::now();
    let symbol = &query.symbol;
    let (market, strategy, strategy_type) = params.into_inner();

    log::info!(
        "[BACKTEST CHART] for {:?} / {:?} at {:?}",
        strategy,
        symbol,
        Local::now(),
    );

    let query = doc! {"market": market, "strategy": &strategy , "strategy_type": &strategy_type, "instrument.symbol": symbol};

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

    Backend::new()
        .render(
            BackendMode::BackTest,
            &instrument,
            &HigherTMInstrument::None,
            trades,
            &output_file,
        )
        .unwrap();

    let mut image_path = PathBuf::new();
    image_path.push(output_file);

    let file = fs::NamedFile::open(image_path).unwrap();

    log::info!(
        "[BACKTEST CHART RENDER] {:?} {:?} {:?}",
        strategy,
        Local::now(),
        now.elapsed()
    );

    Ok(file.use_last_modified(true))
}
