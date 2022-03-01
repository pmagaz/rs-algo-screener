use crate::helpers::comp::percentage_change;
use crate::patterns::*;
pub use rs_algo_shared::models::*;

use serde::{Deserialize, Serialize};
use std::env;

type Point = (usize, f64);
pub type DataPoints = Vec<Point>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patterns {
    pub local_patterns: Vec<Pattern>,
    pub extrema_patterns: Vec<Pattern>,
}

impl Patterns {
    pub fn new() -> Self {
        Patterns {
            local_patterns: vec![],
            extrema_patterns: vec![],
        }
    }

    pub fn patterns(&self) -> &Self {
        self
    }

    pub fn detect_pattern(
        &mut self,
        pattern_size: PatternSize,
        maxima: &Vec<(usize, f64)>,
        minima: &Vec<(usize, f64)>,
        current_price: &f64,
    ) {
        let local_max_points = env::var("PATTERNS_MAX_POINTS")
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

        let mut max_start = 0;
        let mut max_end = 0;
        let mut min_start = 0;
        let mut min_end = 0;
        let maxima_length = maxima.len();
        let minima_length = minima.len();
        if maxima_length >= min_points && minima_length >= min_points {
            if maxima_length > local_max_points {
                max_start = maxima_length - local_max_points;
                max_end = maxima_length;
            } else {
                max_start = 0;
                max_end = maxima_length;
            }

            if minima_length > local_max_points {
                min_start = minima_length - local_max_points;
                min_end = minima_length;
            } else {
                min_start = 0;
                min_end = minima_length;
            }

            let mut locals = [&maxima[max_start..max_end], &minima[min_start..min_end]].concat();

            locals.sort_by(|(id_a, _price_a), (id_b, _price_b)| id_a.cmp(id_b));
            locals.reverse();
            let mut iter = locals.windows(window_size);
            let mut no_pattern = true;
            while no_pattern {
                match iter.next() {
                    Some(window) => {
                        let data_points = window.to_vec();
                        if triangle::is_ascendant_top(&data_points) {
                            self.set_pattern(
                                PatternType::TriangleAscendant,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                triangle::ascendant_top_active(&data_points, current_price),
                            );
                            //no_pattern = false;
                        } else if triangle::is_ascendant_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::TriangleAscendant,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                triangle::ascendant_bottom_active(&data_points, current_price),
                            );
                            //no_pattern = false;
                        } else if triangle::is_descendant_top(&data_points) {
                            self.set_pattern(
                                PatternType::TriangleDescendant,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                triangle::descendant_top_active(&data_points, current_price),
                            );
                            // no_pattern = false;
                        } else if triangle::is_descendant_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::TriangleDescendant,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                triangle::descendant_bottom_active(&data_points, current_price),
                            );
                            // no_pattern = false;
                        } else if triangle::is_symmetrical_top(&data_points) {
                            self.set_pattern(
                                PatternType::TriangleSymmetrical,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                false,
                            );
                            // no_pattern = false;
                        } else if triangle::is_symmetrical_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::TriangleSymmetrical,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                false,
                            );
                            // no_pattern = false;
                        } else if rectangle::is_renctangle_top(&data_points) {
                            self.set_pattern(
                                PatternType::Rectangle,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                rectangle::rectangle_top_active(&data_points, current_price),
                            );
                            // no_pattern = false;
                        } else if rectangle::is_renctangle_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::Rectangle,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                rectangle::rectangle_bottom_active(&data_points, current_price),
                            );
                            //  no_pattern = false;
                        } else if channel::is_ascendant_top(&data_points) {
                            self.set_pattern(
                                PatternType::ChannelUp,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                false,
                            );
                            //no_pattern = false;
                        } else if channel::is_ascendant_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::ChannelUp,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                false,
                            );
                            // no_pattern = false;
                        } else if channel::is_descendant_top(&data_points) {
                            self.set_pattern(
                                PatternType::ChannelDown,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                false,
                            );
                            //  no_pattern = false;
                        } else if channel::is_descendant_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::ChannelDown,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                false,
                            );
                            // no_pattern = false;
                        } else if broadening::is_top(&data_points) {
                            self.set_pattern(
                                PatternType::Broadening,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                false,
                            );
                            // no_pattern = false;
                        } else if broadening::is_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::Broadening,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                false,
                            );
                            // no_pattern = false;
                        } else if double::is_top(&data_points) {
                            self.set_pattern(
                                PatternType::DoubleTop,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                double::top_active(&data_points, current_price),
                            );
                            // no_pattern = false;
                        } else if double::is_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::DoubleBottom,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                double::top_active(&data_points, current_price),
                            );
                            // no_pattern = false;
                        }
                    }
                    None => {
                        self.set_pattern(
                            PatternType::None,
                            PatternDirection::None,
                            &pattern_size,
                            &vec![(0, 0.)],
                            0.,
                            false,
                        );
                        no_pattern = false;
                    }
                }
            }
        } else {
            self.set_pattern(
                PatternType::None,
                PatternDirection::None,
                &pattern_size,
                &vec![(0, 0.)],
                0.,
                false,
            );
        }
    }

    fn calculate_change(&self, data_points: &DataPoints) -> f64 {
        percentage_change(data_points[4].1, data_points[3].1).abs()
    }

    fn set_pattern(
        &mut self,
        pattern_type: PatternType,
        direction: PatternDirection,
        pattern_size: &PatternSize,
        data_points: &DataPoints,
        change: f64,
        active: bool,
    ) {
        match &pattern_size {
            PatternSize::Local => self.local_patterns.push(Pattern {
                pattern_type,
                change,
                direction,
                active,
                pattern_size: pattern_size.clone(),
                data_points: data_points.to_owned(),
            }),
            PatternSize::Extrema => self.extrema_patterns.push(Pattern {
                pattern_type,
                change,
                direction,
                active,
                pattern_size: pattern_size.clone(),
                data_points: data_points.to_owned(),
            }),
        };
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
