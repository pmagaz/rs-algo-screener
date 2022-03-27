use super::helpers::get_collection;
use crate::models::app_state::AppState;
use crate::models::instrument::{CompactInstrument, Instrument};
use crate::strategies::Strategy;

use crate::strategies::stoch::Stoch;
use actix_web::web;
use bson::{doc, Document};
use chrono::Duration;
use futures::stream::StreamExt;
use mongodb::error::Error;
use mongodb::options::{FindOneAndReplaceOptions, FindOneOptions, FindOptions};
use rs_algo_shared::helpers::date::Local;
use rs_algo_shared::models::*;
use std::env;

pub async fn find_by_symbol(
    symbol: &str,
    state: &web::Data<AppState>,
) -> Result<Option<Instrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_DETAIL_COLLECTION").unwrap();
    let collection = get_collection::<Instrument>(state, collection_name).await;

    let instrument = collection
        .find_one(doc! { "symbol": symbol}, FindOneOptions::builder().build())
        .await
        .unwrap();

    Ok(instrument)
}

pub async fn find_by_params(
    state: &web::Data<AppState>,
    params: String,
    strategy: Stoch,
) -> Result<Vec<CompactInstrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_COLLECTION").unwrap();

    println!("[PARAMS RECEIVED] {:?} ", params);
    let collection = get_collection::<CompactInstrument>(state, collection_name).await;

    let query = match params.as_ref() {
        "" => strategy.query().to_owned(),
        _ => serde_json::from_str(&params).unwrap(),
    };

    let mut cursor = collection
        .find(query, FindOptions::builder().build())
        .await
        .unwrap();

    let docs = strategy.format_instrument(cursor).await;
    Ok(docs)
}

pub async fn insert(
    doc: CompactInstrument,
    state: &web::Data<AppState>,
) -> Result<Option<CompactInstrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_COLLECTION").unwrap();
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

pub async fn insert_detail(
    doc: &Instrument,
    state: &web::Data<AppState>,
) -> Result<Option<Instrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_DETAIL_COLLECTION").unwrap();
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
