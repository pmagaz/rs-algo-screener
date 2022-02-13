pub mod macd;
pub mod rsi;
pub mod stoch;

use crate::error::Result;
use crate::indicators::macd::Macd;
use crate::indicators::rsi::Rsi;
use crate::indicators::stoch::Stoch;

use std::marker::Sized;

#[derive(Debug, Clone)]
pub enum IndicatorType {
    MacD,
    Stoch,
    Rsi,
}
#[derive(Debug, Clone)]
pub enum IndicatorStatus {
    Bearish,
    BearishBellowZero,
    Bullish,
    BullishOverZero,
    Oversold,
    Overbought,
    Default,
}

#[derive(Debug, Clone)]
pub struct Indicators {
    pub macd: Macd,
    pub stoch: Stoch,
    pub rsi: Rsi,
}

impl Indicators {
    pub fn new() -> Result<Self> {
        Ok(Self {
            macd: Macd::new().unwrap(),
            stoch: Stoch::new().unwrap(),
            rsi: Rsi::new().unwrap(),
        })
    }

    pub fn macd(&self) -> &Macd {
        &self.macd
    }

    pub fn rsi(&self) -> &Rsi {
        &self.rsi
    }

    pub fn stoch(&self) -> &Stoch {
        &self.stoch
    }

    pub fn calculate_indicators(&mut self, close: f64) {
        &self.macd.next(close).unwrap();
        &self.stoch.next(close).unwrap();
        &self.rsi.next(close).unwrap();
    }
}

pub trait Indicator {
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn next(&mut self, value: f64) -> Result<()>;
    fn get_data_a(&self) -> &Vec<f64>;
    fn get_current_a(&self) -> &f64;
    fn get_current_b(&self) -> &f64;
    fn get_data_b(&self) -> &Vec<f64>;
    fn get_status(&self) -> IndicatorStatus;
}
