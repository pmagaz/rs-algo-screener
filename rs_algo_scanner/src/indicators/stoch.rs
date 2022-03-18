use super::Indicator;
use super::IndicatorStatus;
use crate::error::Result;

use ta::indicators::SlowStochastic;

use serde::{Deserialize, Serialize};
use ta::indicators::ExponentialMovingAverage;
use ta::Next;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Stoch {
    stoch: SlowStochastic,
    ema: ExponentialMovingAverage,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
}

impl Indicator for Stoch {
    fn new() -> Result<Self> {
        Ok(Self {
            stoch: SlowStochastic::new(10, 3).unwrap(),
            ema: ExponentialMovingAverage::new(3).unwrap(),
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
            _x if a > b && a > &70. && b > &70. => IndicatorStatus::Overbought,
            _x if a < b && a < &20. && b < &20. => IndicatorStatus::Oversold,
            _ => IndicatorStatus::Default,
        };
        status
    }

    fn next(&mut self, value: f64) -> Result<()> {
        let a = self.stoch.next(value);
        let b = self.ema.next(a);
        self.data_a.push(a);
        self.data_b.push(b);
        Ok(())
    }
}
