use super::Strategy;
use async_trait::async_trait;
use bson::{doc, Document};
use chrono::Duration;
use futures::stream::StreamExt;
use mongodb::Cursor;

use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::comp::is_equal;
use rs_algo_shared::helpers::date::Local;
use rs_algo_shared::models::*;
use std::env;

pub struct General {
    pub query: Document,
    //pub format: fn(CompactInstrument) -> CompactInstrument,
}
//FIMXE impl trait (fix asyn-trait)
impl General {
    pub fn new() -> Result<General> {
        let stoch_bottom = env::var("STOCH_BOTTOM").unwrap().parse::<f64>().unwrap();
        let minimum_pattern_target = env::var("STOCH_BOTTOM").unwrap().parse::<f64>().unwrap();
        let min_horizontal_level_ocurrences = env::var("MIN_HORIZONTAL_LEVELS_OCCURENCES")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        Ok(Self {
            query: doc! {
             "$or": [
                {"$and": [
                 {"current_candle": "Karakasa"},
                 {"indicators.stoch.current_a":  {"$lt": stoch_bottom }},
               ]},
                {"$and": [
                 {"current_candle": "MorningStar"},
                 {"indicators.stoch.current_a":  {"$lt": stoch_bottom }},
               ]},
                {"$and": [
                 {"current_candle": "BullishGap"},
               ]},
              {
               "$and": [
                 {"indicators.stoch.current_a":  {"$lte": stoch_bottom }},
                 {"$expr": {"$gt": ["$indicators.stoch.current_a","$indicators.stoch.current_b"]}},
                 {"$expr": {"$gt": ["$indicators.stoch.current_a","$indicators.stoch.prev_a"]}},
                 {"$expr": {"$lte": ["$indicators.stoch.prev_a","$indicators.stoch.prev_b"]}}
               ]},
               {
               "$and": [
                 {"$expr": {"$gt": ["$indicators.macd.current_a","$indicators.macd.current_b"]}},
                 {"$expr": {"$gt": ["$indicators.macd.current_a","$indicators.macd.prev_a"]}},
                 {"$expr": {"$lte": ["$indicators.macd.prev_a","$indicators.macd.prev_b"]}}
               ]},
                {"$and": [
                 {"patterns.local_patterns": {"$elemMatch" : {
                    "active.target":{"$gte": minimum_pattern_target },
                    "active.pattern_type":{"$nin": ["None"] },
                    "active.date": { "$gte" : DbDateTime::from_chrono(Local::now() - Duration::days(5)) }
                }}},
            ]},
                {"$and": [
                 {
                    "horizontal_levels.lows": {"$elemMatch" : {
                   // "price":{"$gte": "$current_price" },
                    "occurrences":{"$gte": min_horizontal_level_ocurrences },
                }}},
            ]},
                { "symbol": { "$in": [ "BITCOIN","ETHEREUM","RIPPLE","DOGECOIN","POLKADOT","STELLAR","CARDANO","SOLANA"] } }
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
                        _x if macd.current_a > macd.current_b && macd.current_a > 0. => {
                            Status::Bullish
                        }
                        _x if macd.current_a > macd.current_b && macd.current_a < 0. => {
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
                        _x if ema_a.current_a > ema_b.current_a
                            && ema_b.current_a > ema_c.current_a =>
                        {
                            Status::Bullish
                        }
                        _x if ema_c.current_a < ema_b.current_a
                            && ema_b.current_a < ema_c.current_a =>
                        {
                            Status::Bearish
                        }
                        _ => Status::Neutral,
                    };

                    instrument.indicators.stoch.status = stoch_status.clone();
                    instrument.indicators.macd.status = macd_status.clone();
                    instrument.indicators.rsi.status = rsi_status.clone();

                    //FIXME or ?
                    if instrument.current_candle == CandleType::Karakasa
                        || instrument.current_candle == CandleType::BullishGap
                        || pattern_status != Status::Neutral
                        || (stoch_status != Status::Bearish && macd_status != Status::Bearish)
                    {
                        docs.push(instrument);
                    }
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
