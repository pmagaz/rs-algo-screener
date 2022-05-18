use super::helpers::get_collection;
use crate::models::app_state::AppState;
use crate::models::backtest_strategy::BackTestStrategyResult;
use crate::models::instrument::Instrument;

use actix_web::web;
use bson::{doc, Document};
use futures::StreamExt;
use mongodb::error::Error;
use mongodb::options::{FindOneAndReplaceOptions, FindOneOptions, FindOptions};
use rs_algo_shared::models::backtest_instrument::*;
use std::env;

pub async fn find_instruments(
    query: Document,
    state: &web::Data<AppState>,
) -> Result<Vec<Instrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_BACKTEST_COLLECTION").unwrap();

    let collection = get_collection::<Instrument>(&state.db_mem, collection_name).await;

    let mut cursor = collection
        .find(query, FindOptions::builder().limit(50).build())
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

pub async fn find_strategy_instrument_result(
    strategy: &str,
    symbol: &str,
    state: &web::Data<AppState>,
) -> Result<Option<BackTestInstrumentResult>, Error> {
    let collection_name = &env::var("DB_BACKTEST_INSTRUMENT_RESULT_COLLECTION").unwrap();
    let collection =
        get_collection::<BackTestInstrumentResult>(&state.db_mem, collection_name).await;

    let instrument = collection
        .find_one(
            doc! { "strategy": strategy, "instrument.symbol": symbol},
            FindOneOptions::builder().build(),
        )
        .await
        .unwrap();

    Ok(instrument)
}

pub async fn find_backtest_instruments_result(
    query: Document,
    state: &web::Data<AppState>,
) -> Result<Vec<BackTestInstrumentResult>, Error> {
    let collection_name = &env::var("DB_BACKTEST_INSTRUMENT_RESULT_COLLECTION").unwrap();

    let collection =
        get_collection::<BackTestInstrumentResult>(&state.db_mem, collection_name).await;

    let mut cursor = collection
        .find(query, FindOptions::builder().limit(50).build())
        .await
        .unwrap();

    let mut docs: Vec<BackTestInstrumentResult> = vec![];

    while let Some(result) = cursor.next().await {
        match result {
            Ok(instrument) => docs.push(instrument),
            _ => {}
        }
    }
    Ok(docs)
}

pub async fn find_strategies_result(
    query: Document,
    state: &web::Data<AppState>,
) -> Result<Vec<BackTestStrategyResult>, Error> {
    let collection_name = &env::var("DB_BACKTEST_STRATEGY_RESULT_COLLECTION").unwrap();

    let collection = get_collection::<BackTestStrategyResult>(&state.db_mem, collection_name).await;

    let mut cursor = collection
        .find(
            query,
            FindOptions::builder()
                .sort(doc! {"avg_profit_factor":-1})
                .build(),
        )
        .await
        .unwrap();

    let mut docs: Vec<BackTestStrategyResult> = vec![];

    while let Some(result) = cursor.next().await {
        match result {
            Ok(instrument) => docs.push(instrument),
            _ => {}
        }
    }
    Ok(docs)
}

pub async fn upsert_instruments_result(
    doc: &BackTestInstrumentResult,
    state: &web::Data<AppState>,
) -> Result<Option<BackTestInstrumentResult>, Error> {
    let collection_name = &env::var("DB_BACKTEST_INSTRUMENT_RESULT_COLLECTION").unwrap();
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

pub async fn upsert_strategies_result(
    doc: &BackTestStrategyResult,
    state: &web::Data<AppState>,
) -> Result<Option<BackTestStrategyResult>, Error> {
    let collection_name = &env::var("DB_BACKTEST_STRATEGY_RESULT_COLLECTION").unwrap();
    let collection = get_collection::<BackTestStrategyResult>(&state.db_mem, collection_name).await;

    collection
        .find_one_and_replace(
            doc! { "strategy": doc.strategy.clone() },
            doc,
            FindOneAndReplaceOptions::builder()
                .upsert(Some(true))
                .build(),
        )
        .await
}

pub async fn find_backtest_instrument_by_symbol(
    symbol: &str,
    state: &web::Data<AppState>,
) -> Result<Option<Instrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_BACKTEST_COLLECTION").unwrap();
    let collection = get_collection::<Instrument>(&state.db_mem, collection_name).await;

    let instrument = collection
        .find_one(doc! { "symbol": symbol}, FindOneOptions::builder().build())
        .await
        .unwrap();

    Ok(instrument)
}
