use crate::candle::Candle;
use crate::error::{Result, RsAlgoError, RsAlgoErrorKind};
use crate::helpers::date::{DateTime, Local};
use crate::indicators::macd::Macd;
use crate::indicators::{Indicator, Indicators};
use crate::patterns::horizontal_levels::HorizontalLevels;
use crate::patterns::pattern::Patterns;
use crate::patterns::peaks::Peaks;

use std::env;

#[derive(Debug, Clone)]
pub struct Instrument {
    ticker: String,
    data: Vec<Candle>,
    current_price: f64,
    min_price: f64,
    max_price: f64,
    peaks: Peaks,
    horizontal_levels: HorizontalLevels,
    patterns: Patterns,
    indicators: Indicators,
}

impl Instrument {
    pub fn new() -> InstrumentBuilder {
        InstrumentBuilder::new()
    }

    pub fn ticker(&self) -> &str {
        &self.ticker
    }

    pub fn indicators(&self) -> &Indicators {
        &self.indicators
    }

    pub fn data(&self) -> &Vec<Candle> {
        &self.data
    }

    pub fn set_current_price(&mut self, current_price: f64) -> f64 {
        self.current_price = current_price;
        self.current_price
    }

    pub fn current_price(&self) -> f64 {
        self.current_price
    }

    pub fn min_price(&self) -> f64 {
        self.min_price
    }

    pub fn max_price(&self) -> f64 {
        self.max_price
    }

    pub fn peaks(&self) -> &Peaks {
        &self.peaks
    }
    pub fn patterns(&self) -> &Patterns {
        &self.patterns
    }
    pub fn horizontal_levels(&self) -> &HorizontalLevels {
        &self.horizontal_levels
    }
    pub fn set_data(
        &mut self,
        data: Vec<(DateTime<Local>, f64, f64, f64, f64, f64)>,
    ) -> Result<()> {
        let parsed: Vec<Candle> = data
            .iter()
            .enumerate()
            .map(|(id, x)| {
                let date = x.0;
                let open = x.1;
                let high = x.2;
                let low = x.3;
                let close = x.4;
                let volume = x.4;

                if self.min_price == -100. {
                    self.min_price = low;
                }
                if low < self.min_price {
                    self.min_price = low;
                }
                if self.max_price == -100. {
                    self.max_price = high;
                }
                if high > self.max_price {
                    self.max_price = high;
                }
                self.peaks.highs.push(high);
                self.peaks.lows.push(low);
                self.peaks.close.push(close);
                let key = match id {
                    0 => id,
                    _ => id - 1,
                };

                self.indicators.macd.next(close).unwrap();
                self.indicators.stoch.next(close).unwrap();
                self.indicators.rsi.next(close).unwrap();

                Candle::new()
                    .date(date)
                    .open(open)
                    .high(high)
                    .low(low)
                    .close(close)
                    .volume(volume)
                    .previous(data[key])
                    .build()
                    .unwrap()
            })
            .collect();

        self.set_current_price(parsed[0].close());
        self.peaks.calculate_peaks(&self.max_price).unwrap();
        self.patterns.detect_pattern(&self.peaks);

        //self.indicators.calculate_macd(&parsed).unwrap();

        //self.patterns.detect_upper_channel(&self.peaks);

        self.horizontal_levels
            .calculate_horizontal_highs(&self.current_price, &self.peaks)
            .unwrap();

        self.horizontal_levels
            .calculate_horizontal_lows(&self.current_price, &self.peaks)
            .unwrap();
        self.data = parsed;
        Ok(())
    }
}

pub struct InstrumentBuilder {
    ticker: Option<String>,
    //indicators: Option<Indicators>,
}

impl InstrumentBuilder {
    pub fn new() -> InstrumentBuilder {
        Self { ticker: None }
    }
    pub fn ticker(mut self, val: &str) -> Self {
        self.ticker = Some(String::from(val));
        self
    }

    // pub fn indicators(mut self, indicators: Indicators) -> Self {
    //   self.indicators = Some(indicators);
    //   self
    // }

    pub fn build(self) -> Result<Instrument> {
        if let (Some(ticker)) = (self.ticker) {
            Ok(Instrument {
                ticker,
                data: vec![],
                peaks: Peaks::new(),
                horizontal_levels: HorizontalLevels::new(),
                patterns: Patterns::new(),
                indicators: Indicators::new().unwrap(),
                current_price: 0.,
                min_price: env::var("MIN_PRICE").unwrap().parse::<f64>().unwrap(),
                max_price: env::var("MIN_PRICE").unwrap().parse::<f64>().unwrap(),
            })
        } else {
            Err(RsAlgoError {
                err: RsAlgoErrorKind::WrongInstrumentConf,
            })
        }
    }
}
