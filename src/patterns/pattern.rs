type Point = (usize, f64);
pub type DataPoints = Vec<Point>;
use crate::patterns::*;

use std::env;

#[derive(Debug, Clone)]
pub enum PatternType {
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
    None,
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

    pub fn detect_pattern(
        &mut self,
        maxima: &Vec<(usize, f64)>,
        minima: &Vec<(usize, f64)>,
        current_price: &f64,
    ) {
        let max_points = env::var("PATTERNS_MAX_POINTS")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let min_points = env::var("PATTERNS_MIN_POINTS")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let window_size = env::var("PATTERNS_WINDOW_SIZE")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut start = 0;
        let mut max = 0;
        let mut min = 0;
        let maxima_length = maxima.len();
        let minima_length = minima.len();

        if maxima_length >= min_points && minima_length >= min_points {
            if maxima_length > max_points {
                max = max_points
            } else {
                max = maxima_length
            }

            if minima_length > min_points {
                min = min_points
            } else {
                min = minima_length
            }

            // if maxima_length < max_points {
            //     start = maxima_length
            // } else {
            //     start = maxima_length - max_points;
            // }
            // CONTINUE HERE SHOULD START max points from the end
            //println!("444444 {:?}", [&maxima[start..max]].concat());
            //let mut locals = [maxima.clone(), minima.clone()].concat();
            // let mut locals = [maxima.clone(), minima.clone()].concat();
            let mut locals = [&maxima[0..max], &minima[0..min]].concat();

            locals.sort_by(|(id_a, _price_a), (id_b, _price_b)| id_a.cmp(id_b));
            locals.reverse();
            let mut iter = locals.windows(window_size);
            let mut no_pattern = true;
            while no_pattern {
                match iter.next() {
                    Some(window) => {
                        let data_points = window.to_vec();
                        if triangle::is_ascendant_top(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::TriangleAscendantTop);
                            //no_pattern = false;
                        } else if triangle::is_ascendant_bottom(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::TriangleAscendantBottom);
                            //no_pattern = false;
                        } else if triangle::is_descendant_top(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::TriangleDescendantTop);
                            // no_pattern = false;
                        } else if triangle::is_descendant_bottom(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::TriangleDescendantBottom);
                            // no_pattern = false;
                        } else if triangle::is_symmetrical_top(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::TriangleSymmetricalTop);
                            // no_pattern = false;
                        } else if triangle::is_symmetrical_bottom(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::TriangleSymmetricalBottom);
                            // no_pattern = false;
                        } else if rectangle::is_renctangle_top(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::RectangleTop);
                            // no_pattern = false;
                        } else if rectangle::is_renctangle_bottom(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::RectangleBottom);
                            //  no_pattern = false;
                        } else if channel::is_ascendant_top(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::ChannelUpTop);
                            //no_pattern = false;
                        } else if channel::is_ascendant_bottom(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::ChannelUpBottom);
                            // no_pattern = false;
                        } else if channel::is_descendant_top(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::ChannelDownTop);
                            //  no_pattern = false;
                        } else if channel::is_descendant_bottom(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::ChannelDownBottom);
                            // no_pattern = false;
                        } else if broadening::is_top(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::BroadeningTop);
                            // no_pattern = false;
                        } else if broadening::is_bottom(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::BroadeningBottom);
                            // no_pattern = false;
                        } else if double::is_top(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::DoubleTop);
                            // no_pattern = false;
                        } else if double::is_bottom(&data_points, current_price) {
                            self.set_pattern(&data_points, PatternType::DoubleBottom);
                            // no_pattern = false;
                        }
                    }
                    None => {
                        self.set_pattern(&vec![(0, 0.)], PatternType::None);
                        no_pattern = false;
                    }
                }
            }
        } else {
            self.set_pattern(&vec![(0, 0.)], PatternType::None);
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
