use super::Indicator;
use rs_algo_shared::models::status::Status;

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

    fn next(&mut self, value: f64) -> Result<()> {
        let a = self.stoch.next(value);
        let b = self.ema.next(a);
        self.data_a.push(a);
        self.data_b.push(b);
        Ok(())
    }
}
