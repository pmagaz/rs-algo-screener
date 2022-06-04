use crate::db;
use crate::db::helpers::compact_instrument;
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::render_image::Backend;
use crate::strategies::general::General;

use rs_algo_shared::models::api::*;

use rs_algo_shared::models::instrument::*;
use std::time::Instant;

use actix_files as fs;
use actix_web::{web, HttpResponse};
use rs_algo_shared::helpers::date::Local;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Mode {
    pub mode: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct SymbolQuery {
    pub symbol: String,
}

pub async fn find_one(
    query: web::Query<SymbolQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let symbol = &query.symbol;

    let instrument = db::instrument::find_by_symbol(symbol, &state)
        .await
        .unwrap()
        .unwrap();

    println!(
        "[FINDONE] {:?} {:?} {:?}",
        instrument.symbol,
        Local::now(),
        now.elapsed()
    );

    Ok(HttpResponse::Ok().json(instrument))
}

pub async fn chart(
    symbol: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<fs::NamedFile, RsAlgoError> {
    let now = Instant::now();

    let instrument = db::instrument::find_by_symbol(&*symbol, &state)
        .await
        .unwrap()
        .unwrap();

    let output_file = [
        &env::var("BACKEND_PLOTTER_OUTPUT_FOLDER").unwrap(),
        &instrument.symbol,
        ".png",
    ]
    .concat();

    Backend::new().render(&instrument, &(&vec![], &vec![]), &output_file);

    let mut image_path = PathBuf::new();
    image_path.push(output_file);

    let file = fs::NamedFile::open(image_path).unwrap();

    println!(
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

    println!("[FIND] {:?} {:?}", Local::now(), now.elapsed());

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

    println!("[FIND] {:?} {:?}", Local::now(), now.elapsed());

    Ok(HttpResponse::Ok().json(instruments))
}

pub async fn find_all(
    _params: String,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    let instruments = db::instrument::find_all(&state).await.unwrap();

    println!("[FIND ALL] {:?} {:?}", Local::now(), now.elapsed());

    Ok(HttpResponse::Ok().json(instruments))
}

pub async fn upsert(
    query: web::Query<Mode>,
    instrument: String,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let _now = Instant::now();
    let mode = &query.mode;
    println!(
        "[INSTRUMENT] Received at {:?} for mode {:?}",
        Local::now(),
        mode
    );

    let _now = Instant::now();
    let mut instrument: Instrument = serde_json::from_str(&instrument).unwrap();
    let symbol = instrument.symbol.clone();

    //FOR XTB
    if symbol.contains('_') {
        let symbol_str: Vec<&str> = symbol.split('_').collect();
        instrument.symbol = symbol_str[0].to_owned();
    }

    let insert_compact_instruments_detail = env::var("INSERT_COMPACT_INSTRUMENTS_DETAIL")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    if insert_compact_instruments_detail {
        let now = Instant::now();

        let _insert_result = db::instrument::insert_detail(mode, &instrument, &state)
            .await
            .unwrap();

        println!(
            "{} {:?} at {:?} in {:?}",
            match mode.as_ref() {
                "daily" => "[INSTRUMENT UPSERTED]",
                "backtest" => "[BACKTEST INSTRUMENT UPSERTED]",
                &_ => "",
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

    if mode != "backtest" && insert_compact_instruments {
        let now = Instant::now();
        let _insert_compact =
            db::instrument::upsert(compact_instrument(instrument).unwrap(), &state)
                .await
                .unwrap();

        println!(
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
