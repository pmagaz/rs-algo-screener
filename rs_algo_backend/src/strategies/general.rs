use rs_algo_shared::helpers::status::*;
use rs_algo_shared::models::status::Status;

use rs_algo_shared::scanner::divergence::DivergenceType;
use rs_algo_shared::scanner::instrument::*;
use rs_algo_shared::scanner::pattern::*;

use bson::{doc, Document};
use chrono::Duration;
use futures::stream::StreamExt;
use mongodb::Cursor;
use round::round;
use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::date::*;
use std::cmp::Ordering;
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
        "$and": [
            {"$expr": {"$gte": ["$avg_volume",min_volume,]}},
            {"$or": [
                   {"$and": [

                        {"current_candle": { "$in": ["Reversal","ThreeInRow","Karakasa","Engulfing","BullishGap"] }},
                        //{"$expr": {"$lte": ["$indicators.rsi.current_a", 60]}},
                        {"symbol": {"$regex" : ".*.US"}},
                   ]},
                    {"$and": [
                        {"current_candle": { "$in": ["BearishKarakasa"] }},
                        {"$expr": {"$gte": ["$indicators.rsi.current_a", 67]}},
                        {"symbol": {"$regex" : ".*.US"}},
                   ]},
                {"$or": [
                    {"$and": [
                        {"$expr": {"$ne": [{ "$last": "$patterns.local_patterns.pattern_type" }, "HigherHighsHigherLows"] }},
                        {"$expr": {"$ne": [{ "$last": "$patterns.local_patterns.pattern_type" }, "LowerHighsLowerLows"] }},
                        {"patterns.local_patterns": {"$elemMatch" : {
                            "active.active": false ,
                            "date": { "$gte" : self.max_pattern_date },
                            "$or": [
                                {"target": { "$gte" : minimum_pattern_target }},
                                {"pattern_type": { "$in": ["DoubleTop","DoubleBottom","HeadAndShoulders"] }},
                            ],
                        //"pattern_type": { "$in": ["ChannelUp","TriangleUp","TriangleDown","TriangleSym","Rectangle","BroadeningUp","DoubleBottom","HeadShoulders"] },
                        }}},
                        {"symbol": {"$regex" : ".*.US"}},
                        // {"symbol": {"$regex" : ".*.ES.*"}},
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
                        {"$expr": {"$ne": [{ "$last": "$patterns.local_patterns.pattern_type" }, "HigherHighsHigherLows"] }},
                        {"$expr": {"$ne": [{ "$last": "$patterns.local_patterns.pattern_type" }, "LowerHighsLowerLows"] }},
                        {"patterns.local_patterns": {"$elemMatch" : {
                            "active.active": true ,
                            "active.date": { "$gte" : self.max_activated_date },
                            "$or": [
                                {"active.target": { "$gte" : minimum_pattern_target }},
                                {"pattern_type": { "$in": ["DoubleTop","DoubleBottom","HeadAndShoulders"] }},
                            ],
                        }}},
                        {"symbol": {"$regex" : ".*.US"}},
                        // {"symbol": {"$regex" : ".*.ES"}},
                    ]},
                    ]
                },
                // {"$and": [
                //     {"$expr": {"$ne": [{ "$last": "$patterns.local_patterns.pattern_type" }, "HigherHighsHigherLows"] }},
                //     {"$expr": {"$ne": [{ "$last": "$patterns.local_patterns.pattern_type" }, "LowerHighsLowerLows"] }},
                //     {"symbol": {"$regex" : ".*.US"}},
                //     {"$or": [
                //         {"$and": [
                //             {"$expr": {"$lte": ["$current_price","$indicators.bb.current_b"]}},
                //             {"$expr": {"$lte": ["$prev_price","$indicators.bb.current_c"]}},
                //             {"$expr": {"$gte": ["$prev_price","$indicators.bb.prev_b"]}},
                //         ]},
                //     ]},
                //     // {"$expr": {"$gte": ["$indicators.rsi.current_a", 30]}},
                //     // {"$expr": {"$lte": ["$indicators.rsi.current_a", 40]}},
                //     // {"$and": [
                //     //     {"$expr": {"$ne": [{ "$last": "$patterns.local_patterns.pattern_type" }, "LowerHighsLowerLows"] }},
                //     //     {"$expr": {"$ne": [{ "$last": "$patterns.local_patterns.pattern_type" }, "HigherHighsHigherLows"] }},
                //     // ]},
                // ]},
                // {"$and": [
                //     {"$expr": {"$gte": [{ "$last": "$divergences.data.date" }, self.max_pattern_date ] }},
                //     {"$expr": {"$in": [{ "$last": "$divergences.data.divergence_type" }, ["Bullish", "Bearish"]] }},
                // ]},
                { "symbol": { "$in": [ "BITCOIN","ETHEREUM","RIPPLE","DOGECOIN","CARDANO","BINANCECOIN","SOLANA","STELLAR","POLKADOT"] } },
                { "symbol": { "$in": [ "US500","US100","GOLD","OIL","SILVER"] } },
                { "symbol": { "$in": [ "AUDCAD","AUDCHF","EURUSD","AUDNZD","AUDUSD","CADCHF","CADJPY","CHFJPY","EURAUD","EURCAD","AUDNZD","EURGBP","EURUSD","GBPUSD","USDCHF","USDJPY","NZDUSD"] } },
            ]}]}
    }

    pub async fn format_instrument(
        &self,
        mut instruments: Cursor<CompactInstrument>,
    ) -> Vec<CompactInstrument> {
        log::info!("[STRATEGY] Formating ");
        let mut docs: Vec<CompactInstrument> = vec![];

        while let Some(result) = instruments.next().await {
            match result {
                Ok(mut instrument) => {
                    //let stoch = instrument.indicators.stoch.clone();
                    let macd = instrument.indicators.macd.clone();
                    let rsi = instrument.indicators.rsi.clone();
                    let bb = instrument.indicators.bb.clone();

                    let len = instrument.patterns.local_patterns.len();
                    let last_pattern = instrument.patterns.local_patterns.last();

                    let last_divergence = instrument.divergences.data.last();

                    let _last_pattern_target = match last_pattern {
                        Some(val) => round(val.target, 0),
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

                    let _last_divergence_type = match last_divergence {
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
                        get_pattern_status(last_pattern, second_last_pattern_type, max_days + 20);
                    //let second_last_pattern_status = get_pattern_status(second_last_pattern);
                    if last_pattern_status != Status::Default {
                        let len = instrument.patterns.local_patterns.len();

                        instrument.patterns.local_patterns[len - 1].active.status =
                            last_pattern_status.clone();
                    }

                    //instrument.indicators.stoch.status = get_stoch_status(&stoch);
                    instrument.indicators.macd.status = get_macd_status(&macd);
                    instrument.indicators.rsi.status = get_rsi_status(&rsi);
                    instrument.indicators.bb.status = get_bb_status(&bb, &instrument);

                    //    if ((instrument.current_candle != CandleType::BearishKarakasa ||  (instrument.current_candle == CandleType::BearishKarakasa && instrument.indicators.rsi.current_a > 67.)))
                    //    && (instrument.current_candle != CandleType::Karakasa ||  (instrument.current_candle == CandleType::Karakasa && instrument.indicators.rsi.current_a < 50.))
                    //    {

                    docs.push(instrument);

                    //}
                }
                _ => {}
            }
        }
        // docs.sort_by(|a, b| {
        //     // let a_last_pattern_target = match a.patterns.local_patterns.last() {
        //     //     Some(val) => match val.active.active {
        //     //         true => round(val.active.target, 0),
        //     //         false => round(val.target, 0),
        //     //     },
        //     //     None => 0.,
        //     // };

        //     // let b_last_pattern_target = match b.patterns.local_patterns.last() {
        //     //     Some(val) => match val.active.active {
        //     //         true => round(val.active.target, 0),
        //     //         false => round(val.target, 0),
        //     //     },
        //     //     None => 0.,
        //     // };

        //     a.symbol
        //     // b_last_pattern_target
        //     //     .partial_cmp(&a_last_pattern_target)
        //     //     .unwrap()

        //     // let a_band = percentage_change(a.indicators.bb.current_b, a.indicators.bb.current_a);
        //     // let b_band = percentage_change(b.indicators.bb.current_b, b.indicators.bb.current_a);
        //     // b_band.partial_cmp(&a_band).unwrap()
        // });

        docs.sort_by(|a, b| a.symbol.cmp(&b.symbol));

        // .iter()
        // .filter(x.current_candle != CandleType::BearishKarakasa
        //     || (x.current_candle == CandleType::BearishKarakasa && x.indicators.rsi.current_a > 65.)
        //     || x.current_candle == CandleType::Engulfing || x.current_candle == CandleType::BullishGap)
        // .map(|x| x.clone())
        // .collect();
        docs
    }
}
