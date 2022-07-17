use super::highs_lows::*;
use super::pattern::pattern_active_result;
use crate::candle::Candle;
use crate::prices::*;

use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::models::pattern::{DataPoints, PatternActive, PatternType};

use std::env;

pub fn is_renctangle_top(data: &DataPoints) -> bool {
    let equal_threshold = env::var("EQUAL_THRESHOLD").unwrap().parse::<f64>().unwrap();
    let threshold = percentage_change(data[1].1, data[0].1) * equal_threshold;

    if upper_band_is_equal_top(data)
        && lower_band_is_equal_bottom(data)
        && bands_have_same_slope(data)
        //&& are_parallel_lines(data)
        //&& is_equal_distance((data[0].1, data[1].1), (data[2].1, data[3].1), threshold)
        && has_minimum_bars(data)
    // && has_minimum_target(data)
    {
        true
    } else {
        false
    }
}

pub fn is_renctangle_bottom(data: &DataPoints) -> bool {
    let equal_threshold = env::var("EQUAL_THRESHOLD").unwrap().parse::<f64>().unwrap();
    let threshold = percentage_change(data[1].1, data[0].1) * equal_threshold;

    if upper_band_is_equal_bottom(data)
        && lower_band_is_equal_top(data)
        && bands_have_same_slope(data)
        //&& are_parallel_lines(data)
        //&& is_equal_distance((data[0].1, data[1].1), (data[2].1, data[3].1), threshold)
        && has_minimum_bars(data)
    // && has_minimum_target(data)
    {
        true
    } else {
        false
    }
}

pub fn rectangle_top_active(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: PatternType,
) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_top(&data, candles, &pattern_type),
        price_is_lower_low_band_bottom(&data, candles, &pattern_type),
    )
}

pub fn rectangle_bottom_active(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: PatternType,
) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_bottom(&data, candles, &pattern_type),
        price_is_lower_low_band_top(&data, candles, &pattern_type),
    )
}
