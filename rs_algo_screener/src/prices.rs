use crate::helpers::poly::fit;

use rs_algo_shared::models::*;

pub fn price_is_higher_upper_band_top(data: &DataPoints, close: &Vec<f64>) -> (bool, usize, f64) {
    let band = vec![data[0], data[2], data[4]];
    let top_break = |price: f64, slope: f64| price > slope;
    search_price_break(band, close, &top_break)
}

pub fn price_is_higher_upper_band_bottom(
    data: &DataPoints,
    close: &Vec<f64>,
) -> (bool, usize, f64) {
    let band = vec![data[1], data[3]];
    let top_break = |price: f64, slope: f64| price > slope;
    search_price_break(band, close, &top_break)
}

pub fn price_is_lower_low_band_top(data: &DataPoints, close: &Vec<f64>) -> (bool, usize, f64) {
    let band = vec![data[1], data[3]];
    let bottom_break = |price: f64, slope: f64| price < slope;
    search_price_break(band, close, &bottom_break)
}

pub fn price_is_lower_low_band_bottom(data: &DataPoints, close: &Vec<f64>) -> (bool, usize, f64) {
    let band = vec![data[0], data[2], data[4]];
    let top_break = |price: f64, slope: f64| price < slope;
    search_price_break(band, close, &top_break)
}

pub fn search_price_break(
    band: Vec<(usize, f64)>,
    close: &Vec<f64>,
    comparator: &dyn Fn(f64, f64) -> bool,
) -> (bool, usize, f64) {
    let ids: Vec<f64> = band.iter().map(|(x, _y)| *x as f64).collect();
    let keys: Vec<usize> = band.iter().map(|(x, _y)| *x).collect();
    let prices: Vec<f64> = band.iter().map(|(_x, y)| *y).collect();
    let poly_degree = fit(&ids, &prices, 1);
    let mut id = keys[0];
    while id < close.len() {
        let price = &close[id];
        for (_x, slope) in &poly_degree {
            if comparator(*price, *slope) {
                return (true, id, *price);
            }
        }
        id += 1;
    }

    return (false, 0, 0.);
}
