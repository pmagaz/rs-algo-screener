use super::highs_lows::*;
use super::pattern::{DataPoints, PatternType};

use rs_algo_shared::models::*;

pub fn is_ascendant_top(data: &DataPoints) -> bool {
    if upper_band_is_equal_top(data) && is_higher_lows_top(data) {
        true
    } else {
        false
    }
}

pub fn ascendant_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    let (top_result, top_id, top_price) = price_is_bigger_upper_band_top(&data, close);
    let (bottom_result, bottom_id, bottom_price) = price_is_bigger_upper_band_bottom(&data, close);

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

pub fn descendant_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    let (top_result, top_id, top_price) = price_is_lower_low_band_top(&data, close);
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

pub fn symetrical_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    let (top_result, top_id, top_price) = price_is_lower_low_band_top(&data, close);
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

pub fn is_ascendant_bottom(data: &DataPoints) -> bool {
    if upper_band_is_equal_bottom(data) && is_higher_lows_bottom(data) {
        true
    } else {
        false
    }
}

// pub fn ascendant_bottom_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
//     price_is_bigger_upper_band_bottom(&data, close)
// }

pub fn is_descendant_top(data: &DataPoints) -> bool {
    if lower_band_is_equal_top(data) && is_lower_highs_top(data) {
        true
    } else {
        false
    }
}

pub fn descendant_top_active(data: &DataPoints, close: &Vec<f64>) -> (bool, usize, f64) {
    price_is_lower_low_band_top(&data, close)
}

pub fn is_descendant_bottom(data: &DataPoints) -> bool {
    if lower_band_is_equal_bottom(data) && is_lower_highs_top(data) {
        true
    } else {
        false
    }
}

pub fn descendant_bottom_active(data: &DataPoints, close: &Vec<f64>) -> (bool, usize, f64) {
    price_is_lower_low_band_bottom(&data, close)
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
