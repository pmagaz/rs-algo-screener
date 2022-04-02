use async_trait::async_trait;
use bson::{doc, Document};
use chrono::Duration;
use futures::stream::StreamExt;
use mongodb::Cursor;

use round::round;
use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::*;
use std::env;

pub struct General {
    pub query: Document,
    //pub format: fn(CompactInstrument) -> CompactInstrument,
}
//FIMXE impl trait (fix asyn-trait)
impl General {
    pub fn new() -> Result<General> {
        let minimum_pattern_target = env::var("STOCH_BOTTOM").unwrap().parse::<f64>().unwrap();

        let min_horizontal_level_ocurrences = env::var("MIN_HORIZONTAL_LEVELS_OCCURENCES")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        //  {"$and": [{
        Ok(Self {
            query: doc! {
            "$or": [
                {"$or": [
                    {"$and": [
                        {"patterns.local_patterns": {"$elemMatch" : {
                        "date": { "$gte" : DbDateTime::from_chrono(Local::now() - Duration::days(50)) },
                        }}}
                    ]},
                    {"$and": [
                        {"patterns.local_patterns": {"$elemMatch" : {
                        "active.target":{"$gte": minimum_pattern_target },
                        "active.date": { "$gte" : DbDateTime::from_chrono(Local::now() - Duration::days(5)) }
                        }}}
                    ]},
                    ]
                },
                {"$and": [
                    {"$expr": {"$gte": ["$indicators.ema_a.current_a","$indicators.ema_b.current_a"]}},
                    {"$expr": {"$gte": ["$indicators.ema_b.current_a","$indicators.ema_c.current_a"]}},
               ]},
                { "symbol": { "$in": [ "BITCOIN","ETHEREUM","RIPPLE","DOGECOIN","POLKADOT","STELLAR","CARDANO","SOLANA"] } },
                { "current_candle": { "$in": ["Karakasa","BullishGap","MorningStar"] } },
                {"$and": [
                 {
                    "horizontal_levels.lows": {"$elemMatch" : {
                   // "price":{"$gte": "$current_price" },
                    "occurrences":{"$gte": min_horizontal_level_ocurrences },
                }}},
            ]},
            ]},
            //  format: |ins: CompactInstrument| ins,
        })
    }

    pub async fn format_instrument(
        &self,
        mut instruments: Cursor<CompactInstrument>,
    ) -> Vec<CompactInstrument> {
        println!("[STRATEGY] Formating ");
        let mut docs: Vec<CompactInstrument> = vec![];
        let ema_crossover_th = env::var("EMA_CROSSOVER_THRESHOLD")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        while let Some(result) = instruments.next().await {
            match result {
                Ok(mut instrument) => {
                    //MOVE THIS TO SHARED
                    let stoch = instrument.indicators.stoch.clone();
                    let macd = instrument.indicators.stoch.clone();
                    let rsi = instrument.indicators.rsi.clone();
                    let ema_a = instrument.indicators.ema_a.clone(); //9
                    let ema_b = instrument.indicators.ema_b.clone(); //21
                    let ema_c = instrument.indicators.ema_c.clone(); //55

                    let pattern_status = match instrument.patterns.local_patterns.get(0) {
                        Some(val) => {
                            if val.active.date
                                > DbDateTime::from_chrono(Local::now() - Duration::days(5))
                            {
                                let inst = instrument.patterns.local_patterns.get(0);
                                match inst {
                                    Some(_val) => {
                                        instrument.patterns.local_patterns[0].active.status =
                                            Status::Bullish
                                    }
                                    None => {
                                        instrument.patterns.local_patterns[0].active.status =
                                            Status::Default
                                    }
                                }
                            }
                            instrument.patterns.local_patterns[0].active.status.clone()
                        }
                        None => Status::Default,
                    };

                    let stoch_status = match stoch {
                        _x if stoch.current_a > stoch.current_b
                            && stoch.current_a > 20.
                            && stoch.current_a < 30. =>
                        {
                            Status::Bullish
                        }
                        _x if stoch.current_a < stoch.current_b => Status::Bearish,
                        _x if stoch.current_a > 70. => Status::Bearish,
                        _x if stoch.current_a > stoch.current_b && stoch.current_a > 40. => {
                            Status::Neutral
                        }
                        _ => Status::Neutral,
                    };

                    let macd_status = match macd {
                        _x if round(macd.current_a, 2) >= round(macd.current_b, 2)
                            && macd.current_a > 0. =>
                        {
                            Status::Bullish
                        }
                        _x if round(macd.current_a, 2) >= round(macd.current_b, 2)
                            && round(macd.current_a, 2) < 0. =>
                        {
                            Status::Bearish
                        }
                        _x if round(macd.current_a, 1) == round(macd.current_b, 1)
                            && round(macd.current_a, 1) == 0. =>
                        {
                            Status::Neutral
                        }
                        _x if macd.current_a < macd.current_b => Status::Bearish,
                        _ => Status::Neutral,
                    };

                    let rsi_status = match rsi {
                        _x if rsi.current_a < 30. => Status::Bullish,
                        _x if rsi.current_a > 60. => Status::Bearish,
                        _x if rsi.current_a > 40. && rsi.current_a < 60. => Status::Neutral,
                        _ => Status::Neutral,
                    };

                    let ema_status = match ema_a {
                        _x if round(ema_a.current_a, 2) > round(ema_b.current_a, 2)
                            && round(ema_b.current_a, 2) > round(ema_c.current_a, 2) =>
                        {
                            Status::Bullish
                        }
                        _x if percentage_change(ema_a.prev_a, ema_b.prev_a) < ema_crossover_th
                            && round(ema_b.current_a, 1) > round(ema_c.current_a, 1) =>
                        {
                            Status::Neutral
                        }
                        _x if round(ema_a.current_a, 1) == round(ema_b.current_a, 1)
                            && round(ema_b.current_a, 1) == round(ema_c.current_a, 1) =>
                        {
                            Status::Neutral
                        }
                        _x if round(ema_a.current_a, 2) < round(ema_b.current_a, 2)
                            && round(ema_b.current_a, 2) < round(ema_c.current_a, 2) =>
                        {
                            Status::Bearish
                        }
                        _ => Status::Neutral,
                    };

                    instrument.indicators.stoch.status = stoch_status.clone();
                    instrument.indicators.macd.status = macd_status.clone();
                    instrument.indicators.rsi.status = rsi_status.clone();
                    instrument.indicators.ema_a.status = ema_status.clone();

                    if pattern_status != Status::Default
                        || (ema_status != Status::Bearish
                            && (
                                percentage_change(instrument.indicators.ema_a.prev_a, ema_b.prev_a)
                                    < ema_crossover_th
                                //  && percentage_change(ema_b.current_a, ema_c.prev_a) < 0.3
                                //&& round(ema_b.current_a, 2) < round(ema_c.current_a, 2))
                            ))
                    {
                        docs.push(instrument);
                    }
                    //}
                }
                _ => {}
            }
        }
        docs
    }

    pub fn query(&self) -> &Document {
        println!("[STRATEGY] General ");
        &self.query
    }
}
