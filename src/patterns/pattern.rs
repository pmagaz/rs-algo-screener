use super::peaks::Peaks;
use std::env;
type Point = (usize, f64);
pub type DataPoints = Vec<Point>;
use crate::patterns::*;
use std::fmt::{self, Display};
//TODO use TRAITS

#[derive(Debug, Clone)]
pub enum PatternType {
    Default,
    TriangleSymmetricalTop,
    TriangleSymmetricalBottom,
    TriangleDescendantTop,
    TriangleDescendantBottom,
    TriangleAscendantTop,
    TriangleAscendantBottom,
    RectangleTop,
    RectangleBottom,
    ChannelUpTop,
    ChannelUpBottom,
    ChannelDownTop,
    ChannelDownBottom,
    BroadeningTop,
    BroadeningBottom,
    DoubleBottom,
    DoubleTop,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub pattern_type: PatternType,
    pub data_points: DataPoints,
}

#[derive(Debug, Clone)]
pub struct Patterns {
    pub patterns: Vec<Pattern>,
}

impl Patterns {
    pub fn new() -> Self {
        Patterns { patterns: vec![] }
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
                    if triangle::is_ascendant_top(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::TriangleAscendantTop);
                        no_pattern = false;
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
                    //no_pattern = false;
                    } else if rectangle::is_renctangle_top(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::RectangleTop);
                        //no_pattern = false;
                    } else if rectangle::is_renctangle_bottom(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::RectangleBottom);
                        // no_pattern = false;
                    } else if channel::is_ascendant_top(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::ChannelUpTop);
                    //no_pattern = false;
                    } else if channel::is_ascendant_bottom(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::ChannelUpBottom);
                    //no_pattern = false;
                    } else if channel::is_descendant_top(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::ChannelDownTop);
                    //no_pattern = false;
                    } else if channel::is_descendant_bottom(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::ChannelDownBottom);
                        // no_pattern = false;
                    } else if broadening::is_top(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::BroadeningTop);
                        //no_pattern = false;
                    } else if broadening::is_bottom(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::BroadeningBottom);
                        //no_pattern = false;
                    } else if double::is_top(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::DoubleTop);
                        //no_pattern = false;
                    } else if double::is_bottom(&data_points, current_price) {
                        self.set_pattern(&data_points, PatternType::DoubleBottom);
                        //no_pattern = false;
                    }
                }
                None => {
                    no_pattern = false;
                }
            }
        }
    }

    fn set_pattern(&mut self, data_points: &DataPoints, pattern_type: PatternType) {
        self.patterns.push(Pattern {
            pattern_type,
            data_points: data_points.to_owned(),
        });
    }

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
