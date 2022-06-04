use super::Indicator;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use ta::indicators::{BollingerBands, BollingerBandsOutput};
use ta::Next;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BollingerB {
    bb: BollingerBands,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
    data_c: Vec<f64>,
}

impl Indicator for BollingerB {
    fn new() -> Result<Self> {
        Ok(Self {
            bb: BollingerBands::new(20, 2.0).unwrap(),
            data_a: vec![],
            data_b: vec![],
            data_c: vec![],
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
        let max = self.data_a.len() - 1;
        &self.data_a[max]
    }

    fn get_data_c(&self) -> &Vec<f64> {
        &self.data_c
    }

    fn get_current_c(&self) -> &f64 {
        let max = self.data_c.len() - 1;
        &self.data_c[max]
    }

    fn next(&mut self, value: f64) -> Result<()> {
        let a = self.bb.next(value);
        self.data_a.push(a.upper);
        self.data_b.push(a.lower);
        self.data_c.push(a.average);
        Ok(())
    }
    fn next_OHLC(&mut self, OHLC: (f64, f64, f64, f64)) -> Result<()> {
        Ok(())
    }
}
