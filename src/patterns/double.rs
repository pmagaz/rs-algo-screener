use super::pattern::DataPoints;
use crate::helpers::comp::*;

pub fn is_active_double_top(data: &DataPoints, _current_price: &f64) -> bool {
    if data[0].1 < data[2].1 && is_equal(data[1].1, data[3].1) {
        println!("[DOUBLE TOP ACTIVATED] {:?}", data);
        true
    } else {
        false
    }
}

pub fn is_top(data: &DataPoints, _current_price: &f64) -> bool {
    if is_equal(data[0].1, data[2].1) && data[1].1 < data[0].1 && data[1].1 < data[2].1 {
        println!("[DOUBLE TOP] {:?}", data);
        true
    } else {
        false
    }
}

pub fn is_active_double_bottom(data: &DataPoints, _current_price: &f64) -> bool {
    if data[0].1 > data[2].1 && is_equal(data[1].1, data[3].1) {
        println!("[DOUBLE BOTTOM ACTIVATED] {:?}", data);
        true
    } else {
        false
    }
}

pub fn is_bottom(data: &DataPoints, _current_price: &f64) -> bool {
    if is_equal(data[0].1, data[2].1) && data[1].1 > data[0].1 && data[1].1 > data[2].1 {
        println!("[DOUBLE BOTTOM] {:?}", data);
        true
    } else {
        false
    }
}
