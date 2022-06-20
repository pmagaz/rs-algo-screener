use crate::candle::Candle;
use crate::error::Result;
use crate::helpers::maxima_minima::*;
use crate::indicators::{Indicator, Indicators};
use crate::patterns::highs_lows::*;

use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::divergence::{Divergence, DivergenceType};
use rs_algo_shared::models::indicator::IndicatorType;
use rs_algo_shared::models::pattern::{DataPoints, Pattern, PatternType};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
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
    pub fn calculate(
        &mut self,
        indicators: &Indicators,
        patterns: &Vec<Pattern>,
        candles: &Vec<Candle>,
        maxima_minima: &Vec<(usize, f64)>,
    ) {
        let prominence = env::var("DIVERGENCE_MIN_PROMINENCE")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let min_distance = env::var("DIVERGENCE_PROMINENCE_MIN_DISTANCE")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let data_indicators: [(IndicatorType, &Vec<f64>); 2] = [
            (IndicatorType::Rsi, indicators.rsi().get_data_a()),
            (IndicatorType::Stoch, indicators.stoch().get_data_a()),
        ];

        for (indicator_type, indicator_value) in data_indicators {
            let mut maxima =
                maxima_minima_exp(&indicator_value, &indicator_value, prominence, min_distance)
                    .unwrap();
            maxima.sort_by(|(id_a, _indicator_value_a), (id_b, _indicator_value_b)| id_a.cmp(id_b));

            let minima = maxima_minima_exp(
                &indicator_value.iter().map(|x| -x).collect(),
                &indicator_value,
                prominence,
                min_distance,
            )
            .unwrap();
            //FIXME too many arguments
            self.compare_with_price(
                &maxima,
                &minima,
                &indicator_type,
                &patterns,
                &candles,
                &maxima_minima,
            );
        }
    }

    pub fn compare_with_price(
        &mut self,
        maxima: &Vec<(usize, f64)>,
        minima: &Vec<(usize, f64)>,
        indicator_type: &IndicatorType,
        patterns: &Vec<Pattern>,
        candles: &Vec<Candle>,
        maxima_minima: &Vec<(usize, f64)>,
    ) {
        let local_max_points = env::var("PATTERNS_MAX_POINTS")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let min_points = env::var("DIVERGENCES_MIN_POINTS")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let window_size = env::var("DIVERGENCES_WINDOW_SIZE")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let fake_date = Local::now() - Duration::days(1000);

        let last_pattern = patterns.last();

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

            //let mut locals = [&maxima[max_start..max_end], &minima[min_start..min_end]].concat();
            let mut locals = [&maxima[max_start..max_end]].concat();

            locals.sort_by(|(id_a, _indicator_value_a), (id_b, _indicator_value_b)| id_a.cmp(id_b));
            let mut iter = locals.windows(window_size);
            let mut no_pattern = true;
            while no_pattern {
                match iter.next() {
                    Some(window) => {
                        let data_points = window.to_vec();
                        let last_id = window.last().unwrap().0;
                        let candle = candles.get(last_id).unwrap();

                        let indicator_points: Vec<f64> = data_points.iter().map(|x| x.1).collect();
                        let indicator_peaks_sorted = peaks_are_sorted(&indicator_points);
                        if indicator_peaks_sorted == Ordering::Greater
                            || indicator_peaks_sorted == Ordering::Less
                        {
                            let min_id = &data_points[0].0;
                            let max_id = &data_points[1].0;
                            let maxima_points: Vec<f64> = maxima_minima
                                .iter()
                                .filter(|x| x.0 >= *min_id && x.0 <= *max_id)
                                .map(|x| x.1)
                                .collect();

                            let maxima_peaks_sorted = peaks_are_sorted(&maxima_points);

                            if maxima_points.len() > 1
                                && indicator_peaks_sorted == Ordering::Greater
                                && maxima_peaks_sorted == Ordering::Less
                            {
                                self.set_pattern(
                                    &data_points,
                                    indicator_type,
                                    DivergenceType::Bullish,
                                    candle.date(),
                                );
                            } else if maxima_points.len() > 1
                                && indicator_peaks_sorted == Ordering::Less
                                && maxima_peaks_sorted == Ordering::Greater
                            {
                                self.set_pattern(
                                    &data_points,
                                    indicator_type,
                                    DivergenceType::Bearish,
                                    candle.date(),
                                );
                            }
                        }
                    }
                    None => {
                        self.set_pattern(
                            &vec![(0, 0.)],
                            indicator_type,
                            DivergenceType::None,
                            fake_date,
                        );
                        no_pattern = false;
                    }
                }
            }
        }
        self.data
            .sort_by(|divergence_a, divergence_b| divergence_a.date.cmp(&divergence_b.date));
    }

    fn set_pattern(
        &mut self,
        data_points: &DataPoints,
        indicator: &IndicatorType,
        divergence_type: DivergenceType,
        date: DateTime<Local>,
    ) {
        self.data.push(Divergence {
            divergence_type,
            date: to_dbtime(date),
            indicator: indicator.to_owned(),
            data: data_points.to_owned(),
        })
    }
}
