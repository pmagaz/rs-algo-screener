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
        let extrema_prominence = env::var("EXTREMA_PROMINENCE")
            .unwrap()
            .parse::<f64>()
            .unwrap();
        let local_prominence: f64 = max_price / local_prominence;
        let extrema_prominence: f64 = max_price / extrema_prominence;
        let mut local_maxima_fp = PeakFinder::new(&self.highs);
        local_maxima_fp.with_min_prominence(local_prominence);

        let mut x_values: Vec<f64> = vec![];
        let mut y_values: Vec<f64> = vec![];
        for x in local_maxima_fp.find_peaks() {
            let candle_id = x.middle_position();
            let price = self.highs[candle_id];
            x_values.push(candle_id as f64);
            y_values.push(price);
            self.local_maxima.push((candle_id, price.abs()));
        }

        //CONTINUE HERE
        let mut candle_id = 0;
        for x in &self.highs {
            let leches = kernel_regression(1., *x, &self.highs);
            self.smooth_highs.push((candle_id, leches.abs()));
            candle_id += 1;
        }

        // let mut candle_id = 0;
        // for x in &self.lows {
        //     let leches = kernel_regression(1., *x, &self.lows);
        //     self.smooth_lows.push((candle_id, leches.abs()));
        //     candle_id += 1;
        // }

        let mut local_minima_fp = PeakFinder::new(&self.lows);
        local_minima_fp.with_min_prominence(local_prominence);

        let mut x_values: Vec<f64> = vec![];
        let mut y_values: Vec<f64> = vec![];
        for x in local_minima_fp.find_peaks() {
            let candle_id = x.middle_position();
            let price = self.lows[candle_id];
            x_values.push(candle_id as f64);
            y_values.push(price);
            self.local_minima.push((candle_id, price.abs()));
        }

        self.local_minima = fit(&x_values, &y_values, 7);
        let mut extrema_maxima_fp = PeakFinder::new(&self.highs);
        extrema_maxima_fp.with_min_prominence(extrema_prominence);

        for x in extrema_maxima_fp.find_peaks() {
            let candle_id = x.middle_position();
            let price = self.highs[candle_id];
            self.extrema_maxima.push((candle_id, price));
        }

        let mut extrema_minima_fp = PeakFinder::new(&self.lows);
        extrema_minima_fp.with_min_prominence(extrema_prominence);

        for x in extrema_minima_fp.find_peaks() {
            let candle_id = x.middle_position();
            let price = self.lows[candle_id];
            self.extrema_minima.push((candle_id, price.abs()));
        }

        Ok(())
    }
}
