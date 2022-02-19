use super::highs_lows::*;
use super::pattern::{DataPoints, PatternType};

pub fn is_top(data: &DataPoints, _current_price: &f64) -> PatternType {
    if is_higher_highs_top(data) && is_lower_lows_top(data) {
        PatternType::BroadeningTop
    } else {
        PatternType::None
    }
}

pub fn is_bottom(data: &DataPoints, _current_price: &f64) -> PatternType {
    if is_higher_highs_bottom(data) && is_lower_lows_bottom(data) {
        PatternType::BroadeningBottom
    } else {
        PatternType::None
    }
}
