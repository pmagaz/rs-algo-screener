pub mod ema;
pub mod macd;
pub mod rsi;
pub mod stoch;
pub mod tema;

use crate::error::Result;
use crate::indicators::ema::Ema;
use crate::indicators::macd::Macd;
use crate::indicators::rsi::Rsi;
use crate::indicators::stoch::Stoch;
use crate::indicators::tema::Tema;

use rs_algo_shared::models::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::marker::Sized;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Indicators {
    pub macd: Macd,
    pub stoch: Stoch,
    pub rsi: Rsi,
    pub ema_a: Ema,
    pub ema_b: Ema,
    pub ema_c: Ema,
    pub tema_a: Tema,
    pub tema_b: Tema,
    pub tema_c: Tema,
}

impl Indicators {
    pub fn new() -> Result<Self> {
        let ema_a = &env::var("EMA_A").unwrap().parse::<usize>().unwrap();
        let ema_b = &env::var("EMA_B").unwrap().parse::<usize>().unwrap();
        let ema_c = &env::var("EMA_C").unwrap().parse::<usize>().unwrap();
        let tema_a = &env::var("TEMA_A").unwrap().parse::<usize>().unwrap();
        let tema_b = &env::var("TEMA_B").unwrap().parse::<usize>().unwrap();
        let tema_c = &env::var("TEMA_C").unwrap().parse::<usize>().unwrap();

        Ok(Self {
            macd: Macd::new().unwrap(),
            stoch: Stoch::new().unwrap(),
            rsi: Rsi::new().unwrap(),
            ema_a: Ema::new_ema(*ema_a).unwrap(),
            ema_b: Ema::new_ema(*ema_b).unwrap(),
            ema_c: Ema::new_ema(*ema_c).unwrap(),
            tema_a: Tema::new_tema(*tema_a).unwrap(),
            tema_b: Tema::new_tema(*tema_b).unwrap(),
            tema_c: Tema::new_tema(*tema_c).unwrap(),
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

    pub fn calculate_indicators(&mut self, close: f64) -> Result<()> {
        self.macd.next(close).unwrap();
        self.stoch.next(close).unwrap();
        self.rsi.next(close).unwrap();
        self.ema_a.next(close).unwrap();
        self.ema_b.next(close).unwrap();
        self.ema_c.next(close).unwrap();
        self.tema_a.next(close).unwrap();
        self.tema_b.next(close).unwrap();
        self.tema_c.next(close).unwrap();
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
    //fn get_status(&self, current_price: f64) -> Status;
}
