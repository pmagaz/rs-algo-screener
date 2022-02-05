use super::peaks::Peaks;
use crate::helpers::comp;

use std::cmp;
use std::env;
type Point = (usize, f64);
pub type DataPoints = Vec<Point>;
use crate::patterns::triangle;

//type DataPoints = (Point, Point, Point, Point, Point);
//TODO use TRAITS

#[derive(Debug, Clone)]
pub enum PatternType {
    Default,
    DoubleTop,
    TriangleSymmetricalTop,
    TriangleSymmetricalBottom,
    TriangleDescendantTop,
    TriangleDescendantBottom,
    TriangleAscendantTop,
    TriangleAscendantBottom,
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
    pub pattern_type: PatternType,
    pub data: DataPoints,
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
            data: vec![],
        }
    }

    pub fn detect_pattern(&mut self, peaks: &Peaks, current_price: &f64) {
        let max_bars = env::var("PATTERNS_MAX_BARS")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let num_DataPoints = env::var("PATTERNS_DATA_POINTS")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut locals = [peaks.local_maxima.clone(), peaks.local_minima.clone()].concat();

        locals.sort_by(|(a, _), (b, _)| a.cmp(b));
        locals.reverse();
        let mut iter = locals.windows(num_DataPoints);
        let mut no_pattern = true;
        while no_pattern {
            match iter.next() {
                Some(window) => {
                    let data_points = window.to_vec();
                    if self.is_active_double_top(&data_points, current_price) {
                        self.pattern_type = PatternType::DoubleTopActivated;
                        // no_pattern = false;
                        // } else if self.is_active_double_bottom(&data_points, current_price) {
                        //     self.pattern_type = PatternType::DoubleBottomActivated;
                        //     //  no_pattern = false;
                        // } else if self.is_double_top(&data_points, current_price) {
                        //     self.pattern_type = PatternType::DoubleTop;
                        //     //no_pattern = false;
                        // } else if self.is_double_bottom(&data_points, current_price) {
                        //     self.pattern_type = PatternType::DoubleBottom;
                        //     // no_pattern = false;
                        // } else if triangle::is_ascendant_top(&data_points, current_price) {
                        //     self.set_pattern(&data_points, PatternType::TriangleAscendantTop);
                        //     no_pattern = false;
                        // } else if triangle::is_ascendant_bottom(&data_points, current_price) {
                        //     self.set_pattern(&data_points, PatternType::TriangleAscendantBottom);
                        //     no_pattern = false;
                        // } else if triangle::is_ascendant_top(&data_points, current_price) {
                        //     self.set_pattern(&data_points, PatternType::TriangleAscendantTop);
                        //       no_pattern = false;
                    } else if triangle::is_ascendant_bottom(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::TriangleAscendantBottom);
                        //no_pattern = false;
                    } else if triangle::is_descendant_top(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::TriangleDescendantTop);
                        //   no_pattern = false;
                    } else if triangle::is_descendant_bottom(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::TriangleDescendantBottom);
                        //   no_pattern = false;
                    } else if triangle::is_symmetrical_top(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::TriangleSymmetricalTop);
                        //  no_pattern = false;
                    } else if triangle::is_symmetrical_bottom(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::TriangleSymmetricalBottom);
                        no_pattern = false;
                    }
                    //self.is_double_bottom(&DataPoints, current_price);
                    // self.is_broadening_top(&DataPoints, current_price);
                    // self.is_broadening_bottom(&DataPoints, current_price);
                    // self.is_descendant_triangle(&DataPoints, current_price);
                    // self.is_head_and_shoulders(&DataPoints, current_price);
                    // self.is_inverse_head_and_shoulders(&DataPoints, current_price);
                }
                None => {
                    no_pattern = false;
                }
            }
        }
    }

    fn set_pattern(&mut self, data_points: &DataPoints, pattern_type: PatternType) {
        self.pattern_type = pattern_type;
        self.data = data_points.to_owned();
    }

    pub fn is_active_double_top(&mut self, data: &DataPoints, _current_price: &f64) -> bool {
        if data[0].1 < data[2].1 && comp::is_equal(data[1].1, data[3].1) {
            println!("[DOUBLE TOP ACTIVATED] {:?}", data);
            self.data = data.to_owned();
            true
        } else {
            false
        }
    }

    pub fn is_double_top(&mut self, data: &DataPoints, _current_price: &f64) -> bool {
        if comp::is_equal(data[0].1, data[2].1) && data[1].1 < data[0].1 && data[1].1 < data[2].1 {
            println!("[DOUBLE TOP] {:?}", data);
            self.data = data.to_owned();
            true
        } else {
            false
        }
    }

    pub fn is_active_double_bottom(&mut self, data: &DataPoints, _current_price: &f64) -> bool {
        if data[0].1 > data[2].1 && comp::is_equal(data[1].1, data[3].1) {
            println!("[DOUBLE BOTTOM ACTIVATED] {:?}", data);
            self.data = data.to_owned();
            true
        } else {
            false
        }
    }

    pub fn is_double_bottom(&mut self, data: &DataPoints, _current_price: &f64) -> bool {
        if comp::is_equal(data[0].1, data[2].1) && data[1].1 > data[0].1 && data[1].1 > data[2].1 {
            println!("[DOUBLE BOTTOM] {:?}", data);
            self.data = data.to_owned();
            true
        } else {
            false
        }
    }

    // pub fn is_broadening_top(
    //     &self,
    //     data: &DataPoints,
    //     _current_price: &f64,
    // ) -> Option<(Point, Point, Point)> {
    //     if data[0] .1 > data[1] .1
    //         && data[0] .1 < data[2] .1
    //         && data[2] .1 < data[4] .1
    //         && data[1] .1 > data[3] .1
    //     {
    //         println!("[BROADENING TOP] {:?}", data);
    //         Some((data[0], data[1], data[2]))
    //     } else {
    //         None
    //     }
    // }

    // pub fn is_broadening_bottom(
    //     &self,
    //     data: &DataPoints,
    //     _current_price: &f64,
    // ) -> Option<(Point, Point, Point)> {
    //     if data[0] .1 < data[1] .1
    //         && data[0] .1 > data[2] .1
    //         && data[2] .1 > data[4] .1
    //         && data[1] .1 < data[3] .1
    //     {
    //         println!("[BROADENING BOTTOM] {:?}", data);
    //         Some((data[0], data[1], data[2]))
    //     } else {
    //         None
    //     }
    // }

    // pub fn is_descendant_triangle(
    //     &self,
    //     data: &DataPoints,
    //     _current_price: &f64,
    // ) -> Option<(Point, Point, Point)> {
    //     if comp::is_equal(data[0] .1, data[2] .1)
    //         && data[0] .1 < data[1] .1
    //         && data[2] .1 < data[1] .1
    //         && data[2] .1 < data[3] .1
    //         && data[1] .1 < data[3] .1
    //     {
    //         println!("[DESCENDANT TRIANGLE] {:?}", data);
    //         Some((data[0], data[1], data[2]))
    //     } else {
    //         None
    //     }
    // }

    // pub fn is_head_and_shoulders(
    //     &self,
    //     data: &DataPoints,
    //     _current_price: &f64,
    // ) -> Option<(Point, Point, Point)> {
    //     let hs_threshold = env::var("HEAD_AND_SHOULDERS_THRESHOLD")
    //         .unwrap()
    //         .parse::<f64>()
    //         .unwrap();
    //     if data[0] .1 > data[1] .1
    //         && data[2] .1 > data[0] .1
    //         && data[2] .1 > data[4] .1
    //         && (data[0] .1 - data[4] .1).abs()
    //             <= hs_threshold * comp::average(&[data[0] .1, data[4] .1])
    //         && (data[1] .1 - data[3] .1).abs()
    //             <= hs_threshold * comp::average(&[data[0] .1, data[4] .1])
    //     {
    //         println!("[HEAD AND SHOULDERS] {:?}", data);
    //         Some((data[0], data[1], data[2]))
    //     } else {
    //         None
    //     }
    // }

    // pub fn is_inverse_head_and_shoulders(
    //     &self,
    //     data: &DataPoints,
    //     _current_price: &f64,
    // ) -> Option<(Point, Point, Point)> {
    //     let hs_threshold = env::var("HEAD_AND_SHOULDERS_THRESHOLD")
    //         .unwrap()
    //         .parse::<f64>()
    //         .unwrap();
    //     if data[0] .1 < data[1] .1
    //         && data[2] .1 < data[0] .1
    //         && data[2] .1 < data[4] .1
    //         && (data[0] .1 - data[4] .1).abs()
    //             <= hs_threshold * comp::average(&[data[0] .1, data[4] .1])
    //         && (data[1] .1 - data[3] .1).abs()
    //             <= hs_threshold * comp::average(&[data[0] .1, data[4] .1])
    //     {
    //         println!("[INVERSE HEAD AND SHOULDERS] {:?}", data);
    //         Some((data[0], data[1], data[2]))
    //     } else {
    //         None
    //     }
    // }

    // pub fn upper_channel(&self) -> &UpperChannel {
    //     &self.upper_channel
    // }
    // pub fn is_upper_channel(&mut self, peaks: &Peaks) -> &UpperChannel {
    //     self.upper_channel.scan(peaks)
    // }
}

// impl<T> Default for Pattern<T> {
//     fn default() -> Self {
//         Self::new()
//     }
// }
