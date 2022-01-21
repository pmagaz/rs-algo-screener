use crate::error::Result;
use crate::helpers::poly::fit;
use crate::helpers::regression::kernel_regression;

use find_peaks::PeakFinder;
use std::collections::HashMap;
use std::env;

pub type HashData = HashMap<usize, f64>;

#[derive(Debug, Clone)]
pub struct Peaks {
    pub highs: Vec<f64>,
    pub close: Vec<f64>,
    pub lows: Vec<f64>,
    pub local_maxima: Vec<(usize, f64)>,
    pub smooth_highs: Vec<(usize, f64)>,
    pub smooth_lows: Vec<(usize, f64)>,
    pub local_minima: Vec<(usize, f64)>,
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

    pub fn calculate_peaks(&mut self, max_price: &f64) -> Result<()> {
        let local_prominence = env::var("LOCAL_PROMINENCE")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let kernel_bandwidth = env::var("KERNEL_BANDWIDTH")
            .unwrap()
            .parse::<f64>()
            .unwrap();
        let mut smooth_close: Vec<f64> = vec![];

        let mut candle_id = 0;
        for x in &self.close {
            let leches = kernel_regression(kernel_bandwidth, *x, &self.close);
            smooth_close.push(leches.abs());
            self.smooth_highs.push((candle_id, leches.abs()));
            candle_id += 1;
        }

        let local_prominence: f64 = max_price / local_prominence;
        let mut local_maxima_fp = PeakFinder::new(&smooth_close);
        local_maxima_fp.with_min_prominence(local_prominence);

        let mut x_values: Vec<f64> = vec![];
        let mut y_values: Vec<f64> = vec![];
        for x in local_maxima_fp.find_peaks() {
            let candle_id = x.middle_position();
            let price = self.close[candle_id];
            x_values.push(candle_id as f64);
            y_values.push(price);
            self.local_maxima.push((candle_id, price.abs()));
        }

        let leches: Vec<f64> = smooth_close.iter().map(|x| -x).collect();
        let mut local_minima_fp = PeakFinder::new(&leches);
        local_minima_fp.with_min_prominence(local_prominence);

        let mut x_values: Vec<f64> = vec![];
        let mut y_values: Vec<f64> = vec![];
        for x in local_minima_fp.find_peaks() {
            let candle_id = x.middle_position();
            let price = self.close[candle_id];
            x_values.push(candle_id as f64);
            y_values.push(price);
            self.local_minima.push((candle_id, price.abs()));
        }

        Ok(())
    }
}
