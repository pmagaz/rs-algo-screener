use super::helpers::{compact_instrument, get_collection};
use crate::helpers::date::Local;
use crate::models::app_state::AppState;
use crate::models::instrument::{
    CompactIndicator, CompactIndicators, CompactInstrument, CompactInstrument2, Instrument,
    Patterns,
};

use actix_web::web;
use bson::{doc, Document};
use futures::stream::StreamExt;
use mongodb::error::Error;
use mongodb::options::{FindOneAndReplaceOptions, FindOptions};
use std::env;

pub async fn find_by_params(
    state: &web::Data<AppState>,
    params: String,
) -> Result<Vec<CompactInstrument>, Error> {
    let collection_name = &env::var("DATABASE_INSTRUMENTS_COLLECTION").unwrap();

    println!("[PARAMS RECEIVED] {:?} ", params);
    let collection = get_collection::<CompactInstrument>(state, collection_name).await;
    let filter: Document = serde_json::from_str(&params).unwrap();
    let find_options = FindOptions::builder().build();
    let mut cursor = collection.find(filter, find_options).await.unwrap();
    let mut docs: Vec<CompactInstrument> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => {
                docs.push(doc);
            }
            _ => {}
        }
    }
    Ok(docs)
}

pub async fn insert_compact(
    mut doc: CompactInstrument,
    state: &web::Data<AppState>,
) -> Result<Option<CompactInstrument>, Error> {
    let collection_name = &env::var("DATABASE_INSTRUMENTS_COMPACT_COLLECTION").unwrap();
    let collection = get_collection::<CompactInstrument>(state, collection_name).await;

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

pub async fn insert(
    doc: &Instrument,
    state: &web::Data<AppState>,
) -> Result<Option<Instrument>, Error> {
    let collection_name = &env::var("DATABASE_INSTRUMENTS_COLLECTION").unwrap();
    let collection = get_collection::<Instrument>(state, collection_name).await;

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

/*fn doc_to_instrument(doc: &Document) -> CompactInstrument2 {
    CompactInstrument2 {
        symbol: doc.get_str("symbol").unwrap().to_string(),
        time_frame: doc.get_str("time_frame").unwrap().to_string(),
        current_price: doc.get_f64("current_price").unwrap(),
        current_candle: doc.get_str("current_candle").unwrap().to_string(),
        updated: doc.get_str("current_candle").unwrap().to_string(),
        patterns: doc.get_array("patterns").unwrap(),
    }
}

pub async fn find_all(state: &web::Data<AppState>) -> Result<Vec<CompactInstrument>, Error> {
    let collection = get_collection(state, "compact_instruments").await;
    let filter = doc! {"current_candle": "Doji"};
    let find_options = FindOptions::builder().build();
    let mut cursor = collection.find(None, find_options).await.unwrap();
    let mut docs: Vec<CompactInstrument> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => {
                docs.push(doc);
            }
            _ => {}
        }
    }
    Ok(docs)
}
 */
