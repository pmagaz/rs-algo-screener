use crate::db;
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::render_chart::Backend;

use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::BackTestStrategyResult;
use rs_algo_shared::models::mode::*;
use rs_algo_shared::models::order::Order;
use rs_algo_shared::models::tick::InstrumentTick;
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

    let prices: Vec<InstrumentTick> = db::back_test::find_prices(&state).await.unwrap();

    Ok(HttpResponse::Ok().json(prices))
}

pub async fn find_price(
    path: web::Path<(String)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let _now = Instant::now();
    let (symbol) = path.into_inner();
    log::info!("Request for at {:?}", Local::now());

    let price = db::back_test::find_price(&symbol, &state).await.unwrap();

    Ok(HttpResponse::Ok().json(price))
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

// pub async fn find_mock(
//     path: web::Path<(String, String, u32)>,
//     state: web::Data<AppState>,
// ) -> Result<HttpResponse, RsAlgoError> {
//     let now = Instant::now();
//     let (symbol, time_frame, limit) = path.into_inner();

//     let time_frame = TimeFrameType::from_str(&time_frame);
//     let instrument = read_csv(&symbol, &time_frame, limit).await?;

//     Ok(HttpResponse::Ok().json(instrument))
// }

// fn round_down_to_interval(time: DateTime<Local>, time_frame: &TimeFrameType) -> DateTime<Local> {
//     match time_frame {
//         TimeFrameType::M5 => {
//             time - Duration::minutes(time.minute() as i64 % 5)
//                 - Duration::seconds(time.second() as i64)
//                 - Duration::nanoseconds(time.nanosecond() as i64)
//         }
//         TimeFrameType::M15 => {
//             time - Duration::minutes(time.minute() as i64 % 15)
//                 - Duration::seconds(time.second() as i64)
//                 - Duration::nanoseconds(time.nanosecond() as i64)
//         }
//         TimeFrameType::M30 => {
//             time - Duration::minutes(time.minute() as i64 % 30)
//                 - Duration::seconds(time.second() as i64)
//                 - Duration::nanoseconds(time.nanosecond() as i64)
//         }
//         TimeFrameType::H1 => time
//             .with_minute(0)
//             .unwrap()
//             .with_second(0)
//             .unwrap()
//             .with_nanosecond(0)
//             .unwrap(),
//         TimeFrameType::H4 => {
//             let hour_rounded = time.hour() - (time.hour() % 4);
//             time.with_hour(hour_rounded)
//                 .unwrap()
//                 .with_minute(0)
//                 .unwrap()
//                 .with_second(0)
//                 .unwrap()
//                 .with_nanosecond(0)
//                 .unwrap()
//         }
//         _ => time,
//     }
// }

// async fn read_csv(
//     symbol: &str,
//     time_frame: &TimeFrameType,
//     records_limit: u32,
// ) -> Result<Vec<DOHLC>, RsAlgoError> {
//     let file_path = &format!(
//         "{}{}_{}.csv",
//         env::var("BACKEND_HISTORIC_DATA_FOLDER").unwrap(),
//         symbol,
//         "M1",
//     );

//     let file = File::open(Path::new(&file_path)).map_err(|_| RsAlgoError::File)?;

//     let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

//     let mut count = 0;
//     let mut data: BTreeMap<DateTime<Local>, (f64, f64, f64, f64, f64)> = BTreeMap::new();
//     for result in rdr.records() {
//         if records_limit != 0 && count >= records_limit {
//             break;
//         }
//         let record: StringRecord = result.map_err(|_| RsAlgoError::File)?;
//         let date_str = format!("{} {}", &record[0], &record[1]);
//         let date_time = Local
//             .datetime_from_str(&date_str, "%Y.%m.%d %H:%M")
//             .map_err(|_| RsAlgoError::File)?;

//         let open = record[2].parse::<f64>().map_err(|_| RsAlgoError::File)?;
//         let high = record[3].parse::<f64>().map_err(|_| RsAlgoError::File)?;
//         let low = record[4].parse::<f64>().map_err(|_| RsAlgoError::File)?;
//         let close = record[5].parse::<f64>().map_err(|_| RsAlgoError::File)?;
//         let volume = record[6].parse::<f64>().map_err(|_| RsAlgoError::File)?;

//         let rounded_datetime = round_down_to_interval(date_time, time_frame);
//         data.entry(rounded_datetime)
//             .and_modify(|e| {
//                 e.0 = if e.0 == 0.0 { open } else { e.0 };
//                 e.1 = e.1.max(high);
//                 e.2 = e.2.min(low);
//                 e.3 = close;
//                 e.4 += volume;
//             })
//             .or_insert((open, high, low, close, volume));

//         count += 1;
//     }

//     let final_data = data
//         .into_iter()
//         .map(|(dt, ohlc)| (dt, ohlc.0, ohlc.1, ohlc.2, ohlc.3, ohlc.4))
//         .collect();

//     Ok(final_data)
// }

// fn convert_m1(
//     symbol: &str,
//     time_frame: &str,
//     limit: u32,
//     interval: TimeFrameType,
// ) -> Result<Vec<DOHLC>, Box<dyn std::error::Error>> {
//     let file_path = format!("rs_algo_backend/data/{}_{}.csv", symbol, time_frame);
//     let file = File::open(Path::new(&file_path)).map_err(|_| RsAlgoError::File)?;

//     let mut rdr = csv::ReaderBuilder::new()
//         .has_headers(false)
//         .from_reader(file);

//     let mut data: BTreeMap<NaiveDateTime, (f64, f64, f64, f64, f64)> = BTreeMap::new();

//     for result in rdr.records() {
//         let record = result?;
//         let date_str = format!("{} {}", record[0].to_string(), record[1].to_string());

//         let date_time = Local
//             .datetime_from_str(&date_str, "%Y.%m.%d %H:%M")
//             .map_err(|_| RsAlgoError::File)?;
//         let rounded_datetime = round_down_to_interval(datetime, &interval);
//         let open = record[2].parse::<f64>()?;
//         let high = record[3].parse::<f64>()?;
//         let low = record[4].parse::<f64>()?;
//         let close = record[5].parse::<f64>()?;
//         let volume = record[6].parse::<f64>()?;

//         data.entry(rounded_datetime)
//             .and_modify(|e| {
//                 e.1 = e.1.max(high);
//                 e.2 = e.2.min(low);
//                 e.3 = close;
//                 e.4 += volume;
//             })
//             .or_insert((open, high, low, close, volume));
//     }

//     let records = data
//         .into_iter()
//         .map(|(dt, (open, high, low, close, volume))| {
//             let local_dt = Local.from_utc_datetime(&dt);
//             (local_dt, open, high, low, close, volume)
//         })
//         .collect();

//     Ok(records)
// }
