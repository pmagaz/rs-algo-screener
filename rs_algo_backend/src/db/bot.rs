use super::helpers::*;
use crate::models::app_state::AppState;
use crate::strategies::general::General;
use rs_algo_shared::models::bot::BotData;
use rs_algo_shared::scanner::instrument::*;

use actix_web::web;
use bson::doc;
use futures::stream::StreamExt;
use mongodb::error::Error;
use mongodb::options::{FindOneAndReplaceOptions, FindOneOptions, FindOptions};

use std::env;

pub async fn find_all(state: &web::Data<AppState>) -> Result<Vec<BotData>, Error> {
    let collection_name = &env::var("BOT_COLLECTION").unwrap();

    let collection = get_collection::<BotData>(&state.db_bot, collection_name).await;

    let mut cursor = collection
        .find(doc! {}, FindOptions::builder().build())
        .await
        .unwrap();

    let mut instruments: Vec<BotData> = vec![];
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
