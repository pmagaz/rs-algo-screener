use crate::db;
use crate::error::CustomError;
use crate::models::app_state::AppState;

use actix_web::{web, HttpResponse, Responder};
use shared::models::InstrumentRes;

pub async fn instrument(
    data: String,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let response: InstrumentRes = serde_json::from_str(&data).unwrap();
    let _insert_result = db::instrument::insert(response, &state).await.unwrap();
    println!("[Request]");

    Ok(HttpResponse::Ok().body("ok"))
}
