use crate::candle::Candle;

use rs_algo_shared::helpers::comp::percentage_change;

use rs_algo_shared::helpers::date::{DbDateTime, Duration, Local};
use rs_algo_shared::models::{DataPoints, PatternType};
use std::env;

pub type PriceBreak = (bool, usize, f64, DbDateTime);

pub fn price_is_higher_upper_band_top(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    //let next_points = calculate_price_break(&data, pattern_type);
    //let band = vec![data[0], data[2], data[4]];
    let band = vec![data[4]];
    let break_price_comparator = |price: f64, price_break: f64| price > price_break;
    search_price_break(band, candles, &break_price_comparator)
}

pub fn price_is_higher_upper_band_bottom(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    // let next_points = calculate_price_break(&data, pattern_type);
    //let band = vec![data[1], data[3]];
    let band = vec![data[3]];
    let break_price_comparator = |price: f64, price_break: f64| price > price_break;
    search_price_break(band, candles, &break_price_comparator)
}

pub fn price_is_lower_low_band_bottom(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    // let next_points = calculate_price_break(&data, pattern_type);
    // let band = vec![data[1], data[3]];
    let band = vec![data[3]];
    let bottom_break = |price: f64, price_break: f64| price < price_break;
    search_price_break(band, candles, &bottom_break)
}

pub fn price_is_lower_low_band_top(
    data: &DataPoints,
    candles: &Vec<Candle>,
    pattern_type: &PatternType,
) -> PriceBreak {
    // let next_points = calculate_price_break(&data, pattern_type);
    // let band = vec![data[0], data[2], data[4]];
    let band = vec![data[4]];
    let break_price_comparator = |price: f64, price_break: f64| price < price_break;
    search_price_break(band, candles, &break_price_comparator)
}

pub fn price_is_higher_peak(
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
pub fn calculate_price_target(data_points: &DataPoints) -> f64 {
    percentage_change(data_points[4].1, data_points[3].1).abs()
}

pub fn calculate_price_break(data: &Vec<(usize, f64)>, pattern_type: &PatternType) -> (usize, f64) {
    let percentage_change = percentage_change(data[0].1, data[1].1);
    let price = data.get(data.len() - 1).unwrap().1;

    let next_points = match pattern_type {
        PatternType::ChannelUp => calculate_price_break_up(price, percentage_change),
        PatternType::ChannelDown => calculate_price_break_down(price, percentage_change),
        PatternType::Triangle => calculate_price_break_up(price, percentage_change),
        PatternType::TriangleSym => calculate_price_break_up(price, percentage_change),
        PatternType::TriangleDown => calculate_price_break_down(price, percentage_change),
        PatternType::TriangleUp => calculate_price_break_up(price, percentage_change),
        PatternType::Rectangle => calculate_price_break_up(price, percentage_change),
        PatternType::ChannelUp => calculate_price_break_up(price, percentage_change),
        PatternType::ChannelDown => calculate_price_break_down(price, percentage_change),
        PatternType::Broadening => calculate_price_break_up(price, percentage_change),
        PatternType::DoubleTop => calculate_price_break_down(price, percentage_change),
        PatternType::DoubleBottom => calculate_price_break_up(price, percentage_change),
        PatternType::HeadShoulders => calculate_price_break_down(price, percentage_change),
        PatternType::None => (9999999, 9999999.),
    };
    next_points
}

pub fn calculate_price_break_up(price: f64, percentage: f64) -> (usize, f64) {
    (9999999, price + percentage * (price / 100.))
}

pub fn calculate_price_break_down(price: f64, percentage: f64) -> (usize, f64) {
    (9999999, price - percentage * (price / 100.))
}

pub fn search_price_break(
    band: Vec<(usize, f64)>,
    candles: &Vec<Candle>,
    comparator: &dyn Fn(f64, f64) -> bool,
) -> PriceBreak {
    let keys: Vec<usize> = band.iter().map(|(x, _y)| *x).collect();
    let mut current_id = *keys.get(keys.len() - 1).unwrap();
    if band.len() > 0 {
        while current_id < candles.len() {
            let current_price = &candles[current_id].close();
            let current_date = &candles[current_id].date();

            for (id, price_break) in &band {
                if comparator(*current_price, *price_break) {
                    return (
                        true,
                        *id,
                        *price_break,
                        DbDateTime::from_chrono(*current_date),
                    );
                }
            }
            current_id += 1;
        }
    }

    return (
        false,
        0,
        0.,
        DbDateTime::from_chrono(Local::now() - Duration::days(1000)),
    );
}
