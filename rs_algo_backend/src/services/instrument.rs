use crate::db;
use crate::db::helpers::compact_instrument;
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::render_chart::Backend;
use crate::strategies::general::General;

use rs_algo_shared::models::api::*;
use rs_algo_shared::models::mode::*;
use rs_algo_shared::scanner::instrument::*;

use actix_files as fs;
use actix_web::{web, HttpResponse};
use rs_algo_shared::helpers::date::Local;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Params {
    pub mode: String,
    pub time_frame: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct SymbolQuery {
    pub symbol: String,
}

pub async fn find_one(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let symbol = path.into_inner();

    let instrument = db::instrument::find_by_symbol(&symbol, &state)
        .await
        .unwrap()
        .unwrap();

    log::info!(
        "[FINDONE] {} {} {:?}",
        instrument.symbol,
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(instrument))
}

pub async fn chart(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<fs::NamedFile, RsAlgoError> {
    let now = Instant::now();

    let symbol = path.into_inner();

    let instrument = db::instrument::find_by_symbol(&symbol, &state)
        .await
        .unwrap()
        .unwrap();

    let output_file = [
        &env::var("BACKEND_PLOTTER_OUTPUT_FOLDER").unwrap(),
        &instrument.symbol,
        ".png",
    ]
    .concat();

    Backend::new()
        .render(
            ExecutionMode::Scanner,
            &instrument,
            &HTFInstrument::None,
            &(&vec![], &vec![], &vec![]),
            &output_file,
        )
        .unwrap();

    let mut image_path = PathBuf::new();
    image_path.push(output_file);

    let file = fs::NamedFile::open(image_path).unwrap();

    log::info!(
        "[CHART RENDER] {:?} {:?} {:?}",
        symbol,
        Local::now(),
        now.elapsed()
    );

    Ok(file.use_last_modified(true))
}

pub async fn find(params: String, state: web::Data<AppState>) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let strategy = General::new().unwrap();

    let instruments = db::instrument::find_by_params(&state, params, strategy)
        .await
        .unwrap();

    log::info!("[FIND] {:?} {:?}", Local::now(), now.elapsed());

    Ok(HttpResponse::Ok().json(instruments))
}

pub async fn find_detail(
    params: String,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let strategy = General::new().unwrap();
    let instruments = db::instrument::find_detail_by_params(&state, params, strategy)
        .await
        .unwrap();

    log::info!("[FIND] {:?} {:?}", Local::now(), now.elapsed());

    Ok(HttpResponse::Ok().json(instruments))
}

pub async fn find_all(
    _params: String,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let instruments = db::instrument::find_all(&state).await.unwrap();

    log::info!("[FIND ALL] {:?} {:?}", Local::now(), now.elapsed());

    Ok(HttpResponse::Ok().json(instruments))
}

pub async fn upsert(
    query: web::Query<Params>,
    instrument: String,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let mode = &query.mode;
    let time_frame = &query.time_frame;

    let mut instrument: Instrument = serde_json::from_str(&instrument).unwrap();
    let symbol = instrument.symbol.clone();
    //let execution_mode = env::var("EXECUTION_MODE").unwrap();
    //let backtest_market = env::var("BACKTEST_MARKET").unwrap_or("".to_string());

    // match execution_mode.as_ref() {
    //     "Scanner" | "ScannerBackTest" | "BackTest" => {
    //         let symbol_str: Vec<&str> = symbol.split('_').collect();
    //         instrument.symbol = symbol_str[0].to_owned();
    //     }
    //     &_ => {
    //         log::warn!("Change fucking xtb");
    //     }
    // };

    // match backtest_market.as_ref() {
    //     "Forex" | "Crypto" => {
    //         let symbol_str: Vec<&str> = symbol.split('_').collect();
    //         instrument.symbol = symbol_str[0].to_owned();
    //     }
    //     &_ => {
    //         log::warn!("Change fucking xtb");
    //     }
    // };

    log::info!(
        "[INSTRUMENT] Received {} {:?} in {} mode at {:?}",
        instrument.symbol,
        time_frame,
        mode,
        Local::now(),
    );

    let symbol_str: Vec<&str> = symbol.split('_').collect();
    instrument.symbol = symbol_str[0].to_owned();

    let insert_compact_instruments_detail = env::var("INSERT_COMPACT_INSTRUMENTS_DETAIL")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    if insert_compact_instruments_detail {
        let now = Instant::now();

        let _insert_result =
            db::instrument::upsert_instrument(mode, time_frame, &instrument, &state)
                .await
                .unwrap();

        log::info!(
            "{} {:?} at {:?} in {:?}",
            match mode.as_ref() {
                "daily" => "[INSTRUMENT UPSERTED]",
                "backtest" => "[BACKTEST INSTRUMENT UPSERTED]",
                &_ => "Wrong mode!",
            },
            symbol,
            Local::now(),
            now.elapsed()
        );
    }

    let insert_compact_instruments = env::var("INSERT_COMPACT_INSTRUMENTS")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    if !mode.contains("backtest") && insert_compact_instruments {
        let now = Instant::now();
        let _insert_compact = db::instrument::upsert_compact_instrument(
            compact_instrument(instrument).unwrap(),
            &state,
        )
        .await
        .unwrap();

        log::info!(
            "[COMPACT INSTRUMENT UPSERTED] {:?} at {:?} in {:?}",
            symbol,
            Local::now(),
            now.elapsed()
        );
    }
    Ok(HttpResponse::Ok().json(ApiResponse {
        result: "ok".to_owned(),
    }))
}
