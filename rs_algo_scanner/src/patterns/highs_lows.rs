use super::pattern::DataPoints;
use crate::helpers::slope_intercept::slope_intercept;

use round::round;
use rs_algo_shared::helpers::comp::*;
use std::env;

pub fn is_higher_highs_top(data: &DataPoints) -> bool {
    if data[0].1 < data[2].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_lows_top(data: &DataPoints) -> bool {
    if data[0].1 < data[2].1 {
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
    if data[0].1 > data[2].1 {
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
    if data[0].1 > data[2].1 {
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
    let threshold = percentage_change(data[2].1, data[0].1) * equal_threshold;

    if is_equal(data[0].1, data[2].1, threshold) {
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
    let threshold = percentage_change(data[2].1, data[0].1) * equal_threshold;
    if is_equal(data[0].1, data[2].1, threshold) {
        true
    } else {
        false
    }
}

//FXIME MOVE TO RIGHT PLACE
pub fn points_are_in_slope(data: &DataPoints) -> bool {
    let slope_threshold = env::var("SLOPE_DEVIATION_THRESHOLD")
        .unwrap()
        .parse::<f64>()
        .unwrap();

    let threshold = ((data[1].1 - data[2].1) * slope_threshold).abs();
    let (points_1, _y) = slope_intercept(data[0].0 as f64, data[0].1, data[2].0 as f64, data[2].1);
    let (points_2, _y) = slope_intercept(data[2].0 as f64, data[2].1, data[4].0 as f64, data[4].1);

    (round(points_1.abs(), 2) - round(points_2.abs(), 2)).abs() < threshold
}

pub fn bands_have_same_slope(data: &DataPoints) -> bool {
    let slope_threshold = env::var("SLOPE_DEVIATION_THRESHOLD")
        .unwrap()
        .parse::<f64>()
        .unwrap();

    let (points_1, _y) = slope_intercept(data[0].0 as f64, data[0].1, data[2].0 as f64, data[2].1);
    let (points_2, _y) = slope_intercept(data[1].0 as f64, data[1].1, data[3].0 as f64, data[3].1);

    let threshold = (points_1 - points_2).abs();

    (round(points_1.abs(), 2) - round(points_2.abs(), 2)).abs() < slope_threshold
}

pub fn are_parallel_lines(data: &DataPoints) -> bool {
    let slope_threshold = env::var("SLOPE_DEVIATION_THRESHOLD")
        .unwrap()
        .parse::<f64>()
        .unwrap();

    let (points_1, _y) = slope_intercept(data[0].0 as f64, data[0].1, data[2].0 as f64, data[2].1);
    let (points_2, _y) = slope_intercept(data[1].0 as f64, data[1].1, data[3].0 as f64, data[3].1);

    (round(points_1.abs(), 2) - round(points_2.abs(), 2)).abs() < slope_threshold
}

pub fn has_minimum_bars(data: &DataPoints) -> bool {
    let min_bars = env::var("MIN_PATTERN_BARS")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    data[2].0 - data[0].0 > min_bars && data[3].0 - data[1].0 > min_bars
}

pub fn has_minimum_target(data: &DataPoints) -> bool {
    let min_target = env::var("MINIMUM_PATTERN_TARGET")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    //true
    //FIXME
    percentage_change(data[0].1, data[1].1).abs() > min_target
}
