use super::pattern::DataPoints;
use crate::helpers::comp::*;

pub fn is_higher_highs_top(data: &DataPoints) -> bool {
    if data[4].1 < data[2].1 && data[2].1 < data[0].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_highs_bottom(data: &DataPoints) -> bool {
    if data[3].1 < data[1].1 && data[3].1 > data[2].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_highs_top(data: &DataPoints) -> bool {
    if data[4].1 > data[2].1 && data[2].1 > data[0].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_lows_top(data: &DataPoints) -> bool {
    if data[3].1 < data[1].1 && data[1].1 < data[0].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_lows_top(data: &DataPoints) -> bool {
    if data[3].1 > data[1].1 && data[2].1 > data[1].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_lows_bottom(data: &DataPoints) -> bool {
    if data[4].1 < data[2].1 && data[2].1 < data[0].1
    // FIXME improve degree of increment
    //&& increase_equally((data[2].1, data[4].1), (data[0].1, data[2].1))
    {
        true
    } else {
        false
    }
}

pub fn is_lower_highs_bottom(data: &DataPoints) -> bool {
    if data[3].1 > data[1].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_lows_bottom(data: &DataPoints) -> bool {
    if data[4].1 > data[2].1 && data[2].1 > data[0].1 {
        true
    } else {
        false
    }
}

pub fn upper_band_is_equal_top(data: &DataPoints) -> bool {
    if is_equal(data[4].1, data[2].1) && is_equal(data[2].1, data[0].1) {
        true
    } else {
        false
    }
}

pub fn upper_band_is_equal_bottom(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) {
        true
    } else {
        false
    }
}

pub fn price_is_bigger_upper_band_top(data: &DataPoints, current_price: &f64) -> bool {
    if current_price > &data[4].1 && current_price > &data[2].1 && current_price > &data[0].1 {
        true
    } else {
        false
    }
}

pub fn price_is_bigger_upper_band_bottom(data: &DataPoints, current_price: &f64) -> bool {
    if current_price > &data[3].1 && current_price > &data[1].1 {
        true
    } else {
        false
    }
}

pub fn price_is_lower_lower_band_bottom(data: &DataPoints, current_price: &f64) -> bool {
    if current_price < &data[4].1 && current_price < &data[2].1 && current_price < &data[0].1 {
        true
    } else {
        false
    }
}

pub fn price_is_lower_lower_band_top(data: &DataPoints, current_price: &f64) -> bool {
    if current_price < &data[3].1 && current_price < &data[1].1 {
        true
    } else {
        false
    }
}

pub fn lower_band_is_equal_bottom(data: &DataPoints) -> bool {
    if is_equal(data[4].1, data[2].1) && is_equal(data[2].1, data[0].1) {
        true
    } else {
        false
    }
}

pub fn lower_band_is_equal_top(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) {
        true
    } else {
        false
    }
}
