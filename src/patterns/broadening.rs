use super::pattern::DataPoints;
use crate::helpers::comp;

pub fn top(data: &DataPoints, _current_price: &f64) -> bool {
    if data[4].1 > data[2].1
        && data[2].1 > data[0].1
        && data[3].1 < data[1].1
        && data[2].1 < data[0].1
    {
        println!("[DESCENDANT TRIANGLE] {:?}", data);
        true
    } else {
        false
    }
}

pub fn bottom(data: &DataPoints, _current_price: &f64) -> bool {
    if data[4].1 < data[2].1
        && data[2].1 < data[0].1
        && data[3].1 > data[1].1
        && data[2].1 > data[0].1
    {
        println!("[DESCENDANT TRIANGLE] {:?}", data);
        true
    } else {
        false
    }
}
