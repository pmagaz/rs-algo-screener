use super::pattern::pattern_active_result;
use crate::candle::Candle;
use crate::prices::*;
use rs_algo_shared::helpers::comp::*;

use rs_algo_shared::models::*;
use std::env;

pub fn is_top(data: &DataPoints) -> bool {
    let equal_threshold = env::var("EQUAL_THRESHOLD").unwrap().parse::<f64>().unwrap();
    let threshold = percentage_change(data[1].1, data[0].1) * equal_threshold;

    if is_equal(data[3].1, data[1].1, threshold)
        && data[0].1 < data[1].1
        && data[2].1 < data[1].1
        && data[2].1 < data[3].1
        && data[4].1 < data[3].1
    {
        true
    } else {
        false
    }
}

pub fn top_active(
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

pub fn is_bottom(data: &DataPoints) -> bool {
    let equal_threshold = env::var("EQUAL_THRESHOLD").unwrap().parse::<f64>().unwrap();
    let threshold = percentage_change(data[1].1, data[0].1) * equal_threshold;

    if is_equal(data[3].1, data[1].1, threshold)
        && data[0].1 > data[1].1
        && data[2].1 > data[1].1
        && data[2].1 > data[3].1
        && data[4].1 > data[3].1
    {
        true
    } else {
        false
    }
}

pub fn bottom_active(
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
