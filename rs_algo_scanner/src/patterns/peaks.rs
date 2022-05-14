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

        let local_min_distance = env::var("LOCAL_PROMINENCE_MIN_DISTANCE")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let extrema_min_distance = env::var("EXTREMA_PROMINENCE_MIN_DISTANCE")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut kernel_bandwidth = env::var("KERNEL_REGRESSION_BANDWIDTH")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let kernel_source = env::var("PRICE_SOURCE").unwrap();

        let mut smooth_highs: Vec<f64> = vec![];
        let mut smooth_lows: Vec<f64> = vec![];
        let mut smooth_close: Vec<f64> = vec![];

        kernel_bandwidth = (max_price - min_price) * kernel_bandwidth;
        local_prominence = (max_price - min_price) * local_prominence;
        extrema_prominence = (max_price - min_price) * extrema_prominence;

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
        //ISSUE!
        self.local_maxima =
            maxima_minima(source.0, source.1, local_prominence, local_min_distance)?;

        self.local_minima = maxima_minima(
            &minima_smooth,
            source.3,
            local_prominence,
            local_min_distance,
        )?;

        self.extrema_maxima =
            maxima_minima(source.0, source.2, extrema_prominence, extrema_min_distance)?;

        self.extrema_minima = maxima_minima(
            &minima_smooth,
            source.3,
            extrema_prominence,
            extrema_min_distance,
        )?;

        // let data = &self.close;
        // let mut iter = self.highs.chunks(500);
        // let mut local_maxima: Vec<(usize, f64)> = vec![];
        // let mut local_minima: Vec<(usize, f64)> = vec![];
        // let leches: Vec<(usize, f64)> = vec![];
        // let mut iterating = true;
        // while iterating {
        //     match iter.next() {
        //         Some(window) => {
        //             let data_points = window.to_vec();
        //             let max_price = max_number(&data_points);
        //             let min_price = min_number(&data_points);
        //             println!(
        //                 "11111 {:?} {:?}, {:?}",
        //                 self.highs.len(),
        //                 min_price,
        //                 max_price
        //             );
        //             kernel_bandwidth = (max_price - min_price) * kernel_bandwidth;
        //             local_prominence = (max_price - min_price) * local_prominence;
        //             //extrema_prominence = (max_price - min_price) * extrema_prominence;
        //             //println!("11111 {:?} {:?}", min_price, max_price);

        //             let mut candle_id = 0;
        //             for x in &data_points {
        //                 if kernel_source == "highs_lows" {
        //                     let smoothed_high =
        //                         kernel_regression(kernel_bandwidth, *x, &data_points);
        //                     let smoothed_low =
        //                         kernel_regression(kernel_bandwidth, *x, &data_points);
        //                     smooth_highs.push(smoothed_high.abs());
        //                     smooth_lows.push(smoothed_low.abs());
        //                     self.smooth_highs.push((candle_id, smoothed_high.abs()));
        //                     self.smooth_lows.push((candle_id, smoothed_low.abs()));
        //                 } else {
        //                     let smoothed_close =
        //                         kernel_regression(kernel_bandwidth, *x, &data_points);
        //                     smooth_close.push(smoothed_close.abs());
        //                     self.smooth_close.push((candle_id, smoothed_close.abs()));
        //                 }

        //                 candle_id += 1;
        //             }

        //             let source = match kernel_source.as_ref() {
        //                 "highs_lows" => (&smooth_highs, &self.highs, &smooth_lows, &self.lows),
        //                 "close" => (&smooth_close, &data_points, &smooth_close, &data_points),
        //                 &_ => (&smooth_close, &smooth_close, &self.close, &self.close),
        //             };

        //             let minima_smooth: Vec<f64> = source.2.iter().map(|x| -x).collect();

        //             let mut max =
        //                 maxima_minima(source.0, source.1, local_prominence, local_min_distance)
        //                     .unwrap();
        //             let mut min = maxima_minima(
        //                 &minima_smooth,
        //                 source.3,
        //                 local_prominence,
        //                 local_min_distance,
        //             )
        //             .unwrap();
        //             local_maxima.append(&mut max);
        //             local_minima.append(&mut min);
        //             println!("2222222 {:?}", max.len());

        //             // [local_maxima.clone(), max].concat();
        //             // [local_minima.clone(), min].concat();
        //         }
        //         None => {
        //             iterating = false;
        //         }
        //     }
        // }
        // println!("4444444");
        // iterating = false;

        // self.local_maxima = local_maxima;
        // self.local_minima = local_minima;
        // println!("88888 {:?}", self.local_maxima().len());

        Ok(())
    }
}
