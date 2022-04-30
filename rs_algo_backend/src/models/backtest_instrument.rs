use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackTestInstrument {
    pub symbol: String,
    pub trades_in: Vec<TradeIn>,
    pub trades_out: Vec<TradeOut>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeIn {
    pub index_in: i32,
    pub price_in: f64,
    pub quantity: i32,
    pub date: DbDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOut {
    pub index_in: i32,
    pub price_in: f64,
    pub quantity: i32,
    pub index_out: i32,
    pub price_out: f64,
    pub diff: f64,
}
