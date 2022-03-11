use super::helpers::{compact_instrument, get_collection};
use crate::helpers::date::Local;
use crate::models::app_state::AppState;
use crate::models::instrument::{
    CompactIndicator, CompactIndicators, CompactInstrument, Instrument, Patterns,
};

use actix_web::web;
use bson::{doc, Bson};
use futures::stream::StreamExt;
use mongodb::error::Error;
use mongodb::options::{FindOneAndReplaceOptions, FindOptions};
use mongodb::results::InsertOneResult;

pub async fn find_all<'a>(state: &web::Data<AppState>) -> Result<Vec<CompactInstrument>, Error> {
    let collection = get_collection(state, "instruments").await;
    let filter = doc! {"current_candle": "Doji"};
    let find_options = FindOptions::builder().build();
    let mut cursor = collection.find(filter, find_options).await.unwrap();

    let mut docs: Vec<CompactInstrument> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => {
                docs.push(compact_instrument(doc).unwrap());
            }
            _ => {}
        }
    }
    Ok(docs)
}

pub async fn insert(
    mut doc: Instrument,
    state: &web::Data<AppState>,
) -> Result<Option<Instrument>, Error> {
    let db_name = &state.db_name;

    let collection = get_collection::<Instrument>(state, "instruments").await;

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
