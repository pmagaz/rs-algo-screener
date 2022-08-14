use crate::helpers::slope_intercept::slope_intercept;

use super::pattern::pattern_active_result;
use crate::candle::Candle;
use crate::prices::*;

use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::models::pattern::{DataPoints, PatternActive, PatternType};
use std::env;

pub fn is_upperhighs_upperlows_top(data: &DataPoints) -> bool {
    if is_upperhighs_top(data) && is_upperlows_bottom(data) {
        true
    } else {
        false
    }
}

pub fn is_upperhighs_upperlows_bottom(data: &DataPoints) -> bool {
    if is_upperhighs_bottom(data) && is_upperlows_top(data) {
        true
    } else {
        false
    }
}

pub fn ascendant_top_active(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: PatternType,
) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_upperlast_high_top(&data, candles, &pattern_type),
        price_is_lower_last_low_bottom(&data, candles, &pattern_type),
    )
}

pub fn ascendant_bottom_active(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: PatternType,
) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_upperlast_high_bottom(&data, candles, &pattern_type),
        price_is_lower_last_low_top(&data, candles, &pattern_type),
    )
}

pub fn is_lower_highs_lower_lows_top(data: &DataPoints) -> bool {
    if is_lower_highs_top(data) && is_lower_lows_bottom(data) {
        true
    } else {
        false
    }
}

pub fn is_lower_highs_lower_lows_bottom(data: &DataPoints) -> bool {
    if is_lower_highs_bottom(data) && is_lower_lows_top(data) {
        true
    } else {
        false
    }
}

pub fn descendant_top_active(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: PatternType,
) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_lower_last_low_top(&data, candles, &pattern_type),
        price_is_upperlast_high_bottom(&data, candles, &pattern_type),
    )
}

pub fn descendant_bottom_active(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: PatternType,
) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_lower_last_low_bottom(&data, candles, &pattern_type),
        price_is_upperlast_high_top(&data, candles, &pattern_type),
    )
}

pub fn is_upperhighs_top(data: &DataPoints) -> bool {
    if data[0].1 < data[2].1 {
        true
    } else {
        false
    }
}

pub fn is_upperlows_top(data: &DataPoints) -> bool {
    if data[0].1 < data[2].1 {
        true
    } else {
        false
    }
}

pub fn is_upperlows_bottom(data: &DataPoints) -> bool {
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

pub fn is_upperhighs_bottom(data: &DataPoints) -> bool {
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
// pub fn points_are_in_slope(data: &DataPoints) -> bool {
//     let slope_threshold = env::var("SLOPE_DEVIATION_THRESHOLD")
//         .unwrap()
//         .parse::<f64>()
//         .unwrap();

//     let threshold = ((data[1].1 - data[2].1) * slope_threshold).abs();
//     let (points_1, _y) = slope_intercept(data[0].0 as f64, data[0].1, data[2].0 as f64, data[2].1);
//     let (points_2, _y) = slope_intercept(data[2].0 as f64, data[2].1, data[4].0 as f64, data[4].1);

//     (round(points_1.abs(), 2) - round(points_2.abs(), 2)).abs() < threshold
// }

pub fn bands_have_same_slope(data: &DataPoints) -> bool {
    let threshold = env::var("SLOPE_DEVIATION_THRESHOLD")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let (slope_1, _y1) = slope_intercept(data[0].0 as f64, data[0].1, data[2].0 as f64, data[2].1);
    let (slope_2, _y2) = slope_intercept(data[1].0 as f64, data[1].1, data[3].0 as f64, data[3].1);
    let diff = (slope_1 / slope_2).abs();
    diff < threshold
}

pub fn are_parallel_lines(data: &DataPoints) -> bool {
    let threshold = env::var("PARALLEL_LINES_THRESHOLD")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let (slope_1, _y1) = slope_intercept(data[0].0 as f64, data[0].1, data[2].0 as f64, data[2].1);
    let (slope_2, _y2) = slope_intercept(data[1].0 as f64, data[1].1, data[3].0 as f64, data[3].1);

    let angle_degree = (slope_2 - slope_1 / (1. + slope_1 * slope_2)).abs() * 180.;
    angle_degree <= threshold
}

pub fn is_valid_triangle(data: &DataPoints) -> bool {
    let threshold = env::var("PARALLEL_LINES_THRESHOLD")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    ((data[0].1 * data[3].1) / (data[2].1 * data[1].1)).abs() <= threshold
}

pub fn is_valid_broadening(data: &DataPoints) -> bool {
    let threshold = env::var("PARALLEL_LINES_THRESHOLD")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let (slope_1, _y1) = slope_intercept(data[0].0 as f64, data[0].1, data[2].0 as f64, data[2].1);
    let (slope_2, _y2) = slope_intercept(data[1].0 as f64, data[1].1, data[3].0 as f64, data[3].1);

    let angle_degree = (slope_2 - slope_1 / (1. + slope_1 * slope_2)).abs() * 180.;

    //angle_degree <= threshold
    true
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
