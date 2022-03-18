use super::Indicator;
use crate::error::Result;

use rs_algo_shared::models::*;
use serde::{Deserialize, Serialize};
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
        Ok(Self {
            ema12: ExponentialMovingAverage::new(12).unwrap(),
            ema26: ExponentialMovingAverage::new(26).unwrap(),
            ema9: ExponentialMovingAverage::new(9).unwrap(),
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

    fn get_status(&self, _current_price: f64) -> IndicatorStatus {
        let a = self.get_current_a();
        let b = self.get_current_b();
        let status = match (a, b) {
            _x if a > b => IndicatorStatus::Bullish,
            _x if a < b => IndicatorStatus::Bearish,
            _x if a > b && a > &0. && b > &0. => IndicatorStatus::BullishOverZero,
            _x if a < b && a < &0. && b < &0. => IndicatorStatus::BearishBellowZero,
            _ => IndicatorStatus::Default,
        };
        status
    }

    fn next(&mut self, value: f64) -> Result<()> {
        let a = self.ema12.next(value) - self.ema26.next(value);
        let b = self.ema9.next(a);
        self.data_a.push(a);
        self.data_b.push(b);
        Ok(())
    }
}
