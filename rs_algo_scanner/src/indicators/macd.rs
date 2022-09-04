use super::Indicator;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use std::env;
use ta::indicators::ExponentialMovingAverage;
use ta::Next;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macd {
    ema26: ExponentialMovingAverage,
    ema12: ExponentialMovingAverage,
    ema9: ExponentialMovingAverage,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
}

impl Indicator for Macd {
    fn new() -> Result<Self> {
        let macd_a = env::var("MACD_A").unwrap().parse::<usize>().unwrap();
        let macd_b = env::var("MACD_B").unwrap().parse::<usize>().unwrap();
        let macd_c = env::var("MACD_C").unwrap().parse::<usize>().unwrap();

        Ok(Self {
            ema12: ExponentialMovingAverage::new(macd_a).unwrap(),
            ema26: ExponentialMovingAverage::new(macd_b).unwrap(),
            ema9: ExponentialMovingAverage::new(macd_c).unwrap(),
            data_a: vec![],
            data_b: vec![],
        })
    }
    fn get_data_a(&self) -> &Vec<f64> {
        &self.data_a
    }

    fn get_current_a(&self) -> &f64 {
        let max = self.data_a.len() - 1;
        &self.data_a[max]
    }

    fn get_data_b(&self) -> &Vec<f64> {
        &self.data_b
    }

    fn get_current_b(&self) -> &f64 {
        let max = self.data_b.len() - 1;
        &self.data_b[max]
    }

    fn get_data_c(&self) -> &Vec<f64> {
        &self.data_a
    }

    fn get_current_c(&self) -> &f64 {
        let max = self.data_a.len() - 1;
        &self.data_a[max]
    }

    fn next(&mut self, value: f64) -> Result<()> {
        let a = self.ema12.next(value) - self.ema26.next(value);
        let b = self.ema9.next(a);
        self.data_a.push(a);
        self.data_b.push(b);
        Ok(())
    }
    fn next_OHLC(&mut self, OHLC: (f64, f64, f64, f64)) -> Result<()> {
        Ok(())
    }
}
