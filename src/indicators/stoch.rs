use super::Indicator;
use crate::error::Result;

use ta::indicators::SlowStochastic;

use ta::indicators::ExponentialMovingAverage;
use ta::Next;

#[derive(Debug, Clone)]
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

  fn data_a(&self) -> &Vec<f64> {
    &self.data_a
  }

  fn data_b(&self) -> &Vec<f64> {
    &self.data_b
  }

  fn next(&mut self, value: f64) -> Result<()> {
    let a = self.stoch.next(value);
    let b = self.ema.next(a);
    self.data_a.push(a);
    self.data_b.push(b);
    Ok(())
  }
}
