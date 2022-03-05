use super::pattern::DataPoints;
use crate::helpers::comp::*;
use crate::helpers::poly::fit;

use rs_algo_shared::models::PatternActive;

pub fn is_higher_highs_top(data: &DataPoints) -> bool {
    if data[2].1 > data[0].1 && data[4].1 > data[2].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_highs_bottom(data: &DataPoints) -> bool {
    if data[3].1 > data[1].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_lows_top(data: &DataPoints) -> bool {
    if data[0].1 < data[2].1 && data[2].1 < data[4].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_lows_bottom(data: &DataPoints) -> bool {
    if data[3].1 > data[1].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_highs_top(data: &DataPoints) -> bool {
    if data[2].1 < data[0].1 && data[4].1 < data[2].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_highs_bottom(data: &DataPoints) -> bool {
    if data[3].1 < data[1].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_lows_top(data: &DataPoints) -> bool {
    if data[3].1 < data[1].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_lows_bottom(data: &DataPoints) -> bool {
    if data[0].1 > data[2].1 && data[2].1 > data[4].1 {
        true
    } else {
        false
    }
}

pub fn upper_band_is_equal_top(data: &DataPoints) -> bool {
    if is_equal(data[4].1, data[2].1)
        && is_equal(data[2].1, data[0].1)
        && is_equal(data[4].1, data[0].1)
        && data[2].1 > data[1].1
        && data[4].1 > data[3].1
    {
        true
    } else {
        false
    }
}

pub fn lower_band_is_equal_top(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) && data[1].1 < data[2].1 && data[3].1 < data[4].1 {
        true
    } else {
        false
    }
}

pub fn upper_band_is_equal_bottom(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) && data[1].1 > data[0].1 && data[3].1 > data[2].1 {
        true
    } else {
        false
    }
}

pub fn lower_band_is_equal_bottom(data: &DataPoints) -> bool {
    if is_equal(data[4].1, data[2].1)
        && is_equal(data[2].1, data[0].1)
        && is_equal(data[4].1, data[0].1)
        && data[1].1 > data[0].1
        && data[3].1 > data[2].1
    {
        true
    } else {
        false
    }
}

pub fn price_is_bigger_upper_band_top(data: &DataPoints, close: &Vec<f64>) -> (bool, usize, f64) {
    let band = vec![data[0], data[2], data[4]];
    search_price_break(band, close)
}

pub fn price_is_lower_low_band_top(data: &DataPoints, close: &Vec<f64>) -> (bool, usize, f64) {
    let band = vec![data[1], data[3]];
    search_price_break(band, close)
}

pub fn price_is_bigger_upper_band_bottom(
    data: &DataPoints,
    close: &Vec<f64>,
) -> (bool, usize, f64) {
    let band = vec![data[1], data[3]];
    search_price_break(band, close)
}

pub fn price_is_lower_low_band_bottom(data: &DataPoints, close: &Vec<f64>) -> (bool, usize, f64) {
    let band = vec![data[0], data[2], data[4]];
    search_price_break(band, close)
}

pub fn search_price_break(band: Vec<(usize, f64)>, close: &Vec<f64>) -> (bool, usize, f64) {
    let ids: Vec<f64> = band.iter().map(|(x, _y)| *x as f64).collect();
    let keys: Vec<usize> = band.iter().map(|(x, _y)| *x).collect();
    let prices: Vec<f64> = band.iter().map(|(_x, y)| *y).collect();
    let poly_degree = fit(&ids, &prices, 1);

    //FIXME DESTRUCTURE CLOSE FROM INDEX
    let mut id = keys[0];
    while id < close.len() {
        let price = &close[id];
        for (_x, slope) in &poly_degree {
            if price > slope {
                //println!("2222222  {:} {:} {:} {:}", id, slope, price, price > slope);
                //not_active = false;
                // return PatternActive {
                //     active: true,
                //     index: id,
                // };
                return (true, id, *price);
            }
        }
        id += 1;
    }

    return (false, 0, 0.);
}
