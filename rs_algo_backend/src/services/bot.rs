use crate::db;
use crate::error::RsAlgoError;
use crate::models::app_state::AppState;
use crate::render_chart::{Backend, BackendMode};
use rs_algo_shared::models::order::Order;
use rs_algo_shared::models::trade::{TradeIn, TradeOut};

use actix::{Actor, StreamHandler};
use actix_files as fs;
use actix_web::{web, Error, HttpRequest, HttpResponse, HttpServer};
use rs_algo_shared::helpers::date::Local;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::time::Instant;

// struct MyWs;

// impl Actor for MyWs {
//     type Context = ws::WebsocketContext<Self>;
// }

// /// Handler for ws::Message message
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
//             Ok(ws::Message::Text(text)) => ctx.text(text),
//             Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
//             _ => (),
//         }
//     }
// }

pub async fn find(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    //let resp = ws::start(MyWs {}, &req, stream);
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

    let bot = db::bot::find_by_id(&*id, &state).await.unwrap().unwrap();

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
            BackendMode::Bot,
            &bot.instrument(),
            &bot.htf_instrument(),
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
