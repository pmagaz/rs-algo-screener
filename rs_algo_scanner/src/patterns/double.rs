use super::pattern::pattern_active_result;
use crate::helpers::comp::*;
use crate::prices::*;

use rs_algo_shared::models::*;
pub fn is_top(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) && data[4].1 < data[3].1 && data[2].1 < data[1].1 {
        true
    } else {
        false
    }
}

pub fn top_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_top(&data, close),
        price_is_lower_low_band_top(&data, close),
    )
}

pub fn is_bottom(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) && data[4].1 > data[3].1 && data[2].1 > data[1].1 {
        true
    } else {
        false
    }
}

pub fn bottom_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_bottom(&data, close),
        price_is_lower_low_band_bottom(&data, close),
    )
}
