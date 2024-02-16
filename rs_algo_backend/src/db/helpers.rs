use crate::error::Result;
use crate::models::db::Db;

use rs_algo_shared::indicators::Indicator;
use rs_algo_shared::models::indicator::CompactIndicator;
use rs_algo_shared::models::indicator::CompactIndicators;
use rs_algo_shared::models::status::Status;
use rs_algo_shared::scanner::divergence::*;
use rs_algo_shared::scanner::instrument::*;
use rs_algo_shared::scanner::pattern::*;

use mongodb::Collection;
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
                current_a: match doc.indicators.macd.as_ref() {
                    Some(macd) => *macd.get_data_a().last().unwrap(),
                    None => 0.,
                },
                current_b: match doc.indicators.macd.as_ref() {
                    Some(macd) => *macd.get_data_b().last().unwrap(),
                    None => 0.,
                },
                prev_a: match doc.indicators.macd.as_ref() {
                    Some(macd) => *macd.get_data_a().get(second_last).unwrap(),
                    None => 0.,
                },
                prev_b: match doc.indicators.macd.as_ref() {
                    Some(macd) => *macd.get_data_b().get(second_last).unwrap(),
                    None => 0.,
                },
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
            // stoch: CompactIndicator {
            //     current_a: *doc.indicators.stoch.get_data_a().last().unwrap(),
            //     current_b: *doc.indicators.stoch.get_data_b().last().unwrap(),
            //     prev_a: *doc.indicators.stoch.get_data_a().get(second_last).unwrap(),
            //     prev_b: *doc.indicators.stoch.get_data_b().get(second_last).unwrap(),
            //     current_c: 0.,
            //     prev_c: 0.,
            //     status: Status::Default,
            // },
            atr: CompactIndicator {
                current_a: match doc.indicators.atr.as_ref() {
                    Some(atr) => *atr.get_data_a().last().unwrap(),
                    None => 0.,
                },
                prev_a: match doc.indicators.atr.as_ref() {
                    Some(atr) => *atr.get_data_a().get(second_last).unwrap(),
                    None => 0.,
                },
                current_b: 0.,
                prev_b: 0.,
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
            // adx: CompactIndicator {
            //     current_a: 0.,
            //     prev_a: 0.,
            //     //current_a: *doc.indicators.adx.get_data_a().last().unwrap(),
            //     //prev_a: *doc.indicators.adx.get_data_a().get(second_last).unwrap(),
            //     current_b: 0.,
            //     prev_b: 0.,
            //     current_c: 0.,
            //     prev_c: 0.,
            //     status: Status::Default,
            // },
            bb: CompactIndicator {
                current_a: match doc.indicators.bb.as_ref() {
                    Some(bb) => *bb.get_data_a().last().unwrap(),
                    None => 0.,
                },
                current_b: match doc.indicators.bb.as_ref() {
                    Some(bb) => *bb.get_data_b().last().unwrap(),
                    None => 0.,
                },
                current_c: match doc.indicators.bb.as_ref() {
                    Some(bb) => *bb.get_data_c().last().unwrap(),
                    None => 0.,
                },
                prev_a: match doc.indicators.bb.as_ref() {
                    Some(bb) => *bb.get_data_a().get(second_last).unwrap(),
                    None => 0.,
                },
                prev_b: match doc.indicators.bb.as_ref() {
                    Some(bb) => *bb.get_data_b().get(second_last).unwrap(),
                    None => 0.,
                },
                prev_c: match doc.indicators.bb.as_ref() {
                    Some(bb) => *bb.get_data_c().get(second_last).unwrap(),
                    None => 0.,
                },
                status: Status::Default,
            },
            bbw: CompactIndicator {
                current_a: match doc.indicators.bbw.as_ref() {
                    Some(bbw) => *bbw.get_data_a().last().unwrap(),
                    None => 0.,
                },
                current_b: 0.,
                prev_a: match doc.indicators.bbw.as_ref() {
                    Some(bbw) => *bbw.get_data_a().get(second_last).unwrap(),
                    None => 0.,
                },
                prev_b: 0.,
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
            rsi: CompactIndicator {
                current_a: match doc.indicators.rsi.as_ref() {
                    Some(rsi) => *rsi.get_data_a().last().unwrap(),
                    None => 0.,
                },
                current_b: 0.,
                prev_a: match doc.indicators.rsi.as_ref() {
                    Some(rsi) => *rsi.get_data_a().get(second_last).unwrap(),
                    None => 0.,
                },
                prev_b: 0.,
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
            ema_a: CompactIndicator {
                current_a: match doc.indicators.ema_a.as_ref() {
                    Some(ema) => *ema.get_data_a().last().unwrap(),
                    None => 0.,
                },
                current_b: 0.,
                prev_a: match doc.indicators.ema_a.as_ref() {
                    Some(ema) => *ema.get_data_a().get(second_last).unwrap(),
                    None => 0.,
                },
                prev_b: 0.,
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
            ema_b: CompactIndicator {
                current_a: match doc.indicators.ema_b.as_ref() {
                    Some(ema) => *ema.get_data_a().last().unwrap(),
                    None => 0.,
                },
                current_b: 0.,
                prev_a: match doc.indicators.ema_b.as_ref() {
                    Some(ema) => *ema.get_data_a().get(second_last).unwrap(),
                    None => 0.,
                },
                prev_b: 0.,
                current_c: 0.,
                prev_c: 0.,
                status: Status::Default,
            },
            ema_c: CompactIndicator {
                current_a: match doc.indicators.ema_c.as_ref() {
                    Some(ema) => *ema.get_data_a().last().unwrap(),
                    None => 0.,
                },
                current_b: 0.,
                prev_a: match doc.indicators.ema_c.as_ref() {
                    Some(ema) => *ema.get_data_a().get(second_last).unwrap(),
                    None => 0.,
                },
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
