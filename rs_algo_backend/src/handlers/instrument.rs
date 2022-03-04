use crate::db;
use crate::error::CustomError;
use crate::models::app_state::AppState;
use crate::models::instrument::Instrument;

use actix_web::{web, HttpResponse, Responder};
use rs_algo_shared::helpers::date::Local;

pub async fn instrument(
    data: String,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let response: Instrument = serde_json::from_str(&data).unwrap();
    let symbol = response.symbol.clone();

    let _insert_result = db::instrument::insert(response, &state).await.unwrap();

    println!("[INSERTED] {:?} at {:?}", symbol, Local::now());

    Ok(HttpResponse::Ok().body("ok"))
}
