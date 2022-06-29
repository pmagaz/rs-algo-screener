use super::highs_lows::*;
use super::pattern::pattern_active_result;
use crate::candle::Candle;
use crate::prices::*;
use rs_algo_shared::helpers::date::*;

use rs_algo_shared::models::pattern::{DataPoints, PatternActive, PatternType};

pub fn is_ascendant_top(data: &DataPoints) -> bool {
    if is_higher_highs_top(data)
        && is_higher_lows_bottom(data)
        //&& points_are_in_slope(data)
        && bands_have_same_slope(data)
        && are_parallel_lines(data)
        && has_minimum_bars(data)
        && has_minimum_target(data)
        && data[0].1 > data[1].1
        && data[2].1 > data[3].1
    {
        true
    } else {
        false
    }
}

pub fn is_ascendant_bottom(data: &DataPoints) -> bool {
    if is_higher_highs_bottom(data)
        && is_higher_lows_top(data)
        //&& points_are_in_slope(data)
        && bands_have_same_slope(data)
        && are_parallel_lines(data)
        && has_minimum_bars(data)
        && has_minimum_target(data)
        && data[0].1 < data[1].1
        && data[2].1 < data[3].1
    {
        true
    } else {
        false
    }
}

pub fn is_descendant_top(data: &DataPoints) -> bool {
    if is_lower_highs_top(data)
        && is_lower_lows_bottom(data)
        //&& points_are_in_slope(data)
        && bands_have_same_slope(data)
        && are_parallel_lines(data)
        && has_minimum_bars(data)
        && has_minimum_target(data)
        && data[0].1 > data[1].1
        && data[2].1 > data[3].1
    {
        true
    } else {
        false
    }
}

pub fn is_descendant_bottom(data: &DataPoints) -> bool {
    if is_lower_highs_bottom(data)
        && is_lower_lows_top(data)
        //&& points_are_in_slope(data)
        && bands_have_same_slope(data)
        && are_parallel_lines(data)
        && has_minimum_bars(data)
        && has_minimum_target(data)
        && data[0].1 < data[1].1
        && data[2].1 < data[3].1
    {
        true
    } else {
        false
    }
}

pub fn channel_descendant_top_active(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: PatternType,
) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_top(&data, candles, &pattern_type),
       (false, 0, 0., to_dbtime(Local::now() - Duration::days(1000))),
    )
}

pub fn channel_descendant_bottom_active(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: PatternType,
) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_bottom(&data, candles, &pattern_type),
        (false, 0, 0., to_dbtime(Local::now() - Duration::days(1000))),
    )
}

pub fn channel_ascendant_top_active(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: PatternType,
) -> PatternActive {
    pattern_active_result(
        &data,
        (false, 0, 0., to_dbtime(Local::now() - Duration::days(1000))),
        price_is_lower_low_band_bottom(&data, candles, &pattern_type),
    )
}

pub fn channel_ascendant_bottom_active(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: PatternType,
) -> PatternActive {
    pattern_active_result(
        &data,
        (false, 0, 0., to_dbtime(Local::now() - Duration::days(1000))),
        price_is_lower_low_band_top(&data, candles, &pattern_type),
    )
}
