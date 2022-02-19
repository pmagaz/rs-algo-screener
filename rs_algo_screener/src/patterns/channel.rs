use super::highs_lows::*;
use super::pattern::DataPoints;

pub fn is_ascendant_top(data: &DataPoints) -> bool {
    if is_higher_highs_top(data) && is_higher_lows_top(data) {
        true
    } else {
        false
    }
}

pub fn is_ascendant_bottom(data: &DataPoints) -> bool {
    if is_higher_highs_bottom(data) && is_higher_lows_bottom(data) {
        true
    } else {
        false
    }
}

pub fn is_descendant_top(data: &DataPoints) -> bool {
    if is_lower_highs_top(data) && is_lower_lows_top(data) {
        true
    } else {
        false
    }
}

pub fn is_descendant_bottom(data: &DataPoints) -> bool {
    if is_lower_highs_bottom(data) && is_lower_lows_bottom(data) {
        true
    } else {
        false
    }
}
