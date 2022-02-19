use super::highs_lows::*;
use super::pattern::{DataPoints, PatternType};
use crate::helpers::comp::*;

pub fn is_top(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) && data[4].1 < data[3].1 && data[2].1 < data[1].1 {
        true
    } else {
        false
    }
}

pub fn top_status(data: &DataPoints, current_price: &f64) -> PatternType {
    if price_is_lower_low_band_bottom(&data, current_price) {
        PatternType::DoubleTopActivated
    } else {
        PatternType::DoubleTop
    }
}

pub fn is_bottom(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) && data[4].1 > data[3].1 && data[2].1 > data[1].1 {
        true
    } else {
        false
    }
}

pub fn bottom_status(data: &DataPoints, current_price: &f64) -> PatternType {
    if price_is_bigger_upper_band_top(&data, current_price) {
        PatternType::DoubleBottomActivated
    } else {
        PatternType::DoubleBottom
    }
}
