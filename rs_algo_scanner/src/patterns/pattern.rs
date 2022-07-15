use crate::candle::Candle;
use crate::helpers::poly::poly_fit;
use crate::helpers::slope_intercept::{add_next_bottom_points, add_next_top_points};
use crate::patterns::*;
use crate::prices::{calculate_price_change, calculate_price_target};

use rs_algo_shared::helpers::comp::percentage_change;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::pattern::*;
use rs_algo_shared::models::status::Status;

use serde::{Deserialize, Serialize};
use std::env;

pub type PatternActiveResult = (bool, usize, f64, DbDateTime);

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

    pub fn detect_pattern(
        &mut self,
        pattern_size: PatternSize,
        maxima: &Vec<(usize, f64)>,
        minima: &Vec<(usize, f64)>,
        candles: &Vec<Candle>,
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
            //locals.reverse();

            let mut iter = locals.windows(window_size);
            let mut not_founded = true;

            while not_founded {
                match iter.next() {
                    Some(window) => {
                        let mut data_points = window.to_vec();
                        //Rectangle
                        //let data_points = vec![(1, 100.), (1, 75.), (1, 99.), (1, 74.), (1, 99.9)];
                        //Triangle descendant
                        //let data_points = vec![(1, 100.), (1, 75.), (1, 90.), (1, 75.), (1, 85.)];
                        //Triangle Ascenandt
                        //let data_points = vec![(1, 100.), (1, 75.), (1, 99.), (1, 88.), (1, 100.)];
                        //Channel up
                        //let data_points =
                        // vec![(1, 285.), (1, 189.), (1, 306.), (1, 201.), (1, 329.)];
                        //Channel Down Top
                        // let mut data_points =
                        //     vec![(3, 174.), (4, 133.), (5, 164.), (6, 121.), (500, 155.)];
                        // //Channel Down Bottom
                        // // let data_points = vec![
                        // //     (3, 143.),
                        // //     (3, 174.),
                        // //     (4, 133.),
                        // //     (5, 164.),
                        // //     (6, 121.),
                        // //     (5, 155.),
                        // // ];
                        // data_points.reverse();
                        //Broadening
                        // let mut data_points =
                        //     vec![(1, 100.), (1, 90.), (1, 120.), (1, 80.), (1, 130.)];
                        //Triangle symetrical
                        // let mut data_points =
                        //     vec![(1, 100.), (1, 90.), (1, 120.), (1, 80.), (1, 130.)];
                        // data_points.reverse();
                        //Double Bottom
                        // let mut data_points =
                        //     vec![(1, 100.), (1, 80.), (1, 90.), (1, 79.), (1, 99.)];
                        let last_index = data_points.last().unwrap().0;
                        let candle_date = candles.get(last_index).unwrap().date();

                        let change = self.calculate_change(&data_points);
                        if rectangle::is_renctangle_top(&data_points) {
                            data_points = add_next_top_points(data_points);

                            let is_pattern_active = rectangle::rectangle_top_active(
                                &data_points,
                                candles,
                                PatternType::Rectangle,
                            );

                            self.set_pattern(
                                PatternType::Rectangle,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if rectangle::is_renctangle_bottom(&data_points) {
                            data_points = add_next_bottom_points(data_points);

                            let is_pattern_active = rectangle::rectangle_bottom_active(
                                &data_points,
                                candles,
                                PatternType::Rectangle,
                            );

                            self.set_pattern(
                                PatternType::Rectangle,
                                PatternDirection::Bottom,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if double::is_top(&data_points) {
                            data_points = add_next_top_points(data_points);

                            let is_pattern_active =
                                double::top_active(&data_points, candles, PatternType::DoubleTop);

                            self.set_pattern(
                                PatternType::DoubleTop,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if double::is_bottom(&data_points) {
                            data_points = add_next_bottom_points(data_points);

                            let is_pattern_active = double::top_active(
                                &data_points,
                                candles,
                                PatternType::DoubleBottom,
                            );

                            self.set_pattern(
                                PatternType::DoubleBottom,
                                PatternDirection::Bottom,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if channel::is_ascendant_top(&data_points) {
                            data_points = add_next_top_points(data_points);

                            let is_pattern_active = channel::channel_ascendant_top_active(
                                &data_points,
                                candles,
                                PatternType::ChannelUp,
                            );

                            self.set_pattern(
                                PatternType::ChannelUp,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if channel::is_ascendant_bottom(&data_points) {
                            data_points = add_next_bottom_points(data_points);

                            let is_pattern_active = channel::channel_ascendant_bottom_active(
                                &data_points,
                                candles,
                                PatternType::ChannelUp,
                            );

                            self.set_pattern(
                                PatternType::ChannelUp,
                                PatternDirection::Bottom,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if triangle::is_ascendant_top(&data_points) {
                            data_points = add_next_top_points(data_points);

                            let is_pattern_active = triangle::ascendant_top_active(
                                &data_points,
                                candles,
                                PatternType::TriangleUp,
                            );

                            self.set_pattern(
                                PatternType::TriangleUp,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if triangle::is_ascendant_bottom(&data_points) {
                            data_points = add_next_bottom_points(data_points);

                            let is_pattern_active = triangle::ascendant_bottom_active(
                                &data_points,
                                candles,
                                PatternType::TriangleUp,
                            );

                            self.set_pattern(
                                PatternType::TriangleUp,
                                PatternDirection::Bottom,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if triangle::is_descendant_top(&data_points) {
                            data_points = add_next_top_points(data_points);

                            let is_pattern_active = triangle::descendant_top_active(
                                &data_points,
                                candles,
                                PatternType::TriangleDown,
                            );

                            self.set_pattern(
                                PatternType::TriangleDown,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if triangle::is_descendant_bottom(&data_points) {
                            data_points = add_next_bottom_points(data_points);

                            let is_pattern_active = triangle::descendant_bottom_active(
                                &data_points,
                                candles,
                                PatternType::TriangleDown,
                            );

                            self.set_pattern(
                                PatternType::TriangleDown,
                                PatternDirection::Bottom,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if channel::is_descendant_top(&data_points) {
                            data_points = add_next_top_points(data_points);

                            let is_pattern_active = channel::channel_descendant_top_active(
                                &data_points,
                                candles,
                                PatternType::ChannelDown,
                            );

                            self.set_pattern(
                                PatternType::ChannelDown,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if channel::is_descendant_bottom(&data_points) {
                            data_points = add_next_bottom_points(data_points);

                            let is_pattern_active = channel::channel_descendant_bottom_active(
                                &data_points,
                                candles,
                                PatternType::ChannelDown,
                            );

                            self.set_pattern(
                                PatternType::ChannelDown,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if broadening::is_top(&data_points) {
                            data_points = add_next_top_points(data_points);

                            let is_pattern_active = broadening::broadening_top_active(
                                &data_points,
                                candles,
                                PatternType::Broadening,
                            );

                            self.set_pattern(
                                PatternType::Broadening,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if triangle::is_symmetrical_top(&data_points) {
                            data_points = add_next_top_points(data_points);

                            let is_pattern_active = triangle::symetrical_top_active(
                                &data_points,
                                candles,
                                PatternType::TriangleSym,
                            );

                            self.set_pattern(
                                PatternType::TriangleSym,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if triangle::is_symmetrical_bottom(&data_points) {
                            data_points = add_next_bottom_points(data_points);

                            let is_pattern_active = triangle::symetrical_bottom_active(
                                &data_points,
                                candles,
                                PatternType::TriangleSym,
                            );

                            self.set_pattern(
                                PatternType::TriangleSym,
                                PatternDirection::Bottom,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if broadening::is_bottom(&data_points) {
                            data_points = add_next_bottom_points(data_points);

                            let is_pattern_active = broadening::broadening_top_active(
                                &data_points,
                                candles,
                                PatternType::Broadening,
                            );

                            self.set_pattern(
                                PatternType::Broadening,
                                PatternDirection::Bottom,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                is_pattern_active,
                            );
                            not_founded = true;
                        } else if highs_lows::is_higher_highs_higher_lows_top(&data_points) {
                            data_points = add_next_top_points(data_points);
                            self.set_pattern(
                                PatternType::HigherHighsHigherLows,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                non_activated(),
                            );
                            not_founded = true;
                        } else if highs_lows::is_higher_highs_higher_lows_bottom(&data_points) {
                            data_points = add_next_top_points(data_points);
                            self.set_pattern(
                                PatternType::HigherHighsHigherLows,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                non_activated(),
                            );
                            not_founded = true;
                        } else if highs_lows::is_lower_highs_lower_lows_top(&data_points) {
                            data_points = add_next_top_points(data_points);
                            self.set_pattern(
                                PatternType::LowerHighsLowerLows,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                non_activated(),
                            );
                            not_founded = true;
                        } else if highs_lows::is_lower_highs_lower_lows_bottom(&data_points) {
                            data_points = add_next_top_points(data_points);
                            self.set_pattern(
                                PatternType::LowerHighsLowerLows,
                                PatternDirection::Top,
                                &pattern_size,
                                data_points.to_owned(),
                                change,
                                candle_date,
                                non_activated(),
                            );
                            not_founded = true;
                        }
                        // } else if head_shoulders::is_hs(&data_points) {
                        //     data_points = add_next_top_points(data_points);
                        //     self.set_pattern(
                        //         PatternType::HeadShoulders,
                        //         PatternDirection::Bottom,
                        //         &pattern_size,
                        //         data_points.to_owned(),
                        //         change,
                        //         candle_date,
                        //         head_shoulders::hs_active(
                        //             &data_points,
                        //             candles,
                        //             PatternType::HeadShoulders,
                        //         ),
                        //     );
                        //     not_founded = true;
                        // } else if head_shoulders::is_inverse(&data_points) {
                        //     data_points = add_next_top_points(data_points);
                        //     self.set_pattern(
                        //         PatternType::HeadShoulders,
                        //         PatternDirection::Top,
                        //         &pattern_size,
                        //         data_points.to_owned(),
                        //         change,
                        //         candle_date,
                        //         head_shoulders::hs_active(
                        //             &data_points,
                        //             candles,
                        //             PatternType::HeadShoulders,
                        //         ),
                        //     );
                        //     not_founded = true;
                        // }
                    }
                    None => {
                        let date = Local::now() - Duration::days(1000);
                        self.set_pattern(
                            PatternType::None,
                            PatternDirection::None,
                            &pattern_size,
                            vec![(0, 0.)],
                            0.,
                            date,
                            PatternActive {
                                active: false,
                                completed: true,
                                status: Status::Default,
                                date: to_dbtime(date),
                                target: 0.,
                                change: 0.,
                                index: 0,
                                price: 0.,
                                break_direction: PatternDirection::None,
                            },
                        );
                        not_founded = false;
                    }
                }
            }
        }
    }

    fn calculate_change(&self, data_points: &DataPoints) -> f64 {
        percentage_change(data_points[0].1, data_points[1].1).abs()
    }

    //FXIME too many arguments
    //TOO complex I can't barely understand it after a while XD
    fn set_pattern(
        &mut self,
        pattern_type: PatternType,
        direction: PatternDirection,
        pattern_size: &PatternSize,
        mut data_points: DataPoints,
        change: f64,
        date: DateTime<Local>,
        active: PatternActive,
    ) {
        let len = data_points.len();
        if len > 3 {
            let index = data_points.get(data_points.len() - 2).unwrap().0;
            if pattern_type != PatternType::None {
                let x_values_top: Vec<f64> = data_points
                    .iter()
                    .enumerate()
                    .filter(|(key, x)| key % 2 == 0)
                    .map(|(key, x)| x.0 as f64)
                    .collect();

                let y_values_top: Vec<f64> = data_points
                    .iter()
                    .enumerate()
                    .filter(|(key, x)| key % 2 == 0)
                    .map(|(key, x)| x.1)
                    .collect();

                let x_values_bottom: Vec<f64> = data_points
                    .iter()
                    .enumerate()
                    .filter(|(key, x)| key % 2 != 0)
                    .map(|(key, x)| x.0 as f64)
                    .collect();

                let y_values_bottom: Vec<f64> = data_points
                    .iter()
                    .enumerate()
                    .filter(|(key, x)| key % 2 != 0)
                    .map(|(key, x)| x.1)
                    .collect();

                let polynomial_top = poly_fit(&x_values_top, &y_values_top, 1);
                let polynomial_bottom = poly_fit(&x_values_bottom, &y_values_bottom, 1);
                let top_len = polynomial_top.len();
                let bottom_len = polynomial_bottom.len();

                // CONTINUE
                // let mut poly_points = vec![
                //     (x_values_top[0] as usize, y_values_top[0]),
                //     (x_values_bottom[0] as usize, y_values_bottom[0]),
                //     (x_values_top[1] as usize, y_values_top[1]),
                //     (x_values_bottom[1] as usize, y_values_bottom[1]),
                //     (x_values_top[2] as usize, y_values_top[2]),
                //     (x_values_bottom[2] as usize, y_values_bottom[2]),
                //     (x_values_top[3] as usize, y_values_top[3]),
                // ];
                let mut poly_points = match direction {
                    PatternDirection::Top => [
                        &polynomial_top[0..top_len],
                        &polynomial_bottom[0..bottom_len],
                    ]
                    .concat(),
                    PatternDirection::Bottom => [
                        &polynomial_top[0..top_len],
                        &polynomial_bottom[0..bottom_len],
                    ]
                    .concat(),
                    PatternDirection::None => [
                        &polynomial_top[0..top_len],
                        &polynomial_bottom[0..bottom_len],
                    ]
                    .concat(),
                };

                poly_points.sort_by(|(id_a, _price_a), (id_b, _price_b)| id_a.cmp(id_b));
                //data_points = poly_points;
                match &pattern_size {
                    PatternSize::Local => self.local_patterns.push(Pattern {
                        pattern_type,
                        change,
                        index,
                        date: to_dbtime(date),
                        direction,
                        active,
                        pattern_size: pattern_size.clone(),
                        data_points: data_points,
                    }),
                    PatternSize::Extrema => self.extrema_patterns.push(Pattern {
                        pattern_type,
                        change,
                        index,
                        date: to_dbtime(date),
                        direction,
                        active,
                        pattern_size: pattern_size.clone(),
                        data_points: data_points,
                    }),
                };
            }
        }
    }
}

pub fn pattern_active_result(
    data: &DataPoints,
    top: PatternActiveResult,
    bottom: PatternActiveResult,
) -> PatternActive {
    let (top_result, top_id, top_price, top_date) = top;
    let (bottom_result, bottom_id, bottom_price, bottom_date) = bottom;

    let price_change = calculate_price_change(&data);
    //FIXME pattern direction
    let price_target = calculate_price_target(&data);
    let fake_date = Local::now() - Duration::days(10000);
    if top_result {
        PatternActive {
            active: true,
            completed: false,
            status: Status::Default,
            index: top_id,
            price: top_price,
            date: top_date,
            change: price_change,
            target: price_target,
            break_direction: PatternDirection::Top,
        }
    } else if bottom_result {
        PatternActive {
            active: true,
            completed: false,
            status: Status::Default,
            index: bottom_id,
            date: bottom_date,
            price: bottom_price,
            change: price_change,
            target: price_target,
            break_direction: PatternDirection::Bottom,
        }
    } else {
        PatternActive {
            active: false,
            completed: false,
            status: Status::Default,
            index: 0,
            date: to_dbtime(fake_date),
            price: 0.,
            change: 0.,
            target: 0.,
            break_direction: PatternDirection::None,
        }
    }
}

fn non_activated() -> PatternActive {
    PatternActive {
        active: false,
        completed: false,
        status: Status::Default,
        index: 0,
        date: to_dbtime(Local::now() - Duration::days(10000)),
        price: 0.,
        change: 0.,
        target: 0.,
        break_direction: PatternDirection::None,
    }
}
