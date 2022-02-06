use super::highs_lows::*;
use super::pattern::DataPoints;

pub fn is_top(data: &DataPoints, _current_price: &f64) -> bool {
    if is_higher_highs_top(data, _current_price) && is_lower_lows_top(data, _current_price) {
        true
    } else {
        false
    }
}

pub fn is_bottom(data: &DataPoints, _current_price: &f64) -> bool {
    if is_higher_highs_bottom(data, _current_price) && is_lower_lows_bottom(data, _current_price) {
        true
    } else {
        false
    }
}
