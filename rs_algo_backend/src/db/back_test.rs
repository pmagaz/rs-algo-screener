use super::helpers::*;
use crate::error;
use crate::models::app_state::AppState;
use crate::models::backtest_instrument::BackTestInstrumentResult;
use crate::models::backtest_strategy::BackTestStrategyResult;

use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::helpers::symbols::{crypto, forex, sp500};
use rs_algo_shared::models::pricing::*;
use rs_algo_shared::scanner::instrument::*;

use actix_web::web;
use bson::{doc, Document};
use futures::StreamExt;
use mongodb::error::Error;
use mongodb::options::{FindOneAndReplaceOptions, FindOneOptions, FindOptions};
use std::env;

pub async fn find_one(
    symbol: &str,
    time_frame: &str,
    state: &web::Data<AppState>,
) -> Result<Option<Instrument>, Error> {
    let collection_name = &[
        &env::var("DB_BACKTEST_INSTRUMENTS_COLLECTION").unwrap(),
        "_",
        &time_frame,
    ]
    .concat();

    log::info!("[FINDONE] from {}", collection_name);

    let collection = get_collection::<Instrument>(&state.db_mem, &collection_name).await;

    let instrument = collection
        .find_one(doc! { "symbol": symbol}, FindOneOptions::builder().build())
        .await
        .unwrap();

    Ok(instrument)
}

pub async fn find_instruments(
    query: Document,
    offset: u64,
    limit: i64,
    time_frame: String,
    state: &web::Data<AppState>,
) -> Result<Vec<Instrument>, Error> {
    let collection_name = &[
        &env::var("DB_BACKTEST_INSTRUMENTS_COLLECTION").unwrap(),
        "_",
        &time_frame,
    ]
    .concat();

    let collection = get_collection::<Instrument>(&state.db_mem, collection_name).await;

    let mut cursor = collection
        .find(
            query,
            FindOptions::builder().skip(offset).limit(limit).build(),
        )
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

pub async fn find_backtest_compact_instruments(
    query: Document,
    offset: u64,
    limit: i64,
    state: &web::Data<AppState>,
) -> Result<Vec<CompactInstrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_COMPACT_COLLECTION").unwrap();

    let sp500_symbols = sp500::get_symbols();
    let forex_symbols = forex::get_symbols();
    let crypto_symbols = crypto::get_symbols();

    let collection = get_collection::<CompactInstrument>(&state.db_mem, collection_name).await;

    let mut cursor = collection
        .find(
            query,
            FindOptions::builder().skip(offset).limit(limit).build(),
        )
        .await
        .unwrap();

    let mut docs: Vec<CompactInstrument> = vec![];

    while let Some(result) = cursor.next().await {
        match result {
            Ok(instrument) => {
                if symbol_in_list(&instrument.symbol, &sp500_symbols)
                    || symbol_in_list(&instrument.symbol, &forex_symbols)
                    || symbol_in_list(&instrument.symbol, &crypto_symbols)
                {
                    docs.push(instrument)
                }
            }
            _ => {}
        }
    }
    Ok(docs)
}

pub async fn find_strategy_instrument_result(
    query: Document,
    state: &web::Data<AppState>,
) -> Result<Option<BackTestInstrumentResult>, Error> {
    let collection_name = &env::var("DB_BACKTEST_INSTRUMENT_RESULT_COLLECTION").unwrap();
    let collection =
        get_collection::<BackTestInstrumentResult>(&state.db_mem, collection_name).await;

    let instrument = collection
        .find_one(query, FindOneOptions::builder().build())
        .await
        .unwrap();

    Ok(instrument)
}

pub async fn find_backtest_instruments_result(
    query: Document,
    limit: i64,
    state: &web::Data<AppState>,
) -> Result<Vec<BackTestInstrumentResult>, Error> {
    let collection_name = &env::var("DB_BACKTEST_INSTRUMENT_RESULT_COLLECTION").unwrap();

    let collection =
        get_collection::<BackTestInstrumentResult>(&state.db_mem, collection_name).await;

    let mut cursor = collection
        .find(
            query,
            FindOptions::builder()
                .limit(limit)
                .sort(doc! {"net_profit_per":-1})
                .build(),
        )
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
                .sort(doc! {"avg_net_profit_per":-1})
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
            doc! { "strategy": doc.strategy.clone(), "strategy_type": doc.strategy_type.to_string(), "market": doc.market.to_string(),  "instrument.symbol": doc.instrument.symbol.clone() },
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
            doc! { "strategy": doc.strategy.clone(), "strategy_type": doc.strategy_type.to_string(), "market": doc.market.to_string(),   },
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
    let collection_name = &[
        &env::var("DB_BACKTEST_INSTRUMENTS_COLLECTION").unwrap(),
        "_",
        &env::var("BASE_TIME_FRAME").unwrap(),
    ]
    .concat();
    let collection = get_collection::<Instrument>(&state.db_mem, collection_name).await;

    let instrument = collection
        .find_one(doc! { "symbol": symbol}, FindOneOptions::builder().build())
        .await
        .unwrap();

    Ok(instrument)
}

pub async fn find_htf_backtest_instrument_by_symbol(
    symbol: &str,
    state: &web::Data<AppState>,
) -> Result<Option<Instrument>, Error> {
    let collection_name = &[
        &env::var("DB_BACKTEST_INSTRUMENTS_COLLECTION").unwrap(),
        "_",
        &env::var("UPPER_TIME_FRAME").unwrap(),
    ]
    .concat();
    let collection = get_collection::<Instrument>(&state.db_mem, collection_name).await;

    let instrument = collection
        .find_one(doc! { "symbol": symbol}, FindOneOptions::builder().build())
        .await
        .unwrap();

    Ok(instrument)
}

pub async fn find_prices(state: &web::Data<AppState>) -> Result<Vec<Pricing>, Error> {
    let collection_name = &env::var("DB_PRICING_COLLECTION").unwrap();
    let collection = get_collection::<Pricing>(&state.db_mem, collection_name).await;

    let mut cursor = collection
        .find(doc! {}, FindOptions::builder().build())
        .await
        .unwrap();

    let mut prices: Vec<Pricing> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(spread) => {
                prices.push(spread);
            }
            _ => {}
        }
    }
    Ok(prices)
}

pub async fn find_price(
    symbol: &str,
    state: &web::Data<AppState>,
) -> Result<Option<Pricing>, Error> {
    let collection_name = &env::var("DB_PRICING_COLLECTION").unwrap();
    let collection = get_collection::<Pricing>(&state.db_mem, collection_name).await;

    let instrument = collection
        .find_one(doc! { "symbol": symbol }, FindOneOptions::builder().build())
        .await
        .unwrap();

    Ok(instrument)
}
