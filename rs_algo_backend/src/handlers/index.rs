use crate::db;
use crate::error::CustomError;
use crate::models::app_state::AppState;
use crate::models::instrument::Instrument;

use actix_web::{web, HttpResponse, Responder};

pub async fn index(
    data: web::Json<Instrument>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    // let db = db::instruments::find_access_code(doc! {"access_code": &data.access_code}, &state)
    //     .await
    //     .map_err(|_e| CustomError::Forbidden)?
    //     .unwrap();

    let response = Instrument {
        grant_type: "".to_owned(),
        access_code: "".to_owned(),
        redirect_url: "".to_owned(),
    };
    Ok(HttpResponse::Ok().json(response))
}
