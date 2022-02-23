use crate::helpers::date::Local;
use crate::models::app_state::AppState;
use crate::models::instrument::Instrument;

use actix_web::web;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::options::FindOneAndReplaceOptions;
use mongodb::results::InsertOneResult;

pub async fn insert(
    mut doc: Instrument,
    state: &web::Data<AppState>,
) -> Result<Option<Instrument>, Error> {
    let db_name = &state.db_name;

    let collection = &state
        .db
        .database(db_name)
        .collection::<Instrument>("instruments");

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
