use super::Strategy;
use async_trait::async_trait;
use bson::{doc, Document};
use chrono::Duration;
use futures::stream::StreamExt;
use mongodb::Cursor;

use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::date::Local;
use rs_algo_shared::models::*;
use std::env;

pub struct Stoch {
    pub query: Document,
    //pub format: fn(CompactInstrument) -> CompactInstrument,
}
//FIMXE impl trait (fix asyn-trait)
impl Stoch {
    pub fn new() -> Result<Stoch> {
        let STOCH_BOTTOM = env::var("STOCH_BOTTOM").unwrap().parse::<f64>().unwrap();
        let MINIMUM_PATTERN_TARGET = env::var("STOCH_BOTTOM").unwrap().parse::<f64>().unwrap();

        Ok(Self {
            query: doc! {
             "$or": [
                {"$and": [
                 {"current_candle": "Karakasa"},
                 {"indicators.stoch.current_a":  {"$lte": STOCH_BOTTOM }}
               ]},
                {"$and": [
                 {"current_candle": "BullishGap"},
                 {"indicators.stoch.current_a":  {"$lt": STOCH_BOTTOM }}
               ]},
              {
               "$and": [
                 {"indicators.stoch.current_a":  {"$lte": STOCH_BOTTOM }},
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
                 "active.target":{"$gte": MINIMUM_PATTERN_TARGET },
                 "direction":"Top",
                 "active.date": { "$gte" : DbDateTime::from_chrono(Local::now() - Duration::days(5)) }
                }}},
                { "symbol": { "$in": [ "BITCOIN","ETHEREUM","RIPPLE","DOGECOIN","POLKADOT","STELLAR"] } }
                //{"$expr": {"$gt": ["$indicators.macd.current_a","$indicators.macd.current_b"]}},
                // {"indicators.macd.current_a":  {"$gt": 0 }},
                // {"$expr": {"$gt": ["$indicators.stoch.current_a","$indicators.stoch.prev_a"]}},
                // {"$expr": {"$lte": ["$indicators.stoch.prev_a","$indicators.stoch.prev_b"]}}
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

        while let Some(result) = instruments.next().await {
            match result {
                Ok(mut instrument) => {
                    //MOVE THIS TO SHARED
                    let stoch = instrument.indicators.stoch.clone();
                    let macd = instrument.indicators.stoch.clone();
                    let rsi = instrument.indicators.rsi.clone();
                    let ema_a = instrument.indicators.ema_a.clone(); //50
                    let ema_b = instrument.indicators.ema_b.clone(); //21
                    let ema_c = instrument.indicators.ema_c.clone(); //9

                    let _local_pattern = match instrument.patterns.local_patterns.get(0) {
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
                        }
                        None => {}
                    };

                    instrument.indicators.stoch.status = match stoch {
                        _x if stoch.current_a > stoch.current_b
                            && stoch.current_a > 20.
                            && stoch.current_a < 30. =>
                        {
                            Status::Bullish
                        }
                        _x if stoch.current_a < stoch.current_b => Status::Bearish,
                        _x if stoch.current_a > stoch.current_b && stoch.current_a > 40. => {
                            Status::Neutral
                        }
                        _ => Status::Neutral,
                    };

                    instrument.indicators.macd.status = match macd {
                        _x if macd.current_a > macd.current_b && macd.current_a > 0. => {
                            Status::Bullish
                        }
                        _x if macd.current_a > macd.current_b && macd.current_a < 0. => {
                            Status::Neutral
                        }
                        _x if macd.current_a < macd.current_b => Status::Bearish,
                        _ => Status::Neutral,
                    };

                    instrument.indicators.rsi.status = match rsi {
                        _x if rsi.current_a < 30. => Status::Bullish,
                        _x if rsi.current_a > 60. => Status::Bearish,
                        _x if rsi.current_a > 40. && rsi.current_a < 60. => Status::Neutral,
                        _ => Status::Neutral,
                    };

                    docs.push(instrument);
                }
                _ => {}
            }
        }
        docs
    }

    pub fn query(&self) -> &Document {
        println!("[STRATEGY] Stoch ");
        &self.query
    }
}
