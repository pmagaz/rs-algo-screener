use super::Strategy;
use bson::{doc, Document};
use rs_algo_shared::error::Result;
use rs_algo_shared::models::CompactInstrument;

pub struct Stoch {
    pub query: Document,
    pub filters: fn(Vec<CompactInstrument>) -> Vec<CompactInstrument>,
    //pub filters: dyn Fn(Vec<CompactInstrument>) -> Vec<CompactInstrument>,
}

impl Strategy for Stoch {
    fn new() -> Result<Stoch> {
        Ok(Self {
            query: doc! {
              // "$or": [{
                 "$and": [
                 {"indicators.rsi.current_a":  {"$lte": 30 } },
                 {"indicators.stoch.current_a":  {"$lt": 30 }},
                 {"$expr": {"$eq": ["$indicators.stoch.status","Default"]}},
                 {"$expr": {"$gt": ["$indicators.stoch.current_a","$indicators.stoch.prev_a"]}},
                 {"$expr": {"$lte": ["$indicators.stoch.prev_a","$indicators.stoch.prev_b"]}}
            // ]}
             ]},
            filters: |ins: Vec<CompactInstrument>| ins,
        })
    }

    fn query(&self) -> &Document {
        println!("[STRATEGY] Stoch ");
        &self.query
    }
}
