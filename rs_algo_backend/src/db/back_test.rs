use super::helpers::get_collection;
use crate::models::app_state::AppState;
use crate::models::instrument::Instrument;

use actix_web::web;
use bson::doc;
use futures::StreamExt;
use mongodb::error::Error;
use mongodb::options::{FindOneAndReplaceOptions, FindOneOptions, FindOptions};
use rs_algo_shared::models::backtest_instrument::*;
use std::env;

pub async fn find_all(state: &web::Data<AppState>) -> Result<Vec<Instrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_BACKTEST_COLLECTION").unwrap();

    let collection = get_collection::<Instrument>(&state.db_mem, collection_name).await;

    let mut cursor = collection
        .find(None, FindOptions::builder().limit(20).build())
        .await
        .unwrap();

    let mut docs: Vec<Instrument> = vec![];

    while let Some(result) = cursor.next().await {
        match result {
            Ok(instrument) => docs.push(instrument),
            _ => {}
        }
    }
    Ok(docs)
}

pub async fn find_instruments(state: &web::Data<AppState>) -> Result<Vec<Instrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_BACKTEST_COLLECTION").unwrap();

    let collection = get_collection::<Instrument>(&state.db_mem, collection_name).await;

    let mut cursor = collection.find(None, None).await.unwrap();

    let mut docs: Vec<Instrument> = vec![];

    while let Some(result) = cursor.next().await {
        match result {
            Ok(instrument) => docs.push(instrument),
            _ => {}
        }
    }
    Ok(docs)
}

pub async fn upsert(
    doc: &BackTestInstrumentResult,
    state: &web::Data<AppState>,
) -> Result<Option<BackTestInstrumentResult>, Error> {
    let collection_name = &env::var("DB_BACKTEST_RESULT_COLLECTION").unwrap();
    let collection =
        get_collection::<BackTestInstrumentResult>(&state.db_mem, collection_name).await;

    collection
        .find_one_and_replace(
            doc! { "instrument.symbol": doc.instrument.symbol.clone(), "strategy": doc.strategy.clone() },
            doc,
            FindOneAndReplaceOptions::builder()
                .upsert(Some(true))
                .build(),
        )
        .await
}
