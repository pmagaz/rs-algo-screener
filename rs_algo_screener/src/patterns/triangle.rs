use super::highs_lows::*;
use super::pattern::{DataPoints, PatternType};

pub fn is_ascendant_top(data: &DataPoints) -> bool {
    if upper_band_is_equal_top(data) && is_higher_lows_top(data) {
        true
    } else {
        false
    }
}

pub fn ascendant_top_active(data: &DataPoints, current_price: &f64) -> bool {
    price_is_bigger_upper_band_top(&data, current_price)
}

pub fn is_ascendant_bottom(data: &DataPoints) -> bool {
    if upper_band_is_equal_bottom(data) && is_higher_lows_bottom(data) {
        true
    } else {
        false
    }
}

pub fn ascendant_bottom_active(data: &DataPoints, current_price: &f64) -> bool {
    price_is_bigger_upper_band_bottom(&data, current_price)
}

pub fn is_descendant_top(data: &DataPoints) -> bool {
    if lower_band_is_equal_top(data) && is_lower_highs_top(data) {
        true
    } else {
        false
    }
}

pub fn descendant_top_active(data: &DataPoints, current_price: &f64) -> bool {
    price_is_lower_low_band_top(&data, current_price)
}

pub fn is_descendant_bottom(data: &DataPoints) -> bool {
    if lower_band_is_equal_bottom(data) && is_lower_highs_top(data) {
        true
    } else {
        false
    }
}

pub fn descendant_bottom_active(data: &DataPoints, current_price: &f64) -> bool {
    price_is_lower_low_band_bottom(&data, current_price)
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
