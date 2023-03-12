use crate::db;
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::render_chart::Backend;

use rs_algo_shared::models::mode::*;
use rs_algo_shared::models::order::Order;
use rs_algo_shared::models::trade::{TradeIn, TradeOut};


use actix_files as fs;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use rs_algo_shared::helpers::date::Local;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::time::Instant;

pub async fn find(
    _req: HttpRequest,
    _stream: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let now = Instant::now();
    let bots = db::bot::find_all(&state).await.unwrap();

    log::info!("[FIND ALL] {:?} {:?}", Local::now(), now.elapsed());

    Ok(HttpResponse::Ok().json(bots))
}

pub async fn chart(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<fs::NamedFile, RsAlgoError> {
    let now = Instant::now();

    let id = path.into_inner();

    let bot = db::bot::find_by_id(&id, &state).await.unwrap().unwrap();

    let output_file = [
        &env::var("BACKEND_PLOTTER_OUTPUT_FOLDER").unwrap(),
        bot.symbol(),
        ".png",
    ]
    .concat();

    let trades_in: &Vec<TradeIn> = bot.trades_in();
    let trades_out: &Vec<TradeOut> = bot.trades_out();
    let orders: &Vec<Order> = bot.orders();
    let trades = &(trades_in, trades_out, orders);

    Backend::new()
        .render(
            ExecutionMode::Bot,
            bot.instrument(),
            bot.htf_instrument(),
            trades,
            &output_file,
        )
        .unwrap();

    let mut image_path = PathBuf::new();
    image_path.push(output_file);

    let file = fs::NamedFile::open(image_path).unwrap();

    log::info!(
        "[CHART RENDER] {:?} {:?} {:?}",
        bot.symbol(),
        Local::now(),
        now.elapsed()
    );

    Ok(file.use_last_modified(true))
}

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
