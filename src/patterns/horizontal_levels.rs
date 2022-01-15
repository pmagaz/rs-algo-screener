use crate::error::{Result, RsAlgoError, RsAlgoErrorKind};
use super::peaks::Peaks;

use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
pub enum HorizontalLevelType {
  Resistance,
  Support,
}

#[derive(Debug, Clone)]
pub struct HorizontalLevel {
  price: f64,
  min_value: f64,
  max_value: f64,
  level_type: HorizontalLevelType,
}
impl HorizontalLevel {
  pub fn price(&self) -> &f64 {
    &self.price
  }
  pub fn level_type(&self) -> &HorizontalLevelType {
    &self.level_type
  }
}

#[derive(Debug, Clone)]
pub struct HorizontalLevels {
  horizontal_levels: HashMap<usize, HorizontalLevel>,
}

impl HorizontalLevels {
  pub fn new() -> HorizontalLevels {
    Self {
      horizontal_levels: HashMap::new(),
    }
  }

  pub fn horizontal_levels(&self) -> &HashMap<usize, HorizontalLevel> {
    &self.horizontal_levels
  }

  pub fn calculate_horizontal_levels(
    &mut self,
    current_price: &f64,
    data: &Vec<(usize, f64)>,
    peak_type: &Vec<f64>,
  ) -> Result<&HashMap<usize, HorizontalLevel>> {
    //TODO: Improve upper_channelion. It has to count occurrences and return a band
    //with min and max. Refactor two loops.
    let threshold = env::var("HORIZONTAL_LEVELS_THRESHOLD")
      .unwrap()
      .parse::<f64>()
      .unwrap();

    for (peak_index, peak_price) in data {
      let price = peak_price.abs();
      for (compare_index, _compare_price) in data {
        let last = peak_type[*compare_index].abs();
        let max = price.max(last);
        let min = last.min(price);
        let increase = max - min;
        let percentage_increase = (increase / price) * 100.;
        if percentage_increase > 0.
          && percentage_increase < threshold
          && peak_index != compare_index
          && !self.horizontal_levels.contains_key(&peak_index)
        {
          let min_value = price + percentage_increase;
          let max_value = price - percentage_increase;

          let level_type = match price {
            _x if &price >= &current_price => HorizontalLevelType::Resistance,
            _x if &price <= &current_price => HorizontalLevelType::Support,
            _ => HorizontalLevelType::Support,
          };

          self.horizontal_levels.insert(
            *peak_index,
            HorizontalLevel {
              price,
              min_value,
              max_value,
              level_type,
            },
          );
        }
      }
    }
    Ok(&self.horizontal_levels)
  }

  pub fn calculate_horizontal_highs(
    &mut self,
    current_price: &f64,
    peaks: &Peaks,
  ) -> Result<&HashMap<usize, HorizontalLevel>> {
    Ok(
      &self
        .calculate_horizontal_levels(current_price, peaks.extrema_maxima(), peaks.highs())
        .unwrap(),
    )
  }

  pub fn calculate_horizontal_lows(
    &mut self,
    current_price: &f64,
    peaks: &Peaks,
  ) -> Result<&HashMap<usize, HorizontalLevel>> {
    Ok(
      &self
        .calculate_horizontal_levels(current_price, peaks.extrema_minima(), peaks.lows())
        .unwrap(),
    )
  }
}
