pub mod ema;
pub mod macd;
pub mod rsi;
pub mod stoch;

use crate::error::Result;
use crate::indicators::ema::Ema;
use crate::indicators::macd::Macd;
use crate::indicators::rsi::Rsi;
use crate::indicators::stoch::Stoch;

use std::marker::Sized;

#[derive(Debug, Clone)]
pub enum IndicatorType {
    MacD,
    Stoch,
    Rsi,
    Ema200,
    Ema100,
    Ema50,
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
    pub ema200: Ema,
    pub ema100: Ema,
    pub ema50: Ema,
}

impl Indicators {
    pub fn new() -> Result<Self> {
        Ok(Self {
            macd: Macd::new().unwrap(),
            stoch: Stoch::new().unwrap(),
            rsi: Rsi::new().unwrap(),
            ema200: Ema::new_ema(200).unwrap(),
            ema100: Ema::new_ema(100).unwrap(),
            ema50: Ema::new_ema(50).unwrap(),
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

    pub fn ema200(&self) -> &Ema {
        &self.ema200
    }

    pub fn ema100(&self) -> &Ema {
        &self.ema100
    }

    pub fn ema50(&self) -> &Ema {
        &self.ema50
    }

    pub fn calculate_indicators(&mut self, close: f64) -> Result<()> {
        self.macd.next(close).unwrap();
        self.stoch.next(close).unwrap();
        self.rsi.next(close).unwrap();
        self.ema200.next(close).unwrap();
        self.ema100.next(close).unwrap();
        self.ema50.next(close).unwrap();
        Ok(())
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
    fn get_status(&self, current_price: f64) -> IndicatorStatus;
}
