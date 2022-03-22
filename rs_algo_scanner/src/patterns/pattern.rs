use crate::helpers::comp::percentage_change;
use crate::patterns::*;
use crate::prices::{calculate_price_change, calculate_price_target};

use rs_algo_shared::helpers::date::Local;
pub use rs_algo_shared::models::*;

use serde::{Deserialize, Serialize};
use std::env;

type Point = (usize, f64);
pub type DataPoints = Vec<Point>;
pub type PatternActiveResult = (bool, usize, f64);

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
        close: &Vec<f64>,
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
            let mut unfinished = true;
            while unfinished {
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
                                triangle::ascendant_top_active(&data_points, close),
                            );
                        } else if triangle::is_ascendant_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::TriangleAscendant,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                triangle::ascendant_bottom_active(&data_points, close),
                            );
                        } else if triangle::is_descendant_top(&data_points) {
                            self.set_pattern(
                                PatternType::TriangleDescendant,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                triangle::descendant_top_active(&data_points, close),
                            );
                        } else if triangle::is_descendant_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::TriangleDescendant,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                triangle::descendant_bottom_active(&data_points, close),
                            );
                        } else if triangle::is_symmetrical_top(&data_points) {
                            self.set_pattern(
                                PatternType::TriangleSymmetrical,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                triangle::symetrical_top_active(&data_points, close),
                            );
                        } else if triangle::is_symmetrical_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::TriangleSymmetrical,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                triangle::symetrical_bottom_active(&data_points, close),
                            );
                        } else if rectangle::is_renctangle_top(&data_points) {
                            self.set_pattern(
                                PatternType::Rectangle,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                rectangle::rectangle_top_active(&data_points, close),
                            );
                        } else if rectangle::is_renctangle_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::Rectangle,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                rectangle::rectangle_bottom_active(&data_points, close),
                            );
                        } else if channel::is_ascendant_top(&data_points) {
                            self.set_pattern(
                                PatternType::ChannelUp,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                channel::channel_top_active(&data_points, close),
                            );
                        } else if channel::is_ascendant_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::ChannelUp,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                channel::channel_bottom_active(&data_points, close),
                            );
                        } else if channel::is_descendant_top(&data_points) {
                            self.set_pattern(
                                PatternType::ChannelDown,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                channel::channel_top_active(&data_points, close),
                            );
                        } else if channel::is_descendant_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::ChannelDown,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                channel::channel_bottom_active(&data_points, close),
                            );
                        } else if broadening::is_top(&data_points) {
                            self.set_pattern(
                                PatternType::Broadening,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                broadening::broadening_top_active(&data_points, close),
                            );
                        } else if broadening::is_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::Broadening,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                broadening::broadening_top_active(&data_points, close),
                            );
                        } else if double::is_top(&data_points) {
                            self.set_pattern(
                                PatternType::DoubleTop,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                double::top_active(&data_points, close),
                            );
                        } else if double::is_bottom(&data_points) {
                            self.set_pattern(
                                PatternType::DoubleBottom,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                double::top_active(&data_points, close),
                            );
                        } else if head_shoulders::is_hs(&data_points) {
                            self.set_pattern(
                                PatternType::HeadAndShoulders,
                                PatternDirection::Bottom,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                head_shoulders::hs_active(&data_points, close),
                            );
                        } else if head_shoulders::is_inverse(&data_points) {
                            self.set_pattern(
                                PatternType::HeadAndShoulders,
                                PatternDirection::Top,
                                &pattern_size,
                                &data_points,
                                self.calculate_change(&data_points),
                                head_shoulders::hs_active(&data_points, close),
                            );
                        }
                    }
                    None => {
                        self.set_pattern(
                            PatternType::None,
                            PatternDirection::None,
                            &pattern_size,
                            &vec![(0, 0.)],
                            0.,
                            PatternActive {
                                active: false,
                                completed: true,
                                date: Local::now(),
                                timestamp: Local::now().timestamp(),
                                target: 0.,
                                change: 0.,
                                index: 0,
                                price: 0.,
                                break_direction: PatternDirection::None,
                            },
                        );
                        unfinished = false;
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
                PatternActive {
                    active: false,
                    completed: true,
                    date: Local::now(),
                    timestamp: Local::now().timestamp(),
                    target: 0.,
                    change: 0.,
                    index: 0,
                    price: 0.,
                    break_direction: PatternDirection::None,
                },
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
        active: PatternActive,
    ) {
        if pattern_type != PatternType::None {
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

pub fn pattern_active_result(
    data: &DataPoints,
    top: PatternActiveResult,
    bottom: PatternActiveResult,
) -> PatternActive {
    let (top_result, top_id, top_price) = top;
    let (bottom_result, bottom_id, bottom_price) = bottom;
    let price_change = calculate_price_change(&data);
    let price_target = calculate_price_target(&data);
    if top_result {
        PatternActive {
            active: true,
            completed: false,
            index: top_id,
            price: top_price,
            date: Local::now(),
            timestamp: Local::now().timestamp(),
            change: price_change,
            target: price_target,
            break_direction: PatternDirection::Top,
        }
    } else if bottom_result {
        PatternActive {
            active: true,
            completed: false,
            index: bottom_id,
            date: Local::now(),
            timestamp: Local::now().timestamp(),
            price: bottom_price,
            change: price_change,
            target: price_target,
            break_direction: PatternDirection::Bottom,
        }
    } else {
        PatternActive {
            active: false,
            completed: false,
            index: 0,
            date: Local::now(),
            timestamp: Local::now().timestamp(),
            price: 0.,
            change: 0.,
            target: 0.,
            break_direction: PatternDirection::None,
        }
    }
}

// impl<T> Default for Pattern<T> {
//     fn default() -> Self {
//         Self::new()
//     }
// }
