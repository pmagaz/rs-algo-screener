use super::highs_lows::*;
use super::pattern::DataPoints;
use crate::helpers::comp::*;

pub fn is_top(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) && data[4].1 < data[3].1 && data[2].1 < data[1].1 {
        true
    } else {
        false
    }
}

pub fn top_active(data: &DataPoints, close: &Vec<f64>) -> (bool, usize, f64) {
    price_is_lower_low_band_bottom(&data, close)
}

pub fn is_bottom(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) && data[4].1 > data[3].1 && data[2].1 > data[1].1 {
        true
    } else {
        false
    }
}

pub fn bottom_active(
    data: &DataPoints,
    close: &Vec<f64>,
    current_price: &f64,
) -> (bool, usize, f64) {
    price_is_bigger_upper_band_top(&data, close)
}
