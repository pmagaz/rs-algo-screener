use super::Indicator;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use ta::indicators::MoneyFlowIndex;
use ta::Next;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mfi {
    mfi: MoneyFlowIndex,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
}

impl Indicator for Mfi {
    fn new() -> Result<Self> {
        Ok(Self {
            mfi: MoneyFlowIndex::new(14).unwrap(),
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

    fn next(&mut self, value: f64) -> Result<()> {
        let a = self.mfi.next(value);
        self.data_a.push(a);
        Ok(())
    }
}
