use crate::models::app_state::AppState;

use rs_algo_shared::models::InstrumentRes;

use actix_web::web;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;

pub async fn insert(
    doc: InstrumentRes,
    state: &web::Data<AppState>,
) -> Result<InsertOneResult, Error> {
    let db_name = &state.db_name;
    let db = &state
        .db
        .database(db_name)
        .collection::<InstrumentRes>("instruments");

    db.insert_one(doc, None).await
}
