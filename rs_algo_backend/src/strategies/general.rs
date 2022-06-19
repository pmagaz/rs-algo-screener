use bson::{doc, Document};
use chrono::Duration;
use futures::stream::StreamExt;
use mongodb::Cursor;
use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::helpers::status::*;
use rs_algo_shared::models::divergence::DivergenceType;
use rs_algo_shared::models::instrument::*;
use rs_algo_shared::models::pattern::*;
use rs_algo_shared::models::status::Status;
use std::cmp::Ordering;

use round::round;
use rs_algo_shared::error::Result;

use rs_algo_shared::helpers::date::*;
use std::env;

pub struct General {
    pub query: Document,
    pub max_pattern_date: DbDateTime,
    pub max_activated_date: DbDateTime,
}

//FIMXE impl trait (fix asyn-trait)
impl General {
    pub fn new() -> Result<General> {
        let max_pattern_days = env::var("MAX_PATTERN_DAYS")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let max_pattern_activated_days = env::var("MAX_PATTERN_ACTIVATED_DAYS")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        Ok(Self {
            query: doc! {},
            max_pattern_date: to_dbtime(Local::now() - Duration::days(max_pattern_days)),
            max_activated_date: to_dbtime(
                Local::now() - Duration::days(max_pattern_activated_days),
            ),
        })
    }

    pub fn query(&self) -> Document {
        let minimum_pattern_target = env::var("MINIMUM_PATTERN_TARGET")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let _min_horizontal_level_ocurrences = env::var("MIN_HORIZONTAL_LEVELS_OCCURENCES")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let min_volume = env::var("MIN_VOLUME").unwrap().parse::<f64>().unwrap();

        doc! {
        "$or": [
            {"$and": [
                {"$and": [
                    {"patterns.local_patterns": {"$elemMatch" : {
                    "date": { "$gte" : self.max_pattern_date },
                    "pattern_type": { "$in": ["ChannelUp","TriangleUp","Rectangle","BroadeningUp","DoubleBottom","HeadShoulders"] },
                    }}}
                ]},
              //  {"or": [

                    // {"$expr": {"$eq": [{ "$last": "$patterns.local_patterns.pattern_type" }, "ChannelUp"] }},
                    // {"$expr": {"$eq": [{ "$last": "$patterns.local_patterns.pattern_type" }, "TriangleUp"] }},
                    // {"$expr": {"$eq": [{ "$last": "$patterns.local_patterns.pattern_type" }, "Rectangle"] }},
                    // {"$expr": {"$eq": [{ "$last": "$patterns.local_patterns.pattern_type" }, "BroadeningUp"] }},
                    // {"$expr": {"$eq": [{ "$last": "$patterns.local_patterns.pattern_type" }, "DoubleBottom"] }},
                    // {"$expr": {"$eq": [{ "$last": "$patterns.local_patterns.pattern_type" }, "HeadShoulders"] }},

              //  ]},
                {"$and": [
                    {"patterns.local_patterns": {"$elemMatch" : {
                    "active.target":{"$gte": minimum_pattern_target },
                    "active.date": { "$gte" : self.max_activated_date },
                    "pattern_type": { "$in": ["ChannelUp","TriangleUp","Rectangle","BroadeningUp","DoubleBottom","HeadShoulders"] },
                    }}}
                ]},
                ]
            },
            {"$and": [
                {"$or": [
                    {"symbol": {"$regex" : ".*.US.*"}},
                   // {"symbol": {"$regex" : ".*.DK.*"}},
                   // {"symbol": {"$regex" : ".*.DE.*"}},
                   // {"symbol": {"$regex" : ".*.ES.*"}},
                   // {"symbol": {"$regex" : ".*.CH.*"}},
                ]},
                {"$expr": {"$gte": ["$avg_volume",min_volume,]}},
                {"$or": [
                    {"$expr": {"$lte": ["$indicators.bbw.current_a", 0.2]}},
                    {"$and": [
                        {"$expr": {"$lte": ["$current_price","$indicators.bb.current_b"]}},
                        {"$expr": {"$gte": ["$prev_price","$indicators.bb.prev_b"]}},
                    ]},
                ]},
                {"$expr": {"$gte": ["$indicators.rsi.current_a", 30]}},
                {"$expr": {"$lte": ["$indicators.rsi.current_a", 40]}},
                {"$and": [
                    {"$expr": {"$ne": [{ "$last": "$patterns.local_patterns.pattern_type" }, "LowerHighsLowerLows"] }},
                    {"$expr": {"$ne": [{ "$last": "$patterns.local_patterns.pattern_type" }, "ChannelDown"] }},
                ]},
            ]},
            { "symbol": { "$in": [ "BITCOIN","ETHEREUM","RIPPLE","DOGECOIN","POLKADOT","STELLAR","CARDANO","SOLANA"] } },
            { "symbol": { "$in": [ "US500","US100","GOLD","OIL","SILVER"] } },
        ]}
    }

    pub async fn format_instrument(
        &self,
        mut instruments: Cursor<CompactInstrument>,
    ) -> Vec<CompactInstrument> {
        println!("[STRATEGY] Formating ");
        let mut docs: Vec<CompactInstrument> = vec![];

        while let Some(result) = instruments.next().await {
            match result {
                Ok(mut instrument) => {
                    let stoch = instrument.indicators.stoch.clone();
                    let macd = instrument.indicators.macd.clone();
                    let rsi = instrument.indicators.rsi.clone();
                    let bb = instrument.indicators.bb.clone();

                    let len = instrument.patterns.local_patterns.len();
                    let last_pattern = instrument.patterns.local_patterns.last();

                    let last_divergence = instrument.divergences.data.last();

                    let _last_pattern_target = match last_pattern {
                        Some(val) => round(val.active.change, 0),
                        None => 0.,
                    };

                    let fake_date = to_dbtime(Local::now() - Duration::days(1000));

                    // let last_pattern_type = match last_divergence {
                    //     Some(val) => &val.pattern_type,
                    //     None => &DivergenceType::None,
                    // };

                    let _last_pattern_date = match last_pattern {
                        Some(val) => val.date,
                        None => fake_date,
                    };

                    let last_divergence_type = match last_divergence {
                        Some(val) => &val.divergence_type,
                        None => &DivergenceType::None,
                    };

                    let second_last_pattern_type = match len.cmp(&2) {
                        Ordering::Less => &PatternType::None,
                        Ordering::Greater => {
                            &instrument
                                .patterns
                                .local_patterns
                                .get(len - 2)
                                .unwrap()
                                .pattern_type
                        }
                        Ordering::Equal => {
                            &instrument
                                .patterns
                                .local_patterns
                                .get(len - 2)
                                .unwrap()
                                .pattern_type
                        }
                    };

                    let max_days = env::var("MAX_PATTERN_DAYS")
                        .unwrap()
                        .parse::<i64>()
                        .unwrap();

                    let last_pattern_status =
                        get_pattern_status(last_pattern, second_last_pattern_type, max_days);
                    //let second_last_pattern_status = get_pattern_status(second_last_pattern);
                    if last_pattern_status != Status::Default {
                        let len = instrument.patterns.local_patterns.len();
                        instrument.patterns.local_patterns[len - 1].active.status =
                            last_pattern_status.clone();
                    }

                    instrument.indicators.stoch.status = get_stoch_status(&stoch);
                    instrument.indicators.macd.status = get_macd_status(&macd);
                    instrument.indicators.rsi.status = get_rsi_status(&rsi);
                    instrument.indicators.bb.status = get_bb_status(&bb, &instrument);
                    docs.push(instrument);
                }
                _ => {}
            }
        }
        docs.sort_by(|a, b| {
            let a_band = percentage_change(a.indicators.bb.current_b, a.indicators.bb.current_a);
            let b_band = percentage_change(b.indicators.bb.current_b, b.indicators.bb.current_a);
            b_band.partial_cmp(&a_band).unwrap()
        });
        docs
    }
}
