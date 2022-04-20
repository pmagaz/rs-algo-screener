use super::pattern::DataPoints;
use rs_algo_shared::helpers::comp::*;
use std::env;

pub fn is_higher_highs_top(data: &DataPoints) -> bool {
    if data[0].1 < data[2].1 && data[2].1 < data[4].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_lows_top(data: &DataPoints) -> bool {
    if data[0].1 < data[2].1 && data[2].1 < data[4].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_lows_bottom(data: &DataPoints) -> bool {
    if data[1].1 < data[3].1 {
        true
    } else {
        false
    }
}

pub fn two_increments(data: &DataPoints) -> bool {
    if data[1].1 > data[3].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_highs_bottom(data: &DataPoints) -> bool {
    if data[1].1 < data[3].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_highs_top(data: &DataPoints) -> bool {
    if data[0].1 > data[2].1 && data[2].1 > data[4].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_highs_bottom(data: &DataPoints) -> bool {
    if data[1].1 > data[3].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_lows_top(data: &DataPoints) -> bool {
    if data[0].1 > data[2].1 && data[2].1 > data[4].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_lows_bottom(data: &DataPoints) -> bool {
    if data[1].1 > data[3].1 {
        true
    } else {
        false
    }
}

pub fn upper_band_is_equal_top(data: &DataPoints) -> bool {
    let equal_threshold = env::var("EQUAL_THRESHOLD").unwrap().parse::<f64>().unwrap();
    let threshold = percentage_change(data[4].1, data[0].1) * equal_threshold;

    if is_equal(data[0].1, data[2].1, threshold) && is_equal(data[2].1, data[4].1, threshold) {
        true
    } else {
        false
    }
}

pub fn upper_band_is_equal_bottom(data: &DataPoints) -> bool {
    let equal_threshold = env::var("EQUAL_THRESHOLD").unwrap().parse::<f64>().unwrap();
    let threshold = percentage_change(data[3].1, data[1].1) * equal_threshold;

    if is_equal(data[3].1, data[1].1, threshold) {
        true
    } else {
        false
    }
}

pub fn lower_band_is_equal_bottom(data: &DataPoints) -> bool {
    let equal_threshold = env::var("EQUAL_THRESHOLD").unwrap().parse::<f64>().unwrap();
    let threshold = percentage_change(data[3].1, data[1].1) * equal_threshold;
    if is_equal(data[3].1, data[1].1, threshold) {
        true
    } else {
        false
    }
}

pub fn lower_band_is_equal_top(data: &DataPoints) -> bool {
    let equal_threshold = env::var("EQUAL_THRESHOLD").unwrap().parse::<f64>().unwrap();
    let threshold = percentage_change(data[4].1, data[0].1) * equal_threshold;
    if is_equal(data[0].1, data[2].1, threshold) && is_equal(data[2].1, data[4].1, threshold) {
        true
    } else {
        false
    }
}
