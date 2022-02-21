use crate::models::app_state::AppState;
use crate::models::instrument::InstrumentRes;

use actix_web::web;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::results::InsertOneResult;

pub async fn insert(
    doc: InstrumentRes,
    state: &web::Data<AppState>,
) -> Result<InsertOneResult, Error> {
    let db_name = &state.db_name;

    let options = FindOneAndUpdateOptions::builder()
        .upsert(Some(true))
        .build();

    let db = &state
        .db
        .database(db_name)
        .collection::<InstrumentRes>("instruments");

    // let filter = doc! {
    //     "symbol": doc.symbol
    // };

    //db.find_one_and_update(filter, doc, Some(options)).await
    db.insert_one(doc, None).await
}
