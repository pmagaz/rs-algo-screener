use super::highs_lows::*;
use super::pattern::pattern_active_result;
use crate::prices::*;

use rs_algo_shared::models::*;

pub fn is_top(data: &DataPoints) -> bool {
    if is_higher_highs_top(data) && is_lower_lows_top(data) {
        true
    } else {
        false
    }
}

pub fn is_bottom(data: &DataPoints) -> bool {
    if is_higher_highs_bottom(data) && is_lower_lows_bottom(data) {
        true
    } else {
        false
    }
}

pub fn broadening_top_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_top(&data, close),
        price_is_lower_low_band_top(&data, close),
    )
}

pub fn broadening_bottom_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_bottom(&data, close),
        price_is_lower_low_band_bottom(&data, close),
    )
}
