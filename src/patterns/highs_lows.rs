use super::pattern::DataPoints;
use crate::helpers::comp::*;
use std::env;

pub fn is_lower_highs_top(data: &DataPoints, _current_price: &f64) -> bool {
    if data[4].1 > data[2].1 && data[2].1 > data[0].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_highs_bottom(data: &DataPoints, _current_price: &f64) -> bool {
    if data[3].1 > data[1].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_lows_top(data: &DataPoints, _current_price: &f64) -> bool {
    if data[3].1 < data[1].1 && data[1].1 < data[0].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_lows_bottom(data: &DataPoints, _current_price: &f64) -> bool {
    if data[4].1 < data[2].1 && data[2].1 < data[0].1
    // FIXME improve degree of increment
    //&& increase_equally((data[2].1, data[4].1), (data[0].1, data[2].1))
    {
        true
    } else {
        false
    }
}

pub fn is_equal_top(data: &DataPoints, _current_price: &f64) -> bool {
    if is_equal(data[4].1, data[2].1) && is_equal(data[2].1, data[0].1) {
        true
    } else {
        false
    }
}

pub fn is_equal_bottom(data: &DataPoints, _current_price: &f64) -> bool {
    if is_equal(data[3].1, data[1].1) {
        true
    } else {
        false
    }
}
