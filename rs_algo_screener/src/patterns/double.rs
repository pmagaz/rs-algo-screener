use crate::helpers::comp::*;
use crate::prices::*;

use rs_algo_shared::models::*;

pub fn is_top(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) && data[4].1 < data[3].1 && data[2].1 < data[1].1 {
        true
    } else {
        false
    }
}

pub fn top_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    let (top_result, top_id, top_price) = price_is_lower_low_band_bottom(&data, close);
    if top_result {
        PatternActive {
            active: true,
            index: top_id,
            price: top_price,
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
    let (bottom_result, bottom_id, bottom_price) = price_is_higher_upper_band_top(&data, close);
    if bottom_result {
        PatternActive {
            active: true,
            index: bottom_id,
            price: bottom_price,
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
