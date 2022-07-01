use super::helpers::get_collection;
use crate::models::app_state::AppState;
use rs_algo_shared::models::watch_instrument::*;

use actix_web::web;
use bson::doc;
use futures::StreamExt;
use mongodb::error::Error;
use mongodb::results::DeleteResult;
use mongodb::options::{DeleteOptions, FindOneAndReplaceOptions};
use std::env;

pub async fn find_all(state: &web::Data<AppState>) -> Result<Vec<WatchInstrument>, Error> {
    let collection_name = &env::var("DB_WATCHLIST_COLLECTION").unwrap();

    let collection = get_collection::<WatchInstrument>(&state.db_hdd, collection_name).await;

    let mut cursor = collection.find(None, None).await.unwrap();

    let mut docs: Vec<WatchInstrument> = vec![];

    while let Some(result) = cursor.next().await {
        match result {
            Ok(watch_instrument) => docs.push(watch_instrument),
            _ => {}
        }
    }
    Ok(docs)
}

pub async fn upsert(
    doc: &WatchInstrument,
    state: &web::Data<AppState>,
) -> Result<Option<WatchInstrument>, Error> {
    let collection_name = &env::var("DB_WATCHLIST_COLLECTION").unwrap();
    let collection = get_collection::<WatchInstrument>(&state.db_hdd, collection_name).await;

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


pub async fn delete(
    doc: &WatchInstrument,
    state: &web::Data<AppState>,
) -> Result<DeleteResult, Error> {
    let collection_name = &env::var("DB_WATCHLIST_COLLECTION").unwrap();
    let collection = get_collection::<WatchInstrument>(&state.db_hdd, collection_name).await;

    collection
        .delete_one(
            doc! { "symbol": doc.symbol.clone() },
            DeleteOptions::builder().build(),
        )
        .await
}
