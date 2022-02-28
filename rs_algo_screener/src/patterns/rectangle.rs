use super::highs_lows::*;
use super::pattern::{DataPoints, PatternType};

pub fn is_renctangle_top(data: &DataPoints) -> bool {
    if upper_band_is_equal_top(data) && lower_band_is_equal_top(data) {
        true
    } else {
        false
    }
}

pub fn rectangle_top_active(data: &DataPoints, current_price: &f64) -> bool {
    price_is_bigger_upper_band_top(&data, current_price)
        || price_is_lower_low_band_top(&data, current_price)
}

pub fn is_renctangle_bottom(data: &DataPoints) -> bool {
    if upper_band_is_equal_bottom(data) && lower_band_is_equal_bottom(data) {
        true
    } else {
        false
    }
}

pub fn rectangle_bottom_active(data: &DataPoints, current_price: &f64) -> bool {
    price_is_bigger_upper_band_bottom(&data, current_price)
        || price_is_lower_low_band_bottom(&data, current_price)
}
