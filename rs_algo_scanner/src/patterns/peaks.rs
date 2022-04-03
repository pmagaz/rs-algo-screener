use crate::error::Result;
use crate::helpers::maxima_minima::maxima_minima;
use crate::helpers::regression::kernel_regression;

use rs_algo_shared::helpers::comp::*;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peaks {
    pub highs: Vec<f64>,
    pub close: Vec<f64>,
    pub lows: Vec<f64>,
    pub local_maxima: Vec<(usize, f64)>,
    pub local_minima: Vec<(usize, f64)>,
    pub smooth_highs: Vec<(usize, f64)>,
    pub smooth_lows: Vec<(usize, f64)>,
    pub smooth_close: Vec<(usize, f64)>,
    pub extrema_maxima: Vec<(usize, f64)>,
    pub extrema_minima: Vec<(usize, f64)>,
}

impl Peaks {
    pub fn new() -> Peaks {
        Self {
            highs: vec![],
            close: vec![],
            lows: vec![],
            local_maxima: vec![],
            local_minima: vec![],
            smooth_highs: vec![],
            smooth_lows: vec![],
            smooth_close: vec![],
            extrema_maxima: vec![],
            extrema_minima: vec![],
        }
    }

    pub fn highs(&self) -> &Vec<f64> {
        &self.highs
    }

    pub fn lows(&self) -> &Vec<f64> {
        &self.lows
    }

    pub fn local_maxima(&self) -> &Vec<(usize, f64)> {
        &self.local_maxima
    }
    pub fn smooth_close(&self) -> &Vec<(usize, f64)> {
        &self.smooth_close
    }
    pub fn smooth_highs(&self) -> &Vec<(usize, f64)> {
        &self.smooth_highs
    }
    pub fn smooth_lows(&self) -> &Vec<(usize, f64)> {
        &self.smooth_lows
    }
    pub fn local_minima(&self) -> &Vec<(usize, f64)> {
        &self.local_minima
    }

    pub fn extrema_maxima(&self) -> &Vec<(usize, f64)> {
        &self.extrema_maxima
    }

    pub fn extrema_minima(&self) -> &Vec<(usize, f64)> {
        &self.extrema_minima
    }

    pub fn calculate_peaks(&mut self, max_price: &f64, min_price: &f64) -> Result<()> {
        let mut local_prominence = env::var("LOCAL_MIN_PROMINENCE")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let mut extrema_prominence = env::var("EXTREMA_MIN_PROMINENCE")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let min_distance = env::var("PROMINENCE_MIN_DISTANCE")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut kernel_bandwidth = env::var("KERNEL_REGRESSION_BANDWIDTH")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let kernel_source = env::var("KERNEL_REGRESSION_SOURCE").unwrap();

        kernel_bandwidth = (max_price - min_price) * kernel_bandwidth;
        local_prominence = (max_price - min_price) * local_prominence;
        extrema_prominence = (max_price - min_price) * extrema_prominence;

        let mut smooth_highs: Vec<f64> = vec![];
        let mut smooth_lows: Vec<f64> = vec![];
        let mut smooth_close: Vec<f64> = vec![];

        let mut candle_id = 0;
        for x in &self.close {
            if kernel_source == "highs_lows" {
                let smoothed_high = kernel_regression(kernel_bandwidth, *x, &self.highs);
                let smoothed_low = kernel_regression(kernel_bandwidth, *x, &self.lows);
                smooth_highs.push(smoothed_high.abs());
                smooth_lows.push(smoothed_low.abs());
                self.smooth_highs.push((candle_id, smoothed_high.abs()));
                self.smooth_lows.push((candle_id, smoothed_low.abs()));
            } else {
                let smoothed_close = kernel_regression(kernel_bandwidth, *x, &self.close);
                smooth_close.push(smoothed_close.abs());
                self.smooth_close.push((candle_id, smoothed_close.abs()));
            }

            candle_id += 1;
        }

        let source = match kernel_source.as_ref() {
            "highs_lows" => (&smooth_highs, &self.highs, &smooth_lows, &self.lows),
            "close" => (&smooth_close, &self.close, &smooth_close, &self.close),
            &_ => (&smooth_close, &smooth_close, &self.close, &self.close),
        };

        let minima_smooth: Vec<f64> = source.2.iter().map(|x| -x).collect();

        self.local_maxima = maxima_minima(source.0, source.1, local_prominence, min_distance)?;

        self.local_minima =
            maxima_minima(&minima_smooth, source.3, local_prominence, min_distance)?;

        self.extrema_maxima = maxima_minima(source.0, source.2, extrema_prominence, min_distance)?;

        self.extrema_minima =
            maxima_minima(&minima_smooth, source.3, extrema_prominence, min_distance)?;

        Ok(())
    }

    // fn maxima_minima(
    //     &self,
    //     data: &Vec<f64>,
    //     min_prominence: f64,
    //     min_distance: usize,
    //     max_price: &f64,
    // ) -> Result<Vec<(usize, f64)>> {
    //     let prominence = max_price * min_prominence;
    //     let mut result: Vec<(usize, f64)> = vec![];

    //     let peaks = PeakFinder::new(&data)
    //         .with_min_prominence(prominence)
    //         //.with_min_distance(min_distance);
    //         .find_peaks();

    //     let mut x_values: Vec<f64> = vec![];
    //     let mut y_values: Vec<f64> = vec![];

    //     for peak in peaks {
    //         let candle_id = peak.middle_position();
    //         let price = self.close[candle_id];

    //         x_values.push(candle_id as f64);
    //         y_values.push(price);
    //         result.push((candle_id, price.abs()));
    //     }

    //     Ok(result)
    // }
}
