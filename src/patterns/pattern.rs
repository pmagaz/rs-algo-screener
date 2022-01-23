use super::peaks::Peaks;
use crate::helpers::comp;

use std::cmp;
use std::env;
type point = (usize, f64);
type data_points = (point, point, point, point, point);
//TODO use TRAITS

#[derive(Debug, Clone)]
pub enum PatternType {
    Default,
    DoubleTop,
    DoubleTopActivated,
    DoubleBottom,
    DoubleBottomActivated,
    HeadAndShoulders,
    HeadAndShouldersActivated,
    InverseHeadAndShoulders,
    InverseHeadAndShouldersActivated,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pattern_type: PatternType,
    data: data_points,
}

// #[derive(Debug, Clone)]
// pub struct Patterns {
//     patterns: Vec<Pattern>,
//     upper_channel: UpperChannel,
// }

impl Pattern {
    pub fn new() -> Self {
        Pattern {
            pattern_type: PatternType::Default,
            data: ((0, 0.), (0, 0.), (0, 0.), (0, 0.), (0, 0.)),
        }
    }

    pub fn detect_pattern(&mut self, peaks: &Peaks, current_price: &f64) {
        let max_bars = env::var("PATTERNS_MAX_BARS")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let num_data_points = env::var("PATTERNS_DATA_POINTS")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut locals = [peaks.local_maxima.clone(), peaks.local_minima.clone()].concat();

        locals.sort_by(|(a, _), (b, _)| a.cmp(b));
        locals.reverse();
        let mut iter = locals.windows(num_data_points);
        let mut leches = true;
        while leches {
            match iter.next() {
                Some(x) => {
                    let data_points = self.get_data_points(x);
                    self.detect_double_top(&data_points, current_price);
                    self.detect_double_bottom(&data_points, current_price);
                    self.detect_broadening_top(&data_points, current_price);
                    self.detect_broadening_bottom(&data_points, current_price);
                    self.detect_descendant_triangle(&data_points, current_price);
                    self.detect_head_and_shoulders(&data_points, current_price);
                    self.detect_inverse_head_and_shoulders(&data_points, current_price);
                }
                None => {
                    leches = false;
                }
            }
        }
    }
    pub fn get_data_points(&self, data: &[(usize, f64)]) -> data_points {
        let e1 = data[0];
        let e2 = data[1];
        let e3 = data[2];
        let e4 = data[3];
        let e5 = data[4];
        (e1, e2, e3, e4, e5)
    }

    pub fn detect_double_top(
        &self,
        data: &data_points,
        _current_price: &f64,
    ) -> Option<(point, point, point)> {
        if comp::is_equal(data.0 .1, data.2 .1) && data.1 .1 < data.0 .1 && data.1 .1 < data.2 .1 {
            println!("[DOUBLE TOP] {:?}", data);
            Some((data.0, data.1, data.2))
        } else {
            None
        }
    }

    pub fn detect_double_bottom(
        &self,
        data: &data_points,
        _current_price: &f64,
    ) -> Option<(point, point, point)> {
        if comp::is_equal(data.0 .1, data.2 .1) && data.1 .1 > data.0 .1 && data.1 .1 > data.2 .1 {
            println!("[DOUBLE BOTTOM] {:?}", data);
            Some((data.0, data.1, data.2))
        } else {
            None
        }
    }

    pub fn detect_broadening_top(
        &self,
        data: &data_points,
        _current_price: &f64,
    ) -> Option<(point, point, point)> {
        if data.0 .1 > data.1 .1
            && data.0 .1 < data.2 .1
            && data.2 .1 < data.4 .1
            && data.1 .1 > data.3 .1
        {
            println!("[BROADENING TOP] {:?}", data);
            Some((data.0, data.1, data.2))
        } else {
            None
        }
    }

    pub fn detect_broadening_bottom(
        &self,
        data: &data_points,
        _current_price: &f64,
    ) -> Option<(point, point, point)> {
        if data.0 .1 < data.1 .1
            && data.0 .1 > data.2 .1
            && data.2 .1 > data.4 .1
            && data.1 .1 < data.3 .1
        {
            println!("[BROADENING BOTTOM] {:?}", data);
            Some((data.0, data.1, data.2))
        } else {
            None
        }
    }

    pub fn detect_descendant_triangle(
        &self,
        data: &data_points,
        _current_price: &f64,
    ) -> Option<(point, point, point)> {
        if comp::is_equal(data.0 .1, data.2 .1)
            && data.0 .1 < data.1 .1
            && data.2 .1 < data.1 .1
            && data.2 .1 < data.3 .1
            && data.1 .1 < data.3 .1
        {
            println!("[DESCENDANT TRIANGLE] {:?}", data);
            Some((data.0, data.1, data.2))
        } else {
            None
        }
    }

    pub fn detect_head_and_shoulders(
        &self,
        data: &data_points,
        _current_price: &f64,
    ) -> Option<(point, point, point)> {
        let hs_threshold = env::var("HEAD_AND_SHOULDERS_THRESHOLD")
            .unwrap()
            .parse::<f64>()
            .unwrap();
        if data.0 .1 > data.1 .1
            && data.2 .1 > data.0 .1
            && data.2 .1 > data.4 .1
            && (data.0 .1 - data.4 .1).abs()
                <= hs_threshold * comp::average(&[data.0 .1, data.4 .1])
            && (data.1 .1 - data.3 .1).abs()
                <= hs_threshold * comp::average(&[data.0 .1, data.4 .1])
        {
            println!("[HEAD AND SHOULDERS] {:?}", data);
            Some((data.0, data.1, data.2))
        } else {
            None
        }
    }

    pub fn detect_inverse_head_and_shoulders(
        &self,
        data: &data_points,
        _current_price: &f64,
    ) -> Option<(point, point, point)> {
        let hs_threshold = env::var("HEAD_AND_SHOULDERS_THRESHOLD")
            .unwrap()
            .parse::<f64>()
            .unwrap();
        if data.0 .1 < data.1 .1
            && data.2 .1 < data.0 .1
            && data.2 .1 < data.4 .1
            && (data.0 .1 - data.4 .1).abs()
                <= hs_threshold * comp::average(&[data.0 .1, data.4 .1])
            && (data.1 .1 - data.3 .1).abs()
                <= hs_threshold * comp::average(&[data.0 .1, data.4 .1])
        {
            println!("[INVERSE HEAD AND SHOULDERS] {:?}", data);
            Some((data.0, data.1, data.2))
        } else {
            None
        }
    }

    // pub fn upper_channel(&self) -> &UpperChannel {
    //     &self.upper_channel
    // }
    // pub fn detect_upper_channel(&mut self, peaks: &Peaks) -> &UpperChannel {
    //     self.upper_channel.scan(peaks)
    // }
}

// impl<T> Default for Pattern<T> {
//     fn default() -> Self {
//         Self::new()
//     }
// }
