use crate::patterns::upper_channel::UpperChannel;
use super::peaks::Peaks;

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

  pub fn upper_channel(&self) -> &UpperChannel {
    &self.upper_channel
  }
  pub fn detect_upper_channel(&mut self, peaks: &Peaks) -> &UpperChannel {
    self.upper_channel.scan(peaks)
  }
}
