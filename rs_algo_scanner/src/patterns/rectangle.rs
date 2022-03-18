use super::highs_lows::*;
use super::pattern::pattern_active_result;
use crate::prices::*;

use rs_algo_shared::models::*;
pub fn is_renctangle_top(data: &DataPoints) -> bool {
    if upper_band_is_equal_top(data) && lower_band_is_equal_top(data) {
        true
    } else {
        false
    }
}

pub fn rectangle_top_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_top(&data, close),
        price_is_lower_low_band_top(&data, close),
    )
}

pub fn rectangle_bottom_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_bottom(&data, close),
        price_is_lower_low_band_bottom(&data, close),
    )
}

pub fn is_renctangle_bottom(data: &DataPoints) -> bool {
    if upper_band_is_equal_bottom(data) && lower_band_is_equal_bottom(data) {
        true
    } else {
        false
    }
}
