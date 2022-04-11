use crate::error::Result;
use crate::models::app_state::AppState;
use actix_web::web;
use mongodb::Collection;
use rs_algo_shared::models::*;

pub async fn get_collection<T>(state: &web::Data<AppState>, collection: &str) -> Collection<T> {
    state
        .db
        .database(&state.db_name)
        .collection::<T>(collection)
}

pub fn compact_instrument(mut doc: Instrument) -> Result<CompactInstrument> {
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
                prev_a: *doc.indicators.macd.data_a.get(len - 2).unwrap(),
                prev_b: *doc.indicators.macd.data_b.get(len - 2).unwrap(),
                status: Status::Default,
            },
            rsi: CompactIndicator {
                current_a: *doc.indicators.rsi.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.rsi.data_a.get(len - 2).unwrap(),
                prev_b: 0.,
                status: Status::Default,
            },
            stoch: CompactIndicator {
                current_a: *doc.indicators.stoch.data_a.last().unwrap(),
                current_b: *doc.indicators.stoch.data_b.last().unwrap(),
                prev_a: *doc.indicators.stoch.data_a.get(len - 2).unwrap(),
                prev_b: *doc.indicators.stoch.data_b.get(len - 2).unwrap(),
                status: Status::Default,
            },
            ema_a: CompactIndicator {
                current_a: *doc.indicators.ema_a.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.ema_a.data_a.get(len - 2).unwrap(),
                prev_b: 0.,
                status: Status::Default,
            },
            ema_b: CompactIndicator {
                current_a: *doc.indicators.ema_b.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.ema_b.data_a.get(len - 2).unwrap(),
                prev_b: 0.,
                status: Status::Default,
            },
            ema_c: CompactIndicator {
                current_a: *doc.indicators.ema_c.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.ema_c.data_a.get(len - 2).unwrap(),
                prev_b: 0.,
                status: Status::Default,
            },
            tema_a: CompactIndicator {
                current_a: *doc.indicators.tema_a.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.tema_a.data_a.get(len - 2).unwrap(),
                prev_b: 0.,
                status: Status::Default,
            },
            tema_b: CompactIndicator {
                current_a: *doc.indicators.tema_a.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.tema_a.data_a.get(len - 2).unwrap(),
                prev_b: 0.,
                status: Status::Default,
            },
            tema_c: CompactIndicator {
                current_a: *doc.indicators.tema_a.data_a.last().unwrap(),
                current_b: 0.,
                prev_a: *doc.indicators.tema_a.data_a.get(len - 2).unwrap(),
                prev_b: 0.,
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
        //divergences: doc.divergences,
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
