use super::Indicator;
use crate::error::Result;

use rs_algo_shared::models::*;
use serde::{Deserialize, Serialize};
use ta::indicators::ExponentialMovingAverage;
use ta::Next;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ema {
    ema: ExponentialMovingAverage,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
}

impl Ema {
    pub fn new_ema(index: usize) -> Result<Self> {
        Ok(Self {
            ema: ExponentialMovingAverage::new(index).unwrap(),
            data_a: vec![],
            data_b: vec![],
        })
    }
}

impl Indicator for Ema {
    fn new() -> Result<Self> {
        Ok(Self {
            ema: ExponentialMovingAverage::new(0).unwrap(),
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

    fn get_status(&self, current_price: f64) -> IndicatorStatus {
        let a = self.get_current_a();
        let status = match a {
            _x if a > &current_price => IndicatorStatus::Bullish,
            _x if a < &current_price => IndicatorStatus::Bearish,
            _ => IndicatorStatus::Default,
        };
        status
    }

    fn next(&mut self, value: f64) -> Result<()> {
        let a = self.ema.next(value);
        self.data_a.push(a);
        Ok(())
    }
}
