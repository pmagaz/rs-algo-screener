use super::helpers::*;
use crate::models::app_state::AppState;
use crate::strategies::general::General;

use rs_algo_shared::models::instrument::*;

use actix_web::web;
use bson::doc;
use futures::stream::StreamExt;
use mongodb::error::Error;
use mongodb::options::{FindOneAndReplaceOptions, FindOneOptions, FindOptions};

use std::env;

pub async fn find_by_symbol(
    symbol: &str,
    state: &web::Data<AppState>,
) -> Result<Option<Instrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_COLLECTION").unwrap();
    let collection = get_collection::<Instrument>(&state.db_mem, collection_name).await;

    let instrument = collection
        .find_one(doc! { "symbol": symbol}, FindOneOptions::builder().build())
        .await
        .unwrap();

    Ok(instrument)
}

pub async fn find_by_params(
    state: &web::Data<AppState>,
    params: String,
    strategy: General,
) -> Result<Vec<CompactInstrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_COMPACT_COLLECTION").unwrap();

    log::info!("[PARAMS RECEIVED] {:?} ", params);
    let collection = get_collection::<CompactInstrument>(&state.db_mem, collection_name).await;

    //FIXME
    let query = match params.as_ref() {
        "" => strategy.query(),
        _ => serde_json::from_str(&params).unwrap(),
    };

    let cursor = collection
        .find(
            query,
            FindOptions::builder()
                //.sort(doc! {"avg_volume":-1, })
                .build(),
        )
        .await
        .unwrap();

    let docs = strategy.format_instrument(cursor).await;
    Ok(docs)
}

pub async fn find_detail_by_params(
    state: &web::Data<AppState>,
    params: String,
    strategy: General,
) -> Result<Vec<Instrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_COLLECTION").unwrap();

    log::info!("[PARAMS RECEIVED] {:?} ", params);
    let collection = get_collection::<Instrument>(&state.db_mem, collection_name).await;

    //FIXME
    let query = match params.as_ref() {
        "" => strategy.query(),
        _ => serde_json::from_str(&params).unwrap(),
    };

    let mut cursor = collection
        .find(query, FindOptions::builder().build())
        .await
        .unwrap();

    let mut instruments: Vec<Instrument> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(instrument) => {
                instruments.push(instrument);
            }
            _ => {}
        }
    }
    Ok(instruments)
}

pub async fn find_all(state: &web::Data<AppState>) -> Result<Vec<Instrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_COMPACT_COLLECTION").unwrap();

    let collection = get_collection::<Instrument>(&state.db_mem, collection_name).await;

    let mut cursor = collection
        .find(doc! {}, FindOptions::builder().build())
        .await
        .unwrap();

    let mut instruments: Vec<Instrument> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(instrument) => {
                instruments.push(instrument);
            }
            _ => {}
        }
    }
    Ok(instruments)
}

pub async fn upsert_instrument(
    mode: &str,
    time_frame: &str,
    doc: &Instrument,
    state: &web::Data<AppState>,
) -> Result<Option<Instrument>, Error> {
    let collection = match mode {
        "daily" => {
            let collection_name =
                get_collection_name(&env::var("DB_INSTRUMENTS_COLLECTION").unwrap(), time_frame);

            get_collection::<Instrument>(&state.db_mem, &collection_name).await
        }
        "backtest" => {
            let collection_name = get_collection_name(
                &env::var("DB_BACKTEST_INSTRUMENTS_COLLECTION").unwrap(),
                time_frame,
            );

            get_collection::<Instrument>(&state.db_hdd, &collection_name).await
        }
        _ => {
            get_collection::<Instrument>(
                &state.db_mem,
                &env::var("DB_INSTRUMENTS_COLLECTION").unwrap(),
            )
            .await
        }
    };

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

pub async fn upsert_compact_instrument(
    doc: CompactInstrument,
    state: &web::Data<AppState>,
) -> Result<Option<CompactInstrument>, Error> {
    let collection_name = env::var("DB_INSTRUMENTS_COMPACT_COLLECTION").unwrap();

    let collection = get_collection::<CompactInstrument>(&state.db_mem, &collection_name).await;

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
