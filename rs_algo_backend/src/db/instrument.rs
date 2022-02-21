use crate::helpers::date::Local;
use crate::models::app_state::AppState;
use crate::models::instrument::InstrumentRes;

use actix_web::web;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::options::FindOneAndReplaceOptions;

pub async fn insert(
    mut doc: InstrumentRes,
    state: &web::Data<AppState>,
) -> Result<Option<InstrumentRes>, Error> {
    let db_name = &state.db_name;

    let collection = &state
        .db
        .database(db_name)
        .collection::<InstrumentRes>("instruments");

    // collection
    //     .find_one_and_update(
    //         doc! { "symbol": doc.symbol },
    //         UpdateModifications::Document(doc! { "updated": Local::now().to_string() }),
    //         FindOneAndUpdateOptions::builder()
    //             .upsert(Some(true))
    //             .build(),
    //     )
    //     .await

    doc.updated = Local::now().to_string();

    collection
        .find_one_and_replace(
            doc! { "symbol": doc.symbol.clone() },
            doc,
            FindOneAndReplaceOptions::builder()
                .upsert(Some(true))
                .build(),
        )
        .await
}
