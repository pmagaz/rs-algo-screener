use crate::error::Result;
use crate::models::app_state::AppState;
use crate::models::instrument::{
    CompactDivergence, CompactDivergences, CompactIndicator, CompactIndicators, CompactInstrument,
    Instrument, Patterns,
};
use actix_web::web;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::Collection;

pub async fn get_collection<T>(state: &web::Data<AppState>, collection: &str) -> Collection<T> {
    state
        .db
        .database(&state.db_name)
        .collection::<T>(collection)
}

// pub fn compact_instrument2(doc: &Document) -> Result<CompactInstrument> {
//     let instrument = CompactInstrument {
//         symbol: doc.get_str("symbol").unwrap().to_owned(),
//         time_frame: doc.get_str("time_frame").unwrap().to_owned(),
//         current_price: doc.get_f64("current_price").unwrap(),
//         current_candle: doc.get_str("current_candle").unwrap().to_owned(),
//         //date: doc.get_datetime(key)
//     };

//     Ok(instrument)
// }

pub fn compact_instrument(doc: Instrument) -> Result<CompactInstrument> {
    let len = doc.indicators.macd.data_a.len();
    let doc = CompactInstrument {
        symbol: doc.symbol,
        date: doc.date,
        time_frame: doc.time_frame,
        current_price: doc.current_price,
        current_candle: doc.current_candle,
        indicators: CompactIndicators {
            macd: CompactIndicator {
                current_a: *doc.indicators.macd.data_a.last().unwrap(),
                current_b: *doc.indicators.macd.data_b.last().unwrap(),
                prev_a: *doc.indicators.macd.data_a.get(len - 1).unwrap(),
                prev_b: *doc.indicators.macd.data_b.get(len - 1).unwrap(),
            },
            rsi: CompactIndicator {
                current_a: *doc.indicators.rsi.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.rsi.data_a.get(len - 1).unwrap(),
                prev_b: 0.,
            },
            stoch: CompactIndicator {
                current_a: *doc.indicators.stoch.data_a.last().unwrap(),
                current_b: *doc.indicators.stoch.data_b.last().unwrap(),
                prev_a: *doc.indicators.stoch.data_a.get(len - 1).unwrap(),
                prev_b: *doc.indicators.stoch.data_b.get(len - 1).unwrap(),
            },
            ema_a: CompactIndicator {
                current_a: *doc.indicators.ema_a.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.ema_a.data_a.get(len - 1).unwrap(),
                prev_b: 0.,
            },
            ema_b: CompactIndicator {
                current_a: *doc.indicators.ema_b.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.ema_b.data_a.get(len - 1).unwrap(),
                prev_b: 0.,
            },
            ema_c: CompactIndicator {
                current_a: *doc.indicators.ema_c.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.ema_c.data_a.get(len - 1).unwrap(),
                prev_b: 0.,
            },
            ema_d: CompactIndicator {
                current_a: *doc.indicators.ema_d.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.ema_d.data_a.get(len - 1).unwrap(),
                prev_b: 0.,
            },
            ema_e: CompactIndicator {
                current_a: *doc.indicators.ema_e.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.ema_e.data_a.get(len - 1).unwrap(),
                prev_b: 0.,
            },
        },
        patterns: Patterns {
            local_patterns: doc
                .patterns
                .local_patterns
                .into_iter()
                .enumerate()
                .filter(|(key, _inst)| key < &3)
                .map(|(key, inst)| inst)
                .collect(),
            extrema_patterns: doc
                .patterns
                .extrema_patterns
                .into_iter()
                .enumerate()
                .filter(|(key, _inst)| key < &3)
                .map(|(key, inst)| inst)
                .collect(),
        },
        //divergences: doc.divergences,
        divergences: CompactDivergences {
            divergences: doc
                .divergences
                .divergences
                .into_iter()
                .enumerate()
                .filter(|(key, _inst)| key < &3)
                .map(|(key, inst)| CompactDivergence {
                    indicator: inst.indicator,
                    divergence_type: inst.divergence_type,
                })
                .collect(),
        },
    };
    Ok(doc)
}
