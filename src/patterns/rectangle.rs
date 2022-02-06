use super::highs_lows::*;
use super::pattern::DataPoints;

pub fn is_renctangle_top(data: &DataPoints, _current_price: &f64) -> bool {
    if upper_band_is_equal_top(data, _current_price)
        && lower_band_is_equal_top(data, _current_price)
    {
        true
    } else {
        false
    }
}

pub fn is_renctangle_bottom(data: &DataPoints, _current_price: &f64) -> bool {
    if upper_band_is_equal_bottom(data, _current_price)
        && lower_band_is_equal_bottom(data, _current_price)
    {
        true
    } else {
        false
    }
}
