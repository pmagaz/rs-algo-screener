use super::pattern::DataPoints;
use rs_algo_shared::helpers::comp::*;

pub fn three_increments(data: &DataPoints) -> bool {
    if data[0].1 > data[2].1 && data[2].1 > data[4].1
    //&& is_equal_distance((data[4].1, data[2].1), (data[2].1, data[0].1))
    {
        true
    } else {
        false
    }
}

pub fn is_higher_highs_top(data: &DataPoints) -> bool {
    three_increments(data)
}

pub fn is_higher_lows_bottom(data: &DataPoints) -> bool {
    three_increments(data)
}

pub fn two_increments(data: &DataPoints) -> bool {
    if data[1].1 > data[3].1 {
        true
    } else {
        false
    }
}

pub fn is_higher_highs_bottom(data: &DataPoints) -> bool {
    two_increments(data)
}

pub fn is_higher_lows_top(data: &DataPoints) -> bool {
    two_increments(data)
}

pub fn three_decrements(data: &DataPoints) -> bool {
    if data[0].1 < data[2].1 && data[2].1 < data[4].1
    //&& is_equal_distance((data[0].1, data[1].1), (data[2].1, data[3].1))
    {
        true
    } else {
        false
    }
}

pub fn is_lower_highs_top(data: &DataPoints) -> bool {
    three_decrements(data)
}

pub fn is_lower_lows_bottom(data: &DataPoints) -> bool {
    three_decrements(data)
}

pub fn two_decrements(data: &DataPoints) -> bool {
    if data[1].1 < data[3].1 {
        true
    } else {
        false
    }
}

pub fn is_lower_highs_bottom(data: &DataPoints) -> bool {
    two_decrements(data)
}

pub fn is_lower_lows_top(data: &DataPoints) -> bool {
    two_decrements(data)
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
