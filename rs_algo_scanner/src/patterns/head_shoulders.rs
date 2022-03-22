use super::pattern::pattern_active_result;
use crate::helpers::comp::*;
use crate::prices::*;

use rs_algo_shared::models::*;

pub fn is_hs(data: &DataPoints) -> bool {
    if data[0].1 > data[1].1
        && data[2].1 > data[1].1
        && data[2].1 > data[4].1
        && (data[0].1 - data[4].1).abs() <= 0.03 * average(&[data[0].1, data[4].1])
        && (data[1].1 - data[3].1).abs() <= 0.03 * average(&[data[0].1, data[4].1])
    {
        true
    } else {
        false
    }
}

pub fn is_inverse(data: &DataPoints) -> bool {
    if data[0].1 < data[1].1
        && data[2].1 < data[1].1
        && data[2].1 < data[4].1
        && (data[0].1 - data[4].1).abs() <= 0.03 * average(&[data[0].1, data[4].1])
        && (data[1].1 - data[3].1).abs() <= 0.03 * average(&[data[0].1, data[4].1])
    {
        true
    } else {
        false
    }
}

pub fn hs_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
    pattern_active_result(
        &data,
        price_is_higher_peak(data[2], close),
        price_is_lower_peak(data[2], close),
    )
}

// pub fn inverse_active(data: &DataPoints, close: &Vec<f64>) -> PatternActive {
//     pattern_active_result(
//         &data,
//         price_is_higher_peak(&data, close),
//         price_is_lower_low_band_top(&data, close),
//     )
// }
