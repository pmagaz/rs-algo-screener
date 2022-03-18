use super::highs_lows::*;
use super::pattern::pattern_active_result;
use crate::prices::*;

use rs_algo_shared::models::*;

pub fn is_ascendant_top(data: &DataPoints) -> bool {
    if upper_band_is_equal_top(data) && is_higher_lows_top(data) {
        true
    } else {
        false
    }
}

pub fn ascendant_top_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_top(&data, close),
        price_is_lower_low_band_top(&data, close),
    )
}

pub fn ascendant_bottom_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_bottom(&data, close),
        price_is_lower_low_band_bottom(&data, close),
    )
}

pub fn descendant_top_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_top(&data, close),
        price_is_lower_low_band_top(&data, close),
    )
}

pub fn descendant_bottom_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_bottom(&data, close),
        price_is_lower_low_band_bottom(&data, close),
    )
}

pub fn symetrical_top_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_top(&data, close),
        price_is_lower_low_band_top(&data, close),
    )
}

pub fn symetrical_bottom_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_bottom(&data, close),
        price_is_lower_low_band_bottom(&data, close),
    )
}

pub fn is_ascendant_bottom(data: &DataPoints) -> bool {
    if upper_band_is_equal_bottom(data) && is_higher_lows_bottom(data) {
        true
    } else {
        false
    }
}

pub fn is_descendant_top(data: &DataPoints) -> bool {
    if lower_band_is_equal_top(data) && is_lower_highs_top(data) {
        true
    } else {
        false
    }
}

pub fn is_descendant_bottom(data: &DataPoints) -> bool {
    if lower_band_is_equal_bottom(data) && is_lower_highs_top(data) {
        true
    } else {
        false
    }
}

pub fn is_symmetrical_top(data: &DataPoints) -> bool {
    if is_lower_highs_top(data) && is_higher_lows_top(data) {
        true
    } else {
        false
    }
}

pub fn is_symmetrical_bottom(data: &DataPoints) -> bool {
    if is_lower_highs_bottom(data) && is_higher_lows_bottom(data) {
        true
    } else {
        false
    }
}
