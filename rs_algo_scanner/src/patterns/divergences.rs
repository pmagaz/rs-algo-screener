use crate::error::Result;
use crate::helpers::maxima_minima::maxima_minima;
use crate::indicators::{Indicator, Indicators};
use crate::patterns::highs_lows::*;
pub use rs_algo_shared::models::*;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Divergences {
    data: Vec<Divergence>,
}

impl Divergences {
    pub fn new() -> Result<Self> {
        Ok(Self { data: vec![] })
    }
    // CONTINUE HERE
    pub fn calculate(&mut self, indicators: &Indicators, patterns: &Vec<Pattern>) {
        let prominence = env::var("DIVERGENCE_MIN_PROMINENCE")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let min_distance = env::var("DIVERGENCE_PROMINENCE_MIN_DISTANCE")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let data_indicators = [
            (
                IndicatorType::Stoch,
                indicators
                    .stoch()
                    .get_data_a()
                    .into_iter()
                    .rev()
                    .take(34)
                    .map(|x| *x)
                    .collect(),
            ),
            (
                IndicatorType::Macd,
                indicators
                    .macd()
                    .get_data_a()
                    .into_iter()
                    .rev()
                    .take(34)
                    .map(|x| *x)
                    .collect(),
            ),
            // (
            //     IndicatorType::Rsi,
            //     indicators
            //         .rsi()
            //         .get_data_a()
            //         .into_iter()
            //         .rev()
            //         .take(5)
            //         .map(|x| *x)
            //         .collect(),
            // ),
        ];

        for (indicator_type, data) in data_indicators {
            let maxima = maxima_minima(&data, &data, prominence, min_distance).unwrap();
            let minima = maxima_minima(
                &data.iter().map(|x| -x).collect(),
                &data,
                prominence,
                min_distance,
            )
            .unwrap();

            self.detect_pattern(&maxima, &minima, &indicator_type, &patterns);
        }
    }

    //FIXME divergences should have id or date
    pub fn detect_pattern(
        &mut self,
        maxima: &Vec<(usize, f64)>,
        minima: &Vec<(usize, f64)>,
        indicator_type: &IndicatorType,
        patterns: &Vec<Pattern>,
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

        let last_pattern = patterns.last();
        let pattern_type = match last_pattern {
            Some(pat) => &pat.pattern_type,
            None => &PatternType::None,
        };

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
            let mut no_pattern = true;
            while no_pattern {
                match iter.next() {
                    Some(window) => {
                        let data_points = window.to_vec();

                        if pattern_type == &PatternType::ChannelDown
                            || pattern_type == &PatternType::TriangleDown
                                && (is_higher_highs_top(&data_points)
                                    || is_higher_highs_bottom(&data_points))
                        {
                            self.set_pattern(&data_points, indicator_type, DivergenceType::Bullish);
                        } else if pattern_type == &PatternType::ChannelUp
                            || pattern_type == &PatternType::TriangleUp
                                && (is_lower_highs_top(&data_points)
                                    || is_lower_highs_bottom(&data_points))
                        {
                            self.set_pattern(&data_points, indicator_type, DivergenceType::Bearish);
                        } else {
                            no_pattern = false;
                        }
                    }
                    None => {
                        self.set_pattern(&vec![(0, 0.)], indicator_type, DivergenceType::None);
                        no_pattern = false;
                    }
                }
            }
        } else {
            self.set_pattern(&vec![(0, 0.)], indicator_type, DivergenceType::None);

            //self.set_pattern(&vec![(0, 0.)], &pattern_size, 0., PatternType::None);
        }
    }

    // fn calculate_change(&self, data_points: &DataPoints) -> f64 {
    //     (data_points[4].1 - data_points[3].1).abs()
    // }

    fn set_pattern(
        &mut self,
        data_points: &DataPoints,
        indicator: &IndicatorType,
        divergence_type: DivergenceType,
    ) {
        self.data.push(Divergence {
            divergence_type,
            indicator: indicator.to_owned(),
            data: data_points.to_owned(),
        })
    }
}
