use super::peaks::Peaks;

use crate::error::Result;
use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::horizontal_level::{HorizontalLevel, HorizontalLevelType};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HorizontalLevelType {
    Resistance,
    Support,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizontalLevel {
    price: f64,
    occurrences: usize,
    level_type: HorizontalLevelType,
}
impl HorizontalLevel {
    pub fn price(&self) -> &f64 {
        &self.price
    }
    pub fn level_type(&self) -> &HorizontalLevelType {
        &self.level_type
    }
}*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizontalLevels {
    highs: Vec<HorizontalLevel>,
    lows: Vec<HorizontalLevel>,
}

impl HorizontalLevels {
    pub fn new() -> HorizontalLevels {
        Self {
            highs: vec![],
            lows: vec![],
        }
    }

    pub fn highs(&self) -> &Vec<HorizontalLevel> {
        &self.highs
    }

    pub fn lows(&self) -> &Vec<HorizontalLevel> {
        &self.lows
    }

    pub fn calculate_bands(
        &mut self,
        current_price: &f64,
        local_maxima: &Vec<(usize, f64)>,
        peak_type: &Vec<f64>,
    ) -> Result<Vec<HorizontalLevel>> {
        let mut hash: HashMap<String, HorizontalLevel> = HashMap::new();

        let min_ocurrences = env::var("MIN_HORIZONTAL_LEVELS_OCCURENCES")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let threshold = env::var("HORIZONTAL_LEVELS_THRESHOLD")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        for (peak_index, peak_price) in local_maxima {
            let price = *peak_price;
            for (compare_index, compare_price) in local_maxima {
                //println!("11111 {:?} {:?}", price, compare_price);

                // let price_diff = max_price - min_price;
                // kernel_bandwidth = kernel_bandwidth * price_diff;
                // local_prominence = local_prominence * price_diff;
                if compare_index != peak_index && is_equal(price, *compare_price, threshold) {
                    let mut occurrences = 1;
                    if hash.contains_key(&price.to_string()) {
                        occurrences += 1;
                    }
                    let level_type = match price {
                        _x if &price >= &current_price => HorizontalLevelType::Resistance,
                        _x if &price <= &current_price => HorizontalLevelType::Support,
                        _ => HorizontalLevelType::Support,
                    };
                    hash.insert(
                        price.to_string(),
                        HorizontalLevel {
                            price,
                            occurrences,
                            date: to_dbtime(Local::now()),
                            level_type,
                        },
                    );
                }
            }
        }

        let result: Vec<HorizontalLevel> = hash
            .into_iter()
            .filter(|(_k, level)| level.occurrences >= min_ocurrences)
            .map(|(_k, v)| v)
            .collect();

        Ok(result)
    }

    pub fn calculate_horizontal_highs(&mut self, current_price: &f64, peaks: &Peaks) -> Result<()> {
        self.highs = self
            .calculate_bands(current_price, peaks.local_maxima(), peaks.highs())
            .unwrap();
        Ok(())
    }

    pub fn calculate_horizontal_lows(&mut self, current_price: &f64, peaks: &Peaks) -> Result<()> {
        self.lows = self
            .calculate_bands(current_price, peaks.local_maxima(), peaks.lows())
            .unwrap();
        Ok(())
    }
}
