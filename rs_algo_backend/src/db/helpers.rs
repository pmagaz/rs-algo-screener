use crate::error::Result;

use crate::models::db::Db;
use mongodb::Collection;
use rs_algo_shared::models::divergence::*;
use rs_algo_shared::models::indicator::*;
use rs_algo_shared::models::instrument::*;
use rs_algo_shared::models::pattern::*;
use rs_algo_shared::models::status::Status;


use std::cmp::Ordering;

pub fn get_collection_name(collection: &str, time_frame: &str) -> String {
    let arr_str = collection.split('_').collect::<Vec<_>>();
    let time_frame_code = arr_str.last().unwrap();
    
    collection.replace(time_frame_code, time_frame)
}
pub async fn get_collection<T>(db: &Db, collection: &str) -> Collection<T> {
    db.client.database(&db.name).collection::<T>(collection)
}

pub fn compact_instrument(doc: Instrument) -> Result<CompactInstrument> {
    let len = doc.data.len();
    let second_last = match len.cmp(&2) {
        Ordering::Greater => len - 2,
        Ordering::Equal => len - 1,
        Ordering::Less => len,
    };

    let doc = CompactInstrument {
        symbol: doc.symbol,
        date: doc.date,
        market: doc.market,
        time_frame: doc.time_frame,
        current_price: doc.current_price,
        avg_volume: doc.avg_volume,
        prev_price: doc.data.get(second_last).unwrap().close,
        current_candle: doc.current_candle,
        prev_candle: doc.data.get(second_last).unwrap().candle_type.clone(),
        //FIXME ADD ARRAY
        indicators: CompactIndicators {
            macd: CompactIndicator {
                current_a: *doc.indicators.macd.data_a.last().unwrap(),
                current_b: *doc.indicators.macd.data_b.last().unwrap(),
                prev_a: *doc.indicators.macd.data_a.get(second_last).unwrap(),
                prev_b: *doc.indicators.macd.data_b.get(second_last).unwrap(),
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
            stoch: CompactIndicator {
                current_a: *doc.indicators.stoch.data_a.last().unwrap(),
                current_b: *doc.indicators.stoch.data_b.last().unwrap(),
                prev_a: *doc.indicators.stoch.data_a.get(second_last).unwrap(),
                prev_b: *doc.indicators.stoch.data_b.get(second_last).unwrap(),
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
            atr: CompactIndicator {
                current_a: *doc.indicators.atr.data_a.last().unwrap(),
                prev_a: *doc.indicators.atr.data_a.get(second_last).unwrap(),
                current_b: 0.,
                prev_b: 0.,
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
            bb: CompactIndicator {
                current_a: *doc.indicators.bb.data_a.last().unwrap(),
                current_b: *doc.indicators.bb.data_b.last().unwrap(),
                current_c: *doc.indicators.bb.data_c.last().unwrap(),
                prev_a: *doc.indicators.bb.data_a.get(second_last).unwrap(),
                prev_b: *doc.indicators.bb.data_b.get(second_last).unwrap(),
                prev_c: *doc.indicators.bb.data_c.get(second_last).unwrap(),
                status: Status::Default,
            },
            bbw: CompactIndicator {
                current_a: *doc.indicators.bbw.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.bbw.data_a.get(second_last).unwrap(),
                prev_b: 0.,
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
            rsi: CompactIndicator {
                current_a: *doc.indicators.rsi.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.rsi.data_a.get(second_last).unwrap(),
                prev_b: 0.,
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },

            ema_a: CompactIndicator {
                current_a: *doc.indicators.ema_a.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.ema_a.data_a.get(second_last).unwrap(),
                prev_b: 0.,
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
            ema_b: CompactIndicator {
                current_a: *doc.indicators.ema_b.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.ema_b.data_a.get(second_last).unwrap(),
                prev_b: 0.,
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
            ema_c: CompactIndicator {
                current_a: *doc.indicators.ema_c.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.ema_c.data_a.get(second_last).unwrap(),
                prev_b: 0.,
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
        },
        patterns: Patterns {
            local_patterns: doc
                .patterns
                .local_patterns
                .into_iter()
                .rev()
                .take(3)
                .rev()
                .collect(),
            extrema_patterns: doc
                .patterns
                .extrema_patterns
                .into_iter()
                .rev()
                .take(3)
                .rev()
                .collect(),
        },
        horizontal_levels: doc.horizontal_levels,
        divergences: CompactDivergences {
            data: doc
                .divergences
                .data
                .into_iter()
                .rev()
                .take(3)
                //.rev()
                .map(|div| CompactDivergence {
                    indicator: div.indicator,
                    date: div.date,
                    divergence_type: div.divergence_type,
                })
                .collect(),
        },
    };
    Ok(doc)
}
