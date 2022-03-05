use super::pattern::DataPoints;
use crate::helpers::comp::*;
use crate::helpers::poly::fit;
pub use rs_algo_shared::models::PatternActive;

pub fn is_higher_highs_top(data: &DataPoints) -> bool {
    if data[4].1 < data[2].1 && data[2].1 < data[0].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_highs_bottom(data: &DataPoints) -> bool {
    if data[3].1 < data[1].1 && data[3].1 > data[2].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_highs_top(data: &DataPoints) -> bool {
    if data[4].1 > data[2].1 && data[2].1 > data[0].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_lows_top(data: &DataPoints) -> bool {
    if data[3].1 < data[1].1 && data[1].1 < data[0].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_lows_top(data: &DataPoints) -> bool {
    if data[3].1 > data[1].1 && data[2].1 > data[1].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_lows_bottom(data: &DataPoints) -> bool {
    if data[4].1 < data[2].1 && data[2].1 < data[0].1
    // FIXME improve degree of increment
    //&& increase_equally((data[2].1, data[4].1), (data[0].1, data[2].1))
    {
        true
    } else {
        false
    }
}

pub fn is_lower_highs_bottom(data: &DataPoints) -> bool {
    if data[3].1 > data[1].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_lows_bottom(data: &DataPoints) -> bool {
    if data[4].1 > data[2].1 && data[2].1 > data[0].1 {
        true
    } else {
        false
    }
}

pub fn upper_band_is_equal_top(data: &DataPoints) -> bool {
    if is_equal(data[4].1, data[2].1) && is_equal(data[2].1, data[0].1) {
        true
    } else {
        false
    }
}

pub fn upper_band_is_equal_bottom(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) {
        true
    } else {
        false
    }
}

pub fn lower_band_is_equal_bottom(data: &DataPoints) -> bool {
    if is_equal(data[4].1, data[2].1) && is_equal(data[2].1, data[0].1) {
        true
    } else {
        false
    }
}

pub fn lower_band_is_equal_top(data: &DataPoints) -> bool {
    if is_equal(data[3].1, data[1].1) {
        true
    } else {
        false
    }
}

pub fn price_is_bigger_upper_band_top(data: &DataPoints, current_price: &f64) -> PatternActive {
    let ids: Vec<f64> = data.iter().map(|(x, _y)| *x as f64).collect();
    let prices: Vec<f64> = data.iter().map(|(_x, y)| *y).collect();
    let poly_degree = fit(&ids, &prices, 1);
    let mut key = 0;
    for (id, slope) in poly_degree {
        println!(
            "11111 {:} {:} {:} {:}",
            id,
            slope,
            prices[key],
            &prices[key] > &slope
        );
        key += 1;
    }

    if current_price > &data[4].1 && current_price > &data[2].1
    // && current_price > &data[0].1
    {
        PatternActive {
            active: true,
            index: 0,
        }
    } else {
        PatternActive {
            active: false,
            index: 0,
        }
    }
}

pub fn price_is_bigger_upper_band_bottom(data: &DataPoints, current_price: &f64) -> PatternActive {
    if current_price > &data[3].1 && current_price > &data[1].1 {
        PatternActive {
            active: true,
            index: 0,
        }
    } else {
        PatternActive {
            active: false,
            index: 0,
        }
    }
}

pub fn price_is_lower_low_band_top(data: &DataPoints, current_price: &f64) -> PatternActive {
    if current_price < &data[3].1 && current_price < &data[1].1 {
        PatternActive {
            active: true,
            index: 0,
        }
    } else {
        PatternActive {
            active: false,
            index: 0,
        }
    }
}

pub fn price_is_lower_low_band_bottom(data: &DataPoints, current_price: &f64) -> PatternActive {
    //TODO Verify data[0] comparision
    if current_price < &data[4].1 && current_price < &data[2].1
    // && current_price < &data[0].1
    {
        PatternActive {
            active: true,
            index: 0,
        }
    } else {
        PatternActive {
            active: false,
            index: 0,
        }
    }
}
