use super::peaks::Peaks;
use crate::helpers::comp::is_equal;
use crate::patterns::upper_channel::UpperChannel;

use std::cmp;
use std::env;
type point = (usize, f64);
type data_points = (point, point, point, point, point);
//TODO use TRAITS

#[derive(Debug, Clone)]
pub enum PatternType {
    UpperChannel,
    AscendentTriangel,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pattern_type: PatternType,
}

#[derive(Debug, Clone)]
pub struct Patterns {
    patterns: Vec<Pattern>,
    upper_channel: UpperChannel,
}

impl Patterns {
    pub fn new() -> Self {
        Patterns {
            patterns: vec![],
            upper_channel: UpperChannel::new(),
        }
    }

    pub fn detect_pattern(&mut self, peaks: &Peaks, current_price: &f64) {
        let max_bars = env::var("PATTERNS_MAX_BARS")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let num_data_points = env::var("PATTERNS_DATA_POINTS")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut locals = [peaks.local_maxima.clone(), peaks.local_minima.clone()].concat();

        locals.sort_by(|(a, _), (b, _)| a.cmp(b));
        locals.reverse();
        let mut iter = locals.windows(num_data_points);
        let mut leches = true;
        while leches {
            match iter.next() {
                Some(x) => {
                    let data_points = self.get_data_points(x);
                    self.double_top(&data_points, current_price);
                }
                None => {
                    leches = false;
                }
            }
        }
    }
    pub fn get_data_points(&self, data: &[(usize, f64)]) -> data_points {
        let e1 = data[0];
        let e2 = data[1];
        let e3 = data[2];
        let e4 = data[3];
        let e5 = data[4];
        (e1, e2, e3, e4, e5)
    }

    pub fn double_top(
        &self,
        data: &data_points,
        _current_price: &f64,
    ) -> Option<(point, point, point)> {
        if is_equal(data.0 .1, data.2 .1) && data.1 .1 < data.0 .1 && data.1 .1 < data.2 .1 {
            println!("[DOUBLE TOP] {:?}", data);
            Some((data.0, data.1, data.2))
        } else {
            None
        }
    }

    pub fn upper_channel(&self) -> &UpperChannel {
        &self.upper_channel
    }
    pub fn detect_upper_channel(&mut self, peaks: &Peaks) -> &UpperChannel {
        self.upper_channel.scan(peaks)
    }
}
