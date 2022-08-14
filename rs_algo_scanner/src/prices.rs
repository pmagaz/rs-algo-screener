use crate::candle::Candle;

use crate::helpers::slope_intercept::slope_intercept;
use rs_algo_shared::helpers::comp::percentage_change;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::pattern::*;
use std::env;

pub type PriceBreak = (bool, usize, f64, DbDateTime);

pub fn price_is_upperupper_band_top(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    let points = vec![data[2], data[4]];
    let break_price_comparator = |price: f64, price_break: f64| price > price_break;
    search_price_break(points, candles, &break_price_comparator)
}

pub fn price_is_upperupper_band_bottom(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    let points = vec![data[3], data[5]];
    let break_price_comparator = |price: f64, price_break: f64| price > price_break;
    search_price_break(points, candles, &break_price_comparator)
}

pub fn price_is_lower_low_band_bottom(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    let points = vec![data[3], data[5]];
    let bottom_break = |price: f64, price_break: f64| price < price_break;
    search_price_break(points, candles, &bottom_break)
}

pub fn price_is_lower_low_band_top(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    let points = vec![data[2], data[4]];
    let break_price_comparator = |price: f64, price_break: f64| price < price_break;
    search_price_break(points, candles, &break_price_comparator)
}

pub fn price_is_upperlast_high_top(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    let points = vec![data[2]];
    let break_price_comparator = |price: f64, price_break: f64| price > price_break;
    search_price_break(points, candles, &break_price_comparator)
}

pub fn price_is_upperlast_high_bottom(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    let points = vec![data[3]];
    let break_price_comparator = |price: f64, price_break: f64| price > price_break;
    search_price_break(points, candles, &break_price_comparator)
}

pub fn price_is_lower_last_low_top(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    let points = vec![data[3]];
    let break_price_comparator = |price: f64, price_break: f64| price > price_break;
    search_price_break(points, candles, &break_price_comparator)
}

pub fn price_is_lower_last_low_bottom(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    let points = vec![data[2]];
    let break_price_comparator = |price: f64, price_break: f64| price > price_break;
    search_price_break(points, candles, &break_price_comparator)
}

pub fn price_is_upperpeak(
    peak: (usize, f64),
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    let mut band = vec![];
    band.push(peak);
    let break_price_comparator = |price: f64, price_break: f64| price > price_break;
    search_price_break(band, candles, &break_price_comparator)
}

pub fn price_is_lower_peak(
    peak: (usize, f64),
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    let mut band = vec![];
    band.push(peak);
    let break_price_comparator = |price: f64, price_break: f64| price < price_break;
    search_price_break(band, candles, &break_price_comparator)
}

pub fn calculate_price_change(data_points: &DataPoints) -> f64 {
    percentage_change(data_points[4].1, data_points[3].1).abs()
}
//FIXME
pub fn calculate_price_target(
    pattern_direction: &PatternDirection,
    data_points: &DataPoints,
) -> f64 {
    if data_points.len() < 2 {
        0.
    } else {
        match pattern_direction {
            PatternDirection::Top => percentage_change(data_points[1].1, data_points[0].1).abs(),
            PatternDirection::Bottom => percentage_change(data_points[0].1, data_points[1].1).abs(),
            _ => percentage_change(data_points[1].1, data_points[0].1).abs(),
        }
    }
}

//FIXME UPDATE PATTERN BREAK DETECTION
pub fn search_price_break(
    points: Vec<(usize, f64)>,
    candles: &Vec<Candle>,
    comparator: &dyn Fn(f64, f64) -> bool,
) -> PriceBreak {
    let logarithmic = env::var("LOGARITHMIC_SCANNER")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    let len = points.len();
    if len > 1 {
        let start = points[0];
        let end = points[1];

        let start_index = start.0 as usize;
        let end_index = candles.len(); //end.0 as usize;

        let (slope, y_intercept) = slope_intercept(start.0 as f64, start.1, end.0 as f64, end.1);
        for n in (start_index..=end_index).step_by(2) {
            if n < end_index {
                let next_price = (slope * n as f64) + y_intercept;
                let current_price = match logarithmic {
                    true => candles[n].close().exp(),
                    false => candles[n].close(),
                };
                let current_date = &candles[n].date();

                if comparator(current_price, next_price) {
                    return (true, n, next_price, to_dbtime(*current_date));
                }
            }
        }
    }

    return (false, 0, 0., to_dbtime(Local::now() - Duration::days(1000)));
}
