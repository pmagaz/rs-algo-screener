use super::pattern::{DataPoints, PatternType};
use crate::helpers::comp::*;

pub fn is_top(data: &DataPoints, _current_price: &f64) -> PatternType {
    if is_equal(data[0].1, data[2].1) && data[1].1 < data[0].1 && data[1].1 < data[2].1 {
        PatternType::DoubleTop
    } else {
        PatternType::None
    }
}

pub fn is_bottom(data: &DataPoints, _current_price: &f64) -> PatternType {
    if is_equal(data[0].1, data[2].1) && data[1].1 > data[0].1 && data[1].1 > data[2].1 {
        PatternType::DoubleTop
    } else {
        PatternType::None
    }
}
