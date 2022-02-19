use super::highs_lows::*;
use super::pattern::{DataPoints, PatternType};

pub fn is_ascendant_top(data: &DataPoints) -> bool {
    if upper_band_is_equal_top(data) && is_higher_lows_top(data) {
        true
    } else {
        false
    }
}

pub fn ascendant_top_status(data: &DataPoints, current_price: &f64) -> PatternType {
    match price_is_bigger_upper_band_top(&data, current_price) {
        true => PatternType::TriangleAscendantTopActivated,
        false => PatternType::TriangleAscendantTop,
    }
}

pub fn is_ascendant_bottom(data: &DataPoints) -> bool {
    if upper_band_is_equal_bottom(data) && is_higher_lows_bottom(data) {
        true
    } else {
        false
    }
}

pub fn ascendant_bottom_status(data: &DataPoints, current_price: &f64) -> PatternType {
    match price_is_bigger_upper_band_bottom(&data, current_price) {
        true => PatternType::TriangleAscendantBottomActivated,
        false => PatternType::TriangleAscendantBottom,
    }
}

pub fn is_descendant_top(data: &DataPoints) -> bool {
    if lower_band_is_equal_top(data) && is_lower_highs_top(data) {
        true
    } else {
        false
    }
}

pub fn descendant_top_status(data: &DataPoints, current_price: &f64) -> PatternType {
    match price_is_lower_low_band_top(&data, current_price) {
        true => PatternType::TriangleDescendantTopActivated,
        false => PatternType::TriangleDescendantTop,
    }
}

pub fn is_descendant_bottom(data: &DataPoints) -> bool {
    if lower_band_is_equal_bottom(data) && is_lower_highs_top(data) {
        true
    } else {
        false
    }
}

pub fn descendant_bottom_status(data: &DataPoints, current_price: &f64) -> PatternType {
    match price_is_lower_low_band_bottom(&data, current_price) {
        true => PatternType::TriangleDescendantBottomActivated,
        false => PatternType::TriangleDescendantBottom,
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
