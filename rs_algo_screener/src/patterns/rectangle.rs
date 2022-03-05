use super::highs_lows::*;
use super::pattern::DataPoints;

use rs_algo_shared::models::*;

pub fn is_renctangle_top(data: &DataPoints) -> bool {
    if upper_band_is_equal_top(data) && lower_band_is_equal_top(data) {
        true
    } else {
        false
    }
}

pub fn rectangle_top_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    let (top_result, top_id, top_price) = price_is_bigger_upper_band_bottom(&data, close);
    let (bottom_result, bottom_id, bottom_price) = price_is_lower_low_band_bottom(&data, close);

    if top_result {
        PatternActive {
            active: true,
            index: top_id,
            price: top_price,
            break_direction: PatternDirection::Top,
        }
    } else if bottom_result {
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

pub fn rectangle_bottom_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    let (top_result, top_id, top_price) = price_is_bigger_upper_band_bottom(&data, close);
    let (bottom_result, bottom_id, bottom_price) = price_is_lower_low_band_bottom(&data, close);

    if top_result {
        PatternActive {
            active: true,
            index: top_id,
            price: top_price,
            break_direction: PatternDirection::Top,
        }
    } else if bottom_result {
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

pub fn is_renctangle_bottom(data: &DataPoints) -> bool {
    if upper_band_is_equal_bottom(data) && lower_band_is_equal_bottom(data) {
        true
    } else {
        false
    }
}
