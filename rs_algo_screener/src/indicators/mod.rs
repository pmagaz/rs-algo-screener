pub mod ema;
pub mod macd;
pub mod rsi;
pub mod stoch;

use crate::error::{Result, RsAlgoError, RsAlgoErrorKind};
use crate::indicators::ema::Ema;
use crate::indicators::macd::Macd;
use crate::indicators::rsi::Rsi;
use crate::indicators::stoch::Stoch;

use serde::{Deserialize, Serialize};
use std::env;
use std::marker::Sized;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndicatorType {
    MacD,
    Stoch,
    Rsi,
    Ema_a,
    Ema_b,
    Ema_c,
    Ema_d,
    Ema_e,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorReq {
    indicator_type: IndicatorType,
    status: IndicatorStatus,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
}

impl IndicatorReq {
    pub fn new() -> IndicatorReqBuilder {
        IndicatorReqBuilder::new()
    }
}

pub struct IndicatorReqBuilder {
    indicator_type: Option<IndicatorType>,
    status: Option<IndicatorStatus>,
    data_a: Option<Vec<f64>>,
    data_b: Option<Vec<f64>>,
}

impl IndicatorReqBuilder {
    pub fn new() -> Self {
        Self {
            indicator_type: None,
            status: None,
            data_a: None,
            data_b: None,
        }
    }

    pub fn build(self) -> Result<IndicatorReq> {
        if let (Some(indicator_type), Some(status), Some(data_a), Some(data_b)) =
            (self.indicator_type, self.status, self.data_a, self.data_b)
        {
            Ok(IndicatorReq {
                indicator_type,
                status,
                data_a,
                data_b,
            })
        } else {
            Err(RsAlgoError {
                err: RsAlgoErrorKind::InvalidCandle,
            })
        }
    }

    pub fn indicator_type(mut self, val: IndicatorType) -> Self {
        self.indicator_type = Some(val);
        self
    }

    pub fn status(mut self, val: IndicatorStatus) -> Self {
        self.status = Some(val);
        self
    }
    pub fn data_a(mut self, val: Vec<f64>) -> Self {
        self.data_a = Some(val);
        self
    }
    pub fn data_b(mut self, val: Vec<f64>) -> Self {
        self.data_b = Some(val);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub ema_a: Ema,
    pub ema_b: Ema,
    pub ema_c: Ema,
    pub ema_d: Ema,
    pub ema_e: Ema,
}

impl Indicators {
    pub fn new() -> Result<Self> {
        let ema_a = &env::var("EMA_A").unwrap().parse::<usize>().unwrap();
        let ema_b = &env::var("EMA_B").unwrap().parse::<usize>().unwrap();
        let ema_c = &env::var("EMA_C").unwrap().parse::<usize>().unwrap();
        let ema_d = &env::var("EMA_D").unwrap().parse::<usize>().unwrap();
        let ema_e = &env::var("EMA_E").unwrap().parse::<usize>().unwrap();

        Ok(Self {
            macd: Macd::new().unwrap(),
            stoch: Stoch::new().unwrap(),
            rsi: Rsi::new().unwrap(),
            ema_a: Ema::new_ema(*ema_a).unwrap(),
            ema_b: Ema::new_ema(*ema_b).unwrap(),
            ema_c: Ema::new_ema(*ema_c).unwrap(),
            ema_d: Ema::new_ema(*ema_d).unwrap(),
            ema_e: Ema::new_ema(*ema_e).unwrap(),
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

    pub fn ema_a(&self) -> &Ema {
        &self.ema_a
    }

    pub fn ema_b(&self) -> &Ema {
        &self.ema_b
    }

    pub fn ema_c(&self) -> &Ema {
        &self.ema_c
    }

    pub fn ema_d(&self) -> &Ema {
        &self.ema_d
    }

    pub fn ema_e(&self) -> &Ema {
        &self.ema_e
    }

    pub fn calculate_indicators(&mut self, close: f64) -> Result<()> {
        self.macd.next(close).unwrap();
        self.stoch.next(close).unwrap();
        self.rsi.next(close).unwrap();
        self.ema_a.next(close).unwrap();
        self.ema_b.next(close).unwrap();
        self.ema_c.next(close).unwrap();
        self.ema_d.next(close).unwrap();
        self.ema_e.next(close).unwrap();
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
