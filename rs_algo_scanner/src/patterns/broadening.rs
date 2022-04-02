use super::highs_lows::*;
use super::pattern::pattern_active_result;
use crate::candle::Candle;
use crate::prices::*;

use rs_algo_shared::models::*;

pub fn is_top(data: &DataPoints) -> bool {
    if is_higher_highs_top(data)
        && is_lower_lows_bottom(data)
        && data[0].1 > data[1].1
        && data[2].1 > data[3].1
        && data[4].1 > data[3].1
    {
        true
    } else {
        false
    }
}

pub fn is_bottom(data: &DataPoints) -> bool {
    if is_higher_highs_bottom(data)
        && is_lower_lows_top(data)
        && data[1].1 > data[0].1
        && data[3].1 > data[2].1
        && data[3].1 > data[4].1
    {
        true
    } else {
        false
    }
}

pub fn broadening_top_active(data: &DataPoints, candles: &Vec<Candle>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_top(&data, candles),
        price_is_lower_low_band_top(&data, candles),
    )
}

pub fn broadening_bottom_active(data: &DataPoints, candles: &Vec<Candle>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_upper_band_bottom(&data, candles),
        price_is_lower_low_band_bottom(&data, candles),
    )
}
