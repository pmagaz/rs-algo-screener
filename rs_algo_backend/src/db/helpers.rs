use crate::error::Result;
use crate::models::app_state::AppState;
use crate::models::instrument::{CompactInstrument, Instrument};

use actix_web::web;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::Collection;

pub async fn get_collection(
    state: &web::Data<AppState>,
    collection: &str,
) -> Collection<Instrument> {
    state
        .db
        .database(&state.db_name)
        .collection::<Instrument>(collection)
}

// fn compact_instrument(doc: &Document) -> Result<CompactInstrument> {
//     let instrument = CompactInstrument {
//         symbol: doc.get_str("symbol").unwrap().to_owned(),
//         time_frame: doc.get("time_frame"),
//         current_price: doc.getf64("current_price").unwrap().to_owned(),
//     };

//     Ok(instrument)
// }
