use actix_web::{web, HttpResponse, Responder};

use crate::models::app_state::AppState;

pub async fn index(state: web::Data<AppState>) -> impl Responder {
    let app_name = &state.app_name;
    HttpResponse::Ok().body(format!("Welcome to {}!", app_name))
}
