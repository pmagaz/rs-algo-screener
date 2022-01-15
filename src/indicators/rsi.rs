use super::Indicator;
use crate::error::Result;

use ta::indicators::RelativeStrengthIndex;
use ta::Next;

#[derive(Debug, Clone)]
pub struct Rsi {
  rsi: RelativeStrengthIndex,
  data_a: Vec<f64>,
  data_b: Vec<f64>,
}

impl Indicator for Rsi {
  fn new() -> Result<Self> {
    Ok(Self {
      rsi: RelativeStrengthIndex::new(14).unwrap(),
      data_a: vec![],
      data_b: vec![],
    })
  }

  fn data_a(&self) -> &Vec<f64> {
    &self.data_a
  }

  fn data_b(&self) -> &Vec<f64> {
    &self.data_b
  }

  fn next(&mut self, value: f64) -> Result<()> {
    let a = self.rsi.next(value);
    self.data_a.push(a);
    Ok(())
  }
}
