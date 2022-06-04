use super::Indicator;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use ta::indicators::StandardDeviation as SD;
use ta::Next;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardD {
    sd: SD,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
}

impl Indicator for StandardD {
    fn new() -> Result<Self> {
        Ok(Self {
            sd: SD::new(5).unwrap(),
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
        &self.data_a
    }

    fn get_current_b(&self) -> &f64 {
        let max = self.data_a.len() - 1;
        &self.data_a[max]
    }

    fn get_data_c(&self) -> &Vec<f64> {
        &self.data_a
    }

    fn get_current_c(&self) -> &f64 {
        let max = self.data_a.len() - 1;
        &self.data_a[max]
    }

    fn next(&mut self, value: f64) -> Result<()> {
        let a = self.sd.next(value);
        self.data_a.push(a);
        Ok(())
    }
    fn next_OHLC(&mut self, OHLC: (f64, f64, f64, f64)) -> Result<()> {
        Ok(())
    }
}
