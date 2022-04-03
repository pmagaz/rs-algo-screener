use super::pattern::pattern_active_result;
use crate::candle::Candle;
use crate::prices::*;
use rs_algo_shared::helpers::comp::*;

use rs_algo_shared::models::*;
use std::env;

pub fn is_top(data: &DataPoints) -> bool {
    let threshold = env::var("EQUAL_THRESHOLD").unwrap().parse::<f64>().unwrap();
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

pub fn top_active(data: &DataPoints, candles: &Vec<Candle>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_top(&data, candles),
        price_is_lower_low_band_bottom(&data, candles),
    )
}

pub fn is_bottom(data: &DataPoints) -> bool {
    let threshold = env::var("EQUAL_THRESHOLD").unwrap().parse::<f64>().unwrap();
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

pub fn bottom_active(data: &DataPoints, candles: &Vec<Candle>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_bottom(&data, candles),
        price_is_lower_low_band_top(&data, candles),
    )
}
