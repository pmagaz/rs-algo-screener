use super::highs_lows::*;
use super::pattern::DataPoints;
pub use rs_algo_shared::models::*;

pub fn is_renctangle_top(data: &DataPoints) -> bool {
    if upper_band_is_equal_top(data) && lower_band_is_equal_top(data) {
        true
    } else {
        false
    }
}

pub fn rectangle_top_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    let (upper_result, upper_id, upper_price) = price_is_bigger_upper_band_top(&data, close);
    let (lower_result, lower_id, lower_price) = price_is_lower_low_band_top(&data, close);

    if upper_result {
        PatternActive {
            active: true,
            index: upper_id,
            price: upper_price,
            break_direction: PatternDirection::Top,
        }
    } else if lower_result {
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

pub fn rectangle_bottom_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    let (upper_result, upper_id, upper_price) = price_is_bigger_upper_band_bottom(&data, close);
    let (lower_result, lower_id, lower_price) = price_is_lower_low_band_bottom(&data, close);

    if upper_result {
        PatternActive {
            active: true,
            index: upper_id,
            price: upper_price,
            break_direction: PatternDirection::Top,
        }
    } else if lower_result {
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

pub fn is_renctangle_bottom(data: &DataPoints) -> bool {
    if upper_band_is_equal_bottom(data) && lower_band_is_equal_bottom(data) {
        true
    } else {
        false
    }
}
