use super::Indicator;
use crate::error::Result;

use ta::indicators::ExponentialMovingAverage;
use ta::Next;

#[derive(Debug, Clone)]
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

  fn data_a(&self) -> &Vec<f64> {
    &self.data_a
  }

  fn data_b(&self) -> &Vec<f64> {
    &self.data_b
  }

  fn next(&mut self, value: f64) -> Result<()> {
    let a = self.ema12.next(value) - self.ema26.next(value);
    let b = self.ema9.next(a);
    self.data_a.push(a);
    self.data_b.push(b);
    Ok(())
  }
}
