pub mod atr;
pub mod adx;
pub mod bb;
pub mod bbw;
pub mod ema;
pub mod macd;
pub mod rsi;
pub mod sd;
pub mod stoch;
pub mod tema;

use crate::error::Result;
use crate::indicators::atr::Atr;
use crate::indicators::adx::Adx;
use crate::indicators::bb::BollingerB;
use crate::indicators::bbw::BollingerBW;
use crate::indicators::ema::Ema;
use crate::indicators::macd::Macd;
use crate::indicators::rsi::Rsi;
use crate::indicators::stoch::Stoch;

use serde::{Deserialize, Serialize};
use std::env;
use std::marker::Sized;

pub trait Indicator {
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn next(&mut self, value: f64) -> Result<()>;
    fn next_OHLC(&mut self, OHLC: (f64, f64, f64, f64)) -> Result<()>;
    fn get_data_a(&self) -> &Vec<f64>;
    fn get_current_a(&self) -> &f64;
    fn get_current_b(&self) -> &f64;
    fn get_data_b(&self) -> &Vec<f64>;
    fn get_current_c(&self) -> &f64;
    fn get_data_c(&self) -> &Vec<f64>;
}

//FIXME ARRAY OF TRAIT INDICATORS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Indicators {
    pub macd: Macd,
    pub stoch: Stoch,
    pub atr: Atr,
    pub adx: Adx,
    pub rsi: Rsi,
    pub bb: BollingerB,
    pub bbw: BollingerBW,
    pub ema_a: Ema,
    pub ema_b: Ema,
    pub ema_c: Ema,
}

impl Indicators {
    pub fn new() -> Result<Self> {
        let ema_a = &env::var("EMA_A").unwrap().parse::<usize>().unwrap();
        let ema_b = &env::var("EMA_B").unwrap().parse::<usize>().unwrap();
        let ema_c = &env::var("EMA_C").unwrap().parse::<usize>().unwrap();

        Ok(Self {
            macd: Macd::new().unwrap(),
            rsi: Rsi::new().unwrap(),
            stoch: Stoch::new().unwrap(),
            atr: Atr::new().unwrap(),
            adx: Adx::new().unwrap(),
            bb: BollingerB::new().unwrap(),
            bbw: BollingerBW::new().unwrap(),
            ema_a: Ema::new_ema(*ema_a).unwrap(),
            ema_b: Ema::new_ema(*ema_b).unwrap(),
            ema_c: Ema::new_ema(*ema_c).unwrap(),
        })
    }

    pub fn atr(&self) -> &Atr {
        &self.atr
    }

    pub fn adx(&self) -> &Adx {
        &self.adx
    }

    pub fn bb(&self) -> &BollingerB {
        &self.bb
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

    pub fn calculate_indicators(&mut self, OHLC: (f64, f64, f64, f64)) -> Result<()> {
        let close = OHLC.3;
        self.macd.next(close).unwrap();
        self.stoch.next(close).unwrap();
        let extended_indicators = env::var("EXTENDED_INDICATORS")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        if extended_indicators {
            self.atr.next_OHLC(OHLC).unwrap();
            //self.adx.next(close).unwrap();
            self.bb.next(close).unwrap();
            self.bbw.next(close).unwrap();
            self.rsi.next(close).unwrap();
            self.ema_a.next(close).unwrap();
            self.ema_b.next(close).unwrap();
            self.ema_c.next(close).unwrap();
        }
        Ok(())
    }
}
