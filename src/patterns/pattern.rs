use super::peaks::Peaks;
use crate::patterns::upper_channel::UpperChannel;

use std::cmp;
use std::env;

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

    pub fn detect_pattern(&mut self, peaks: &Peaks) {
        let max_bars = env::var("PATTERNS_MAX_BARS")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut locals = [peaks.local_maxima.clone(), peaks.local_minima.clone()].concat();
        locals.sort_by(|(a, _), (b, _)| a.cmp(b));

        println!("111111, {:?}", locals);
        for x in &peaks.smooth_highs {
            //println!("{:?}", x);
        }
    }

    pub fn upper_channel(&self) -> &UpperChannel {
        &self.upper_channel
    }
    pub fn detect_upper_channel(&mut self, peaks: &Peaks) -> &UpperChannel {
        self.upper_channel.scan(peaks)
    }
}
