use super::helpers::get_collection;
use crate::models::app_state::AppState;
use crate::models::instrument::{CompactInstrument, Instrument};

use rs_algo_shared::helpers::date::Local;
use rs_algo_shared::models::*;

use actix_web::web;
use bson::{doc, Document};
use chrono::Duration;
use futures::stream::StreamExt;
use mongodb::error::Error;
use mongodb::options::{FindOneAndReplaceOptions, FindOneOptions, FindOptions};
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
) -> Result<Vec<CompactInstrument>, Error> {
    let collection_name = &env::var("DB_INSTRUMENTS_COLLECTION").unwrap();

    println!("[PARAMS RECEIVED] {:?} ", params);
    let collection = get_collection::<CompactInstrument>(state, collection_name).await;

    //MOVE TO Backend
    let default_query = doc! {
        "$or": [
            {"$and": [
                { "$or": [{"current_candle":"Karakasa"},{"current_candle":"BullishGap"}, {"current_candle":"BullishCrows"}]},
                {"indicators.rsi.current_a":  {"$lte": 30 }},
            ]},
            //STOCH
            {"$and": [
                {"indicators.rsi.current_a":  {"$lte": 30 } },
                {"indicators.stoch.current_a":  {"$lt": 30 }},
                {"$expr": {"$gt": ["$indicators.stoch.current_a","$indicators.stoch.current_b"]}},
                {"$expr": {"$gt": ["$indicators.stoch.current_a","$indicators.stoch.prev_a"]}},
                {"$expr": {"$lte": ["$indicators.stoch.prev_a","$indicators.stoch.prev_b"]}}
            ]},
            // MACD
            {"$and": [
                {"$expr": {"$gt": ["$indicators.macd.current_a","$indicators.macd.current_b"]}},
                {"$expr": {"$gt": ["$indicators.macd.current_a","$indicators.stoch.prev_a"]}},
                {"$expr": {"$lte": ["$indicators.macd.prev_a","$indicators.macd.prev_b"]}}
            ]},
            // RSI
              {"$and": [
                {"patterns.local_patterns": {"$elemMatch" : {"active.target":{"$gte": 15 }, "active.date": { "$lt" : DbDateTime::from_chrono(Local::now() - Duration::days(3)) }}}},
                {"$expr": {"$gt": ["$indicators.stoch.current_a","$indicators.stoch.current_b"]}},
                {"$expr": {"$gt": ["$indicators.stoch.current_a","$indicators.stoch.prev_a"]}},
                {"$expr": {"$lte": ["$indicators.stoch.prev_a","$indicators.stoch.prev_b"]}}
            ]},

            { "symbol": { "$in": [ "BITCOIN","ETHEREUM","RIPPLE","DOGECOIN","POLKADOT ","STELLAR"] } }
        ]
    };
    let query = match params.as_ref() {
        "" => default_query,
        _ => serde_json::from_str(&params).unwrap(),
    };

    let mut cursor = collection
        .find(query, FindOptions::builder().build())
        .await
        .unwrap();

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
