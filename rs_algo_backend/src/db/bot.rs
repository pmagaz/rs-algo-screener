use super::helpers::*;
use crate::models::app_state::AppState;

use rs_algo_shared::helpers::uuid;
use rs_algo_shared::models::bot::{BotData, CompactBotData};

use actix_web::web;
use bson::doc;
use futures::stream::StreamExt;
use mongodb::error::Error;
use mongodb::options::{FindOneOptions, FindOptions};
use std::env;

pub async fn find_all(state: &web::Data<AppState>) -> Result<Vec<CompactBotData>, Error> {
    let collection_name = &env::var("BOT_COLLECTION").unwrap();

    let collection = get_collection::<CompactBotData>(&state.db_bot, collection_name).await;

    let mut cursor = collection
        .find(
            doc! {},
            FindOptions::builder()
                .sort(doc! {"env":-1, "market":-1, "strategy_name":1, "time_frame": 1, "symbol":1, "strategy_stats.profit_factor":-1, "strategy_type":1, })
                .build(),
        )
        .await
        .unwrap();

    let mut bots: Vec<CompactBotData> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(instrument) => {
                bots.push(instrument);
            }
            _ => {}
        }
    }

    // bots.sort_by(|a, b| {
    //     if a.strategy_name.contains("Back") && !b.strategy_name.contains("Back") {
    //         std::cmp::Ordering::Greater
    //     } else if !a.strategy_name.contains("Back") && b.strategy_name.contains("Back") {
    //         std::cmp::Ordering::Less
    //     } else {
    //         std::cmp::Ordering::Equal
    //     }
    //});

    Ok(bots)
}

pub async fn find_by_id(id: &str, state: &web::Data<AppState>) -> Result<Option<BotData>, Error> {
    let collection_name = &env::var("BOT_COLLECTION").unwrap();

    let collection = get_collection::<BotData>(&state.db_bot, collection_name).await;

    let bot = collection
        .find_one(
            doc! { "_id": uuid::from_str(id.to_owned())},
            FindOneOptions::builder().build(),
        )
        .await
        .unwrap();

    Ok(bot)
}
