use crate::candle::Candle;
use crate::helpers::comp::percentage_change;
use crate::helpers::poly::fit;

use rs_algo_shared::helpers::date::Local;
use rs_algo_shared::models::{DataPoints, DbDateTime};

pub type PriceBreak = (bool, usize, f64, DbDateTime);

pub fn price_is_higher_upper_band_top(data: &DataPoints, candles: &Vec<Candle>) -> PriceBreak {
    let band = vec![data[0], data[2], data[4]];
    let top_break = |price: f64, slope: f64| price > slope;
    search_price_break(band, candles, &top_break)
}

pub fn price_is_higher_upper_band_bottom(data: &DataPoints, candles: &Vec<Candle>) -> PriceBreak {
    let band = vec![data[1], data[3]];
    let top_break = |price: f64, slope: f64| price > slope;
    search_price_break(band, candles, &top_break)
}

pub fn price_is_lower_low_band_top(data: &DataPoints, candles: &Vec<Candle>) -> PriceBreak {
    let band = vec![data[1], data[3]];
    let bottom_break = |price: f64, slope: f64| price < slope;
    search_price_break(band, candles, &bottom_break)
}

pub fn price_is_lower_low_band_bottom(data: &DataPoints, candles: &Vec<Candle>) -> PriceBreak {
    let band = vec![data[0], data[2], data[4]];
    let top_break = |price: f64, slope: f64| price < slope;
    search_price_break(band, candles, &top_break)
}

pub fn price_is_higher_peak(peak: (usize, f64), candles: &Vec<Candle>) -> PriceBreak {
    let mut band = vec![];
    band.push(peak);
    let top_break = |price: f64, slope: f64| price > slope;
    search_price_break(band, candles, &top_break)
}

pub fn price_is_lower_peak(peak: (usize, f64), candles: &Vec<Candle>) -> PriceBreak {
    let mut band = vec![];
    band.push(peak);
    let top_break = |price: f64, slope: f64| price < slope;
    search_price_break(band, candles, &top_break)
}

pub fn calculate_price_change(data_points: &DataPoints) -> f64 {
    percentage_change(data_points[4].1, data_points[3].1).abs()
}
//FIXME
pub fn calculate_price_target(data_points: &DataPoints) -> f64 {
    percentage_change(data_points[4].1, data_points[3].1).abs()
}

pub fn search_price_break(
    band: Vec<(usize, f64)>,
    candles: &Vec<Candle>,
    comparator: &dyn Fn(f64, f64) -> bool,
) -> PriceBreak {
    //FIXME improve price break calculation
    let ids: Vec<f64> = band.iter().map(|(x, _y)| *x as f64).collect();
    let keys: Vec<usize> = band.iter().map(|(x, _y)| *x).collect();
    let prices: Vec<f64> = band.iter().map(|(_x, y)| *y).collect();
    let poly_degree = fit(&ids, &prices, 1);
    let mut id = keys[0];
    while id < candles.len() {
        let price = &candles[id].close();
        for (_x, slope) in &poly_degree {
            if comparator(*price, *slope) {
                return (
                    true,
                    id,
                    *price,
                    DbDateTime::from_chrono(candles[id].date()),
                );
            }
        }
        id += 1;
    }

    return (false, 0, 0., DbDateTime::from_chrono(Local::now()));
}
