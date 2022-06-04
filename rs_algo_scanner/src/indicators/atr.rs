use super::Indicator;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use ta::data_item::DataItem;
use ta::indicators::AverageTrueRange;
use ta::Next;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Atr {
    atr: AverageTrueRange,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
}

impl Indicator for Atr {
    fn new() -> Result<Self> {
        Ok(Self {
            atr: AverageTrueRange::new(14).unwrap(),
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
        let max = self.data_a.len() - 1;
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
        let a = self.atr.next(value);
        self.data_a.push(a);
        Ok(())
    }
    //FIXME MONEKY PATCHING
    fn next_OHLC(&mut self, OHLC: (f64, f64, f64, f64)) -> Result<()> {
        let bar = DataItem::builder()
            .open(OHLC.0)
            .high(OHLC.1)
            .low(OHLC.2)
            .close(OHLC.3)
            .volume(OHLC.3)
            .build()
            .unwrap();
        let a = self.atr.next(&bar);
        self.data_a.push(a);
        Ok(())
    }
}
