use super::highs_lows::*;
use super::pattern::{DataPoints, PatternType};

pub fn is_renctangle_top(data: &DataPoints, _current_price: &f64) -> PatternType {
    if upper_band_is_equal_top(data) && lower_band_is_equal_top(data) {
        PatternType::RectangleTop
    } else {
        PatternType::None
    }
}

pub fn is_renctangle_bottom(data: &DataPoints, _current_price: &f64) -> PatternType {
    if upper_band_is_equal_bottom(data) && lower_band_is_equal_bottom(data) {
        PatternType::RectangleBottom
    } else {
        PatternType::None
    }
}
