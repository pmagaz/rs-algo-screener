use super::helpers::get_collection;
use crate::models::app_state::AppState;
use crate::models::backtest_instrument::BackTestInstrument;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::*;
use rs_algo_shared::models::watch_instrument::*;

use actix_web::web;
use bson::doc;
use futures::StreamExt;
use mongodb::error::Error;
use mongodb::options::FindOneAndReplaceOptions;
use std::env;

pub async fn find_all(state: &web::Data<AppState>) -> Result<Vec<BackTestInstrument>, Error> {
    let collection_name = &env::var("DB_BACKTEST_COLLECTION").unwrap();

    let collection = get_collection::<BackTestInstrument>(&state.db_mem, collection_name).await;

    let mut cursor = collection.find(None, None).await.unwrap();

    let mut docs: Vec<BackTestInstrument> = vec![];

    while let Some(result) = cursor.next().await {
        match result {
            Ok(instrument) => docs.push(instrument),
            _ => {}
        }
    }
    Ok(docs)
}

pub async fn find_instruments(
    state: &web::Data<AppState>,
) -> Result<Vec<BackTestInstrument>, Error> {
    let collection_name = &env::var("DB_BACKTEST_COLLECTION").unwrap();

    let collection = get_collection::<BackTestInstrument>(&state.db_mem, collection_name).await;

    let mut cursor = collection.find(None, None).await.unwrap();

    let mut docs: Vec<BackTestInstrument> = vec![];

    while let Some(result) = cursor.next().await {
        match result {
            Ok(instrument) => docs.push(instrument),
            _ => {}
        }
    }
    Ok(docs)
}

pub async fn upsert(
    doc: WatchInstrument,
    state: &web::Data<AppState>,
) -> Result<Option<WatchInstrument>, Error> {
    let collection_name = &env::var("DB_BACKTEST_COLLECTION").unwrap();
    let collection = get_collection::<WatchInstrument>(&state.db_mem, collection_name).await;

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
