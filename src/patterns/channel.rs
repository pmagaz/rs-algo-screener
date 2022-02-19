use super::highs_lows::*;
use super::pattern::{DataPoints, PatternType};

pub fn is_ascendant_top(data: &DataPoints, _current_price: &f64) -> PatternType {
    if is_higher_highs_top(data) && is_higher_lows_top(data) {
        PatternType::ChannelUpTop
    } else {
        PatternType::None
    }
}

pub fn is_ascendant_bottom(data: &DataPoints, _current_price: &f64) -> PatternType {
    if is_higher_highs_bottom(data) && is_higher_lows_bottom(data) {
        PatternType::ChannelUpBottom
    } else {
        PatternType::None
    }
}

pub fn is_descendant_top(data: &DataPoints, _current_price: &f64) -> PatternType {
    if is_lower_highs_top(data) && is_lower_lows_top(data) {
        PatternType::ChannelDownTop
    } else {
        PatternType::None
    }
}

pub fn is_descendant_bottom(data: &DataPoints, _current_price: &f64) -> PatternType {
    if is_lower_highs_bottom(data) && is_lower_lows_bottom(data) {
        PatternType::ChannelDownBottom
    } else {
        PatternType::None
    }
}
