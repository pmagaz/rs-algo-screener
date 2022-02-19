use super::highs_lows::*;
use super::pattern::{DataPoints, PatternType};

pub fn is_ascendant_top(data: &DataPoints, current_price: &f64) -> PatternType {
    if upper_band_is_equal_top(data) && is_higher_lows_top(data) {
        if price_is_bigger_upper_band_top(data, current_price) {
            PatternType::TriangleAscendantTopActivated
        } else {
            PatternType::TriangleAscendantTop
        }
    } else {
        PatternType::None
    }
}

pub fn is_ascendant_bottom(data: &DataPoints, current_price: &f64) -> PatternType {
    if upper_band_is_equal_bottom(data) && is_higher_lows_bottom(data) {
        if price_is_bigger_upper_band_bottom(data, current_price) {
            PatternType::TriangleAscendantBottomActivated
        } else {
            PatternType::TriangleAscendantBottom
        }
    } else {
        PatternType::None
    }
}

pub fn is_descendant_top(data: &DataPoints, current_price: &f64) -> PatternType {
    if lower_band_is_equal_top(data) && is_lower_highs_top(data) {
        if price_is_lower_lower_band_top(data, current_price) {
            PatternType::TriangleDescendantTopActivated
        } else {
            PatternType::TriangleDescendantTop
        }
    } else {
        PatternType::None
    }
}

pub fn is_descendant_bottom(data: &DataPoints, current_price: &f64) -> PatternType {
    if lower_band_is_equal_bottom(data) && is_lower_highs_top(data) {
        if price_is_lower_lower_band_bottom(data, current_price) {
            PatternType::TriangleDescendantBottomActivated
        } else {
            PatternType::TriangleDescendantBottom
        }
    } else {
        PatternType::None
    }
}

pub fn is_symmetrical_top(data: &DataPoints, _current_price: &f64) -> PatternType {
    // TODO activated symetrical detection
    if is_lower_highs_top(data) && is_higher_lows_top(data) {
        PatternType::TriangleSymmetricalTop
    } else {
        PatternType::None
    }
}

pub fn is_symmetrical_bottom(data: &DataPoints, _current_price: &f64) -> PatternType {
    if is_lower_highs_bottom(data) && is_higher_lows_bottom(data) {
        PatternType::TriangleSymmetricalBottom
    } else {
        PatternType::None
    }
}
