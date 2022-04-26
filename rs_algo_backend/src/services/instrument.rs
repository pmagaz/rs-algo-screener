use crate::db;
use crate::db::helpers::compact_instrument;
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::models::Instrument;
use crate::render_image::Backend;
use crate::strategies::general::General;

pub use rs_algo_shared::models::*;
use std::time::Instant;

use actix_files as fs;
use actix_web::{web, HttpResponse};
use rs_algo_shared::helpers::date::Local;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

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

pub async fn render(
    query: web::Query<SymbolQuery>,
    state: web::Data<AppState>,
) -> Result<fs::NamedFile, RsAlgoError> {
    let now = Instant::now();
    let symbol = &query.symbol;

    let instrument = db::instrument::find_by_symbol(symbol, &state)
        .await
        .unwrap()
        .unwrap();

    let backend = Backend::new();
    let _output = backend.render(&instrument);

    let image_folder = &env::var("BACKEND_PLOTTER_OUTPUT_FOLDER").unwrap();
    let mut image_path = PathBuf::new();
    image_path.push(image_folder);
    image_path.push([symbol, ".png"].concat());

    println!("[PATH] {:?}", image_path.to_str());

    let file = fs::NamedFile::open(image_path).unwrap();

    println!(
        "[RENDER] {:?} {:?} {:?}",
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

pub async fn upsert(
    instrument: String,
    state: web::Data<AppState>,
) -> Result<HttpResponse, RsAlgoError> {
    let now = Instant::now();
    println!(
        "[INSTRUMENT] Received at {:?} in {:?}",
        Local::now(),
        now.elapsed()
    );

    let now = Instant::now();
    let mut instrument: Instrument = serde_json::from_str(&instrument).unwrap();
    let symbol = instrument.symbol.clone();

    println!(
        "[INSTRUMENT] Parsed {:?} at {:?} in {:?}",
        symbol,
        Local::now(),
        now.elapsed()
    );

    //FOR XTB
    if symbol.contains('_') {
        let symbol_str: Vec<&str> = symbol.split('_').collect();
        instrument.symbol = symbol_str[0].to_owned();
    }

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
            db::instrument::upsert(compact_instrument(instrument).unwrap(), &state)
                .await
                .unwrap();

        println!(
            "[INSTRUMENT DETAIL UPSERTED] {:?} at {:?} in {:?}",
            symbol,
            Local::now(),
            now.elapsed()
        );
    }
    Ok(HttpResponse::Ok().json(ApiResponse {
        result: "ok".to_owned(),
    }))
}
