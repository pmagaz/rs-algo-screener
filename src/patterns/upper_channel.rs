use super::peaks::{HashData, Peaks};
use crate::error::{Result, RsAlgoError, RsAlgoErrorKind};

use polyfit_rs::polyfit_rs::polyfit;
use std::collections::BTreeMap;
use std::env;

pub type Band = Vec<Vec<(usize, f64)>>;

#[derive(Debug, Clone)]
pub struct UpperChannel {
  upper_band: Band,
  lower_band: Band,
}

impl UpperChannel {
  pub fn new() -> UpperChannel {
    Self {
      upper_band: vec![],
      lower_band: vec![],
    }
  }
  pub fn upper_band(&self) -> &Band {
    &self.upper_band
  }

  pub fn lower_band(&self) -> &Band {
    &self.lower_band
  }

  pub fn scan(&mut self, peaks: &Peaks) -> &Self {
    let greater = |price: f64, previous_price: f64| price >= previous_price;
    self.upper_band = self.consecutive_peaks(peaks.local_maxima(), &greater);

    let lower = |price: f64, previous_price: f64| price <= previous_price;
    self.lower_band = self.consecutive_peaks(peaks.local_minima(), &lower);
    self
  }

  pub fn consecutive_peaks(
    &mut self,
    data: &Vec<(usize, f64)>,
    comparator: &dyn Fn(f64, f64) -> bool,
  ) -> Vec<Vec<(usize, f64)>> {
    let mut result: Vec<Vec<(usize, f64)>> = vec![vec![]];
    let mut n = 0;
    let min_higher_highs = env::var("MIN_HIGHER_HIGHS").unwrap().parse::<usize>().unwrap();
    let max = data.len() - 1;

    for (_idx, _price) in data {
      let mut x = 0;
      let mut tmp: Vec<(usize, f64)> = vec![];
      let mut track: Vec<(usize, usize)> = vec![];
      let mut occurrences = 0;

      while x < max {
        let key = n + x;
        let previous_key = n + x + 1;
        if key < max {
          let index = data[key].0;
          let price = data[key].1;
          let previous_price = data[previous_key].1;

          tmp.push((index, price));

          if comparator(price, previous_price) {
            occurrences += 1;
            track.push((index, occurrences));
          } else {
            if occurrences >= min_higher_highs {
              // let max = track.iter().max_by_key(|t| t.1);
              // let max_id = max.unwrap().0;
              result.push(tmp);
            }
            break;
          }
        }
        x += 1;
      }
      track = vec![];
      n += 1;
    }

    if result.len() > 0 {
      result.remove(0);
    } else {
    }
    //println!("555555, {:?}", result);
    result
  }

  /*
  pub fn scan_bands(&mut self, data: &Vec<(usize, f64)>) -> Band {
    println!("111111, {:?}", data);
    let mut result: Vec<Vec<(usize, f64)>> = vec![vec![]];
    let number_peaks = env::var("CHANNEL_PEAKS").unwrap().parse::<usize>().unwrap();
    let data = data;
    let mut x = 0;
    for (idx, price) in data {
      let mut x_max_values: Vec<f64> = vec![];
      let mut y_max_values: Vec<f64> = vec![];

      let mut n = 0;
      while n <= number_peaks {
        let key = x + n;
        if key < data.len() {
          x_max_values.push(data[key].0 as f64);
          y_max_values.push(data[key].1);
          let max_poly = polyfit(&x_max_values, &y_max_values, 1).unwrap();
          let max_slope = max_poly[0];
          let max_intercept = max_poly[1];
          //println!("11111, {:?}, {:?}", data[key].1, max_slope);
        }
        n += 1;
        if n == number_peaks {
          let max_poly = polyfit(&x_max_values, &y_max_values, 1).unwrap();
          let max_slope = max_poly[0];
          let max_intercept = max_poly[1];

          let poly_degree: Vec<(usize, f64)> = x_max_values
            .iter()
            .map(|item| {
              return (*item as usize, max_slope + max_intercept * item);
            })
            .collect();
          // println!("3333333, {:?}, {:?}", y_max_values, max_slope);
          result.push(poly_degree);
        }
      }
      x += 1;
    }
    result
  }*/
}
