use super::highs_lows::*;
use super::pattern::{DataPoints, PatternType};

pub fn is_renctangle_top(data: &DataPoints) -> bool {
    if upper_band_is_equal_top(data) && lower_band_is_equal_top(data) {
        true
    } else {
        false
    }
}

pub fn rectangle_top_status(data: &DataPoints, current_price: &f64) -> PatternType {
    if price_is_bigger_upper_band_top(&data, current_price) {
        PatternType::RectangleTopActivatedUp
    } else if price_is_lower_low_band_top(&data, current_price) {
        PatternType::RectangleTopActivatedLow
    } else {
        PatternType::RectangleTop
    }
}

pub fn is_renctangle_bottom(data: &DataPoints) -> bool {
    if upper_band_is_equal_bottom(data) && lower_band_is_equal_bottom(data) {
        true
    } else {
        false
    }
}

pub fn rectangle_bottom_status(data: &DataPoints, current_price: &f64) -> PatternType {
    if price_is_bigger_upper_band_bottom(&data, current_price) {
        PatternType::RectangleTopActivatedUp
    } else if price_is_lower_low_band_bottom(&data, current_price) {
        PatternType::RectangleTopActivatedLow
    } else {
        PatternType::RectangleTop
    }
}
