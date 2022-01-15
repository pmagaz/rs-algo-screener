pub mod macd;
pub mod rsi;
pub mod stoch;

use crate::candle::Candle;
use crate::error::Result;
use crate::indicators::macd::Macd;
use crate::indicators::rsi::Rsi;
use crate::indicators::stoch::Stoch;

use std::fmt;
use std::marker::Sized;

#[derive(Debug, Clone)]
pub enum IndicatorType {
  MacD,
  Stoch,
  Rsi,
}
#[derive(Debug, Clone)]
pub struct Indicators {
  pub macd: Macd,
  pub stoch: Stoch,
  pub rsi: Rsi,
}

impl Indicators {
  pub fn new() -> Result<Self> {
    Ok(Self {
      macd: Macd::new().unwrap(),
      stoch: Stoch::new().unwrap(),
      rsi: Rsi::new().unwrap(),
    })
  }

  pub fn calculate_macd(&mut self, data: &Vec<Candle>) -> Result<()> {
    for candle in data {
      self.macd.next(candle.close());
    }

    Ok(())
  }

  pub fn macd(&self) -> &Macd {
    &self.macd
  }

  pub fn rsi(&self) -> &Rsi {
    &self.rsi
  }

  pub fn stoch(&self) -> &Stoch {
    &self.stoch
  }
}

pub trait Indicator {
  fn new() -> Result<Self>
  where
    Self: Sized;
  fn next(&mut self, value: f64) -> Result<()>;
  fn data_a(&self) -> &Vec<f64>;
  fn data_b(&self) -> &Vec<f64>;
}

// impl fmt::Debug for dyn Indicator {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "Hi")
//   }
// }

// #[derive(Debug, Clone)]
// pub struct Indicators {
//   indicators: Vec<Box<dyn Indicator>>,
// }

// impl Indicators {
//   pub fn new() -> Self {
//     Indicators { indicators: vec![] }
//   }
// }
