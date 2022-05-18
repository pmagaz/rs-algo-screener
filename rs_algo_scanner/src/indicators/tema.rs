use super::Indicator;
use crate::error::Result;

use rs_algo_shared::models::*;
use serde::{Deserialize, Serialize};
use ta::indicators::ExponentialMovingAverage;
use ta::Next;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tema {
    ema1: ExponentialMovingAverage,
    ema2: ExponentialMovingAverage,
    ema3: ExponentialMovingAverage,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
}

impl Tema {
    pub fn new_tema(index: usize) -> Result<Self> {
        Ok(Self {
            ema1: ExponentialMovingAverage::new(index).unwrap(),
            ema2: ExponentialMovingAverage::new(index).unwrap(),
            ema3: ExponentialMovingAverage::new(index).unwrap(),
            data_a: vec![],
            data_b: vec![],
        })
    }
}

impl Indicator for Tema {
    fn new() -> Result<Self> {
        Ok(Self {
            ema1: ExponentialMovingAverage::new(0).unwrap(),
            ema2: ExponentialMovingAverage::new(0).unwrap(),
            ema3: ExponentialMovingAverage::new(0).unwrap(),
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
        let ema1 = self.ema1.next(value);
        let ema2 = self.ema2.next(ema1);
        let ema3 = self.ema3.next(ema2);
        //TEMA =(3∗EMA1)−(3∗EMA2 )+EMA3
        let tema = (3. * ema1) - (3. * ema2) + ema3;
        self.data_a.push(tema);
        Ok(())
    }
}
