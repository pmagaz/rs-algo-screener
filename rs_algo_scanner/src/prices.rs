use crate::candle::Candle;
use crate::helpers::poly::fit;
use rs_algo_shared::helpers::comp::percentage_change;

use rs_algo_shared::helpers::date::Local;
use rs_algo_shared::models::{DataPoints, DbDateTime};

pub type PriceBreak = (bool, usize, f64, DbDateTime);

pub fn price_is_higher_upper_band_top(data: &DataPoints, candles: &Vec<Candle>) -> PriceBreak {
    let band = vec![data[0], data[2], data[4]];
    let break_price_comparator = |price: f64, break_point: f64| price > break_point;
    search_price_break(band, candles, &break_price_comparator)
}

pub fn price_is_higher_upper_band_bottom(data: &DataPoints, candles: &Vec<Candle>) -> PriceBreak {
    let band = vec![data[1], data[3]];
    let break_price_comparator = |price: f64, break_point: f64| price > break_point;
    search_price_break(band, candles, &break_price_comparator)
}

pub fn price_is_lower_low_band_top(data: &DataPoints, candles: &Vec<Candle>) -> PriceBreak {
    let band = vec![data[1], data[3]];
    let bottom_break = |price: f64, break_point: f64| price < break_point;
    search_price_break(band, candles, &bottom_break)
}

pub fn price_is_lower_low_band_bottom(data: &DataPoints, candles: &Vec<Candle>) -> PriceBreak {
    let band = vec![data[0], data[2], data[4]];
    let break_price_comparator = |price: f64, break_point: f64| price < break_point;
    search_price_break(band, candles, &break_price_comparator)
}

pub fn price_is_higher_peak(peak: (usize, f64), candles: &Vec<Candle>) -> PriceBreak {
    let mut band = vec![];
    band.push(peak);
    let break_price_comparator = |price: f64, break_point: f64| price > break_point;
    search_price_break(band, candles, &break_price_comparator)
}

pub fn price_is_lower_peak(peak: (usize, f64), candles: &Vec<Candle>) -> PriceBreak {
    let mut band = vec![];
    band.push(peak);
    let break_price_comparator = |price: f64, break_point: f64| price < break_point;
    search_price_break(band, candles, &break_price_comparator)
}

pub fn calculate_price_change(data_points: &DataPoints) -> f64 {
    percentage_change(data_points[4].1, data_points[3].1).abs()
}
//FIXME
pub fn calculate_price_target(data_points: &DataPoints) -> f64 {
    percentage_change(data_points[4].1, data_points[3].1).abs()
}

pub fn calculate_next_point(data: Vec<(usize, f64)>) -> f64 {
    let percentage_change = percentage_change(data[0].1, data[1].1);
    let last = data.last().unwrap().1;
    last + percentage_change * (last / 100.)
}

pub fn search_price_break(
    band: Vec<(usize, f64)>,
    candles: &Vec<Candle>,
    comparator: &dyn Fn(f64, f64) -> bool,
) -> PriceBreak {
    let keys: Vec<usize> = band.iter().map(|(x, _y)| *x).collect();
    let mut id = keys[0];
    if band.len() > 1 {
        let break_point = calculate_next_point(band);
        while id < candles.len() {
            let price = &candles[id].close();
            if comparator(*price, break_point) {
                return (
                    true,
                    id,
                    *price,
                    DbDateTime::from_chrono(candles[id].date()),
                );
            }
            id += 1;
        }
    }

    return (false, 0, 0., DbDateTime::from_chrono(Local::now()));
}
