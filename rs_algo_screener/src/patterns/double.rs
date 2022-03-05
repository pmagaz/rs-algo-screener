use super::highs_lows::*;
use super::pattern::DataPoints;
use crate::helpers::comp::*;

pub use rs_algo_shared::models::*;
pub fn is_top(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) && data[4].1 < data[3].1 && data[2].1 < data[1].1 {
        true
    } else {
        false
    }
}

pub fn top_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    let (upper_result, upper_id, upper_price) = price_is_lower_low_band_bottom(&data, close);
    if upper_result {
        PatternActive {
            active: true,
            index: upper_id,
            price: upper_price,
            break_direction: PatternDirection::Top,
        }
    } else {
        PatternActive {
            active: false,
            index: 0,
            price: 0.,
            break_direction: PatternDirection::None,
        }
    }
}

pub fn is_bottom(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) && data[4].1 > data[3].1 && data[2].1 > data[1].1 {
        true
    } else {
        false
    }
}

pub fn bottom_active(data: &DataPoints, close: &Vec<f64>, current_price: &f64) -> PatternActive {
    let (lower_result, lower_id, lower_price) = price_is_bigger_upper_band_top(&data, close);
    if lower_result {
        PatternActive {
            active: true,
            index: lower_id,
            price: lower_price,
            break_direction: PatternDirection::Bottom,
        }
    } else {
        PatternActive {
            active: false,
            index: 0,
            price: 0.,
            break_direction: PatternDirection::None,
        }
    }
}
