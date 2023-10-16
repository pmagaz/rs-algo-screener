use crate::db;
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::render_chart::Backend;

use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::BackTestStrategyResult;
use rs_algo_shared::models::mode::*;
use rs_algo_shared::models::order::Order;
use rs_algo_shared::models::pricing::Pricing;
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
        .expect("Error while finding one instrument");

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
        "development" => doc! {"market": &market, "symbol": "AUDCHF"},
        _ => doc! { "market": &market},
    };

    log::info!(" Request {:} for {}", time_frame, market,);

    let backtest_instruments: Vec<Instrument> =
        db::back_test::find_instruments(query, offset, limit, time_frame, &state)
            .await
            .unwrap();

    log::info!(
        " {:?} instruments returned at {:?} {:?}",
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

    log::info!(" All");

    let backtest_instruments: Vec<CompactInstrument> =
        db::back_test::find_backtest_compact_instruments(query, 0, 5000, &state)
            .await
            .unwrap();

    log::info!(
        " {:?} instruments returned at {:?} {:?}",
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

    log::info!("Request at {:?}", Local::now());
    let query = doc! {};
    let backtest_instruments_result: Vec<BackTestInstrumentResult> =
        db::back_test::find_backtest_instruments_result(query, 50, &state)
            .await
            .unwrap();

    log::info!(" {:?} {:?}", Local::now(), now.elapsed());

    Ok(HttpResponse::Ok().json(backtest_instruments_result))
}

pub async fn find_instruments_result_by_strategy(
    params: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();

    let uuid = params.into_inner();

    let strategy_result: BackTestStrategyResult =
        db::back_test::find_strategy_result(&uuid, &state)
            .await
            .unwrap()
            .unwrap();

    log::info!(
        "For {:?} Request at {:?}",
        (
            &strategy_result.market,
            &strategy_result.strategy,
            &strategy_result.strategy_type,
            &strategy_result.time_frame
        ),
        Local::now()
    );

    let query = doc! {"market": strategy_result.market.to_string(), "strategy": strategy_result.strategy, "strategy_type": strategy_result.strategy_type.to_string(), "time_frame": strategy_result.time_frame.to_string(), "higher_time_frame": strategy_result.higher_time_frame.unwrap().to_string()};

    let backtest_instruments_result: Vec<BackTestInstrumentResult> =
        db::back_test::find_backtest_instruments_result(query, 500, &state)
            .await
            .unwrap();

    log::info!(
        " {} {:?} {:?}",
        backtest_instruments_result.len(),
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

    // Log the incoming JSON data
    log::info!(
        "[BACKTEST INSTRUMENT] Received JSON data: {:?}",
        backtested_result
    );

    log::info!(
        "[BACKTEST INSTRUMENT] instrument {} received at {:?} in {:?}",
        &symbol,
        Local::now(),
        now
    );

    let now = Instant::now();
    let _upsert = db::back_test::upsert_instruments_result(&backtested_result, &state)
        .await
        .expect("Error while upserting instruments result into the database");

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

    log::info!("Request at {:?}", Local::now());

    let query = doc! {};

    let backtest_instruments: Vec<BackTestStrategyResult> =
        db::back_test::find_strategies_result(query, &state)
            .await
            .unwrap();

    log::info!("{:?} {:?}", Local::now(), now.elapsed());

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

    log::info!("{:?} {:?}", Local::now(), now.elapsed());

    Ok(HttpResponse::Ok().json(backtest_instruments))
}

pub async fn find_prices(state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let _now = Instant::now();

    log::info!("Request for at {:?}", Local::now());

    let prices: Vec<Pricing> = db::back_test::find_prices(&state).await.unwrap();

    Ok(HttpResponse::Ok().json(prices))
}

pub async fn chart(
    params: web::Path<(String, String)>,
    state: web::Data<AppState>,
) -> Result<fs::NamedFile, RsAlgoError> {
    let now = Instant::now();
    let (uuid, symbol) = params.into_inner();

    let strategy_result: BackTestStrategyResult =
        db::back_test::find_strategy_result(&uuid, &state)
            .await
            .unwrap()
            .unwrap();

    let query = match &strategy_result.higher_time_frame {
        Some(htf) => {
            doc! {"instrument.symbol": symbol.clone(), "market": strategy_result.market.to_string(), "strategy": strategy_result.strategy.clone(), "strategy_type": strategy_result.strategy_type.to_string(), "time_frame": strategy_result.time_frame.to_string(), "higher_time_frame": htf.to_string()}
        }
        None => {
            doc! {"instrument.symbol": symbol.clone(), "market": strategy_result.market.to_string(), "strategy": strategy_result.strategy.clone(), "strategy_type": strategy_result.strategy_type.to_string(), "time_frame": strategy_result.time_frame.to_string()}
        }
    };

    let backtest_result: BackTestInstrumentResult =
        db::back_test::find_strategy_instrument_result(query, &state)
            .await
            .unwrap()
            .unwrap();

    let trades_in: Vec<TradeIn> = backtest_result.instrument.trades_in;
    let trades_out: Vec<TradeOut> = backtest_result.instrument.trades_out;
    let orders: Vec<Order> = backtest_result.instrument.orders;
    let trades = &(&trades_in, &trades_out, &orders);
    let time_frame = strategy_result.time_frame.to_string();
    let higher_time_frame = match &strategy_result.higher_time_frame {
        Some(htf) => htf.to_string(),
        None => "".to_string(),
    };

    let instrument =
        db::back_test::find_backtest_instrument_by_symbol_time_frame(&symbol, &time_frame, &state)
            .await
            .unwrap()
            .unwrap();

    let htf_instrument = db::back_test::find_htf_backtest_instrument_by_symbol_time_frame(
        &symbol,
        &higher_time_frame,
        &state,
    )
    .await
    .unwrap();

    let htf_instrument = match htf_instrument {
        Some(htf_ins) => HTFInstrument::HTFInstrument(htf_ins),
        None => HTFInstrument::None,
    };

    let output_file = [
        &env::var("BACKEND_PLOTTER_OUTPUT_FOLDER").unwrap(),
        &strategy_result.strategy,
        "_",
        &symbol,
        ".png",
    ]
    .concat();

    Backend::new()
        .render(
            ExecutionMode::BackTest,
            &instrument,
            &htf_instrument,
            trades,
            &output_file,
        )
        .unwrap();

    let mut image_path = PathBuf::new();
    image_path.push(output_file);

    let file = fs::NamedFile::open(image_path).unwrap();

    log::info!(
        "[BACKTEST CHART RENDER] {:?} {:?} {:?}",
        &strategy_result.strategy,
        Local::now(),
        now.elapsed()
    );

    Ok(file.use_last_modified(true))
}
