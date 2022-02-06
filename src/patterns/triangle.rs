use super::highs_lows::*;
use super::pattern::DataPoints;

pub fn is_ascendant_top(data: &DataPoints, _current_price: &f64) -> bool {
    if upper_band_is_equal_top(data, _current_price) && is_higher_lows_top(data, _current_price) {
        true
    } else {
        false
    }
}

pub fn is_ascendant_bottom(data: &DataPoints, _current_price: &f64) -> bool {
    if upper_band_is_equal_bottom(data, _current_price)
        && is_higher_lows_bottom(data, _current_price)
    {
        true
    } else {
        false
    }
}

pub fn is_descendant_top(data: &DataPoints, _current_price: &f64) -> bool {
    if lower_band_is_equal_top(data, _current_price) && is_lower_highs_top(data, _current_price) {
        true
    } else {
        false
    }
}

pub fn is_descendant_bottom(data: &DataPoints, _current_price: &f64) -> bool {
    if lower_band_is_equal_bottom(data, _current_price) && is_lower_highs_top(data, _current_price)
    {
        true
    } else {
        false
    }
}

pub fn is_symmetrical_top(data: &DataPoints, _current_price: &f64) -> bool {
    if is_lower_highs_top(data, _current_price) && is_higher_lows_top(data, _current_price) {
        true
    } else {
        false
    }
}

pub fn is_symmetrical_bottom(data: &DataPoints, _current_price: &f64) -> bool {
    if is_lower_highs_bottom(data, _current_price) && is_higher_lows_bottom(data, _current_price) {
        true
    } else {
        false
    }
}
