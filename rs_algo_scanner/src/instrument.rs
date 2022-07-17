use crate::candle::{Candle, CandleType};
use crate::error::{Result, RsAlgoError, RsAlgoErrorKind};
use crate::indicators::Indicators;
use crate::patterns::divergences::Divergences;
use crate::patterns::horizontal_levels::HorizontalLevels;
use crate::patterns::pattern::Patterns;
use crate::patterns::peaks::Peaks;

use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::pattern::PatternSize;
use rs_algo_shared::models::time_frame::TimeFrameType;

use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    symbol: String,
    time_frame: TimeFrameType,
    data: Vec<Candle>,
    current_price: f64,
    current_candle: CandleType,
    date: DbDateTime,
    min_price: f64,
    max_price: f64,
    avg_volume: f64,
    peaks: Peaks,
    horizontal_levels: HorizontalLevels,
    patterns: Patterns,
    indicators: Indicators,
    divergences: Divergences,
}

impl Instrument {
    pub fn new() -> InstrumentBuilder {
        InstrumentBuilder::new()
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn time_frame(&self) -> &TimeFrameType {
        &self.time_frame
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

    pub fn date(&self) -> &DbDateTime {
        &self.date
    }

    // pub fn current_price(&self) -> f64 {
    //     self.current_price
    // }

    pub fn current_candle(&self) -> &Candle {
        let num_candles = &self.data().len() - 1;
        &self.data()[num_candles]
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

    pub fn divergences(&self) -> &Divergences {
        &self.divergences
    }
    pub fn set_data(
        &mut self,
        data: Vec<(DateTime<Local>, f64, f64, f64, f64, f64)>,
    ) -> Result<()> {
        let mut avg_volume = vec![];
        let logarithmic = env::var("LOGARITHMIC_SCANNER")
            .unwrap()
            .parse::<bool>()
            .unwrap();

        let candles: Vec<Candle> = data
            .iter()
            .enumerate()
            .map(|(id, x)| {
                let date = x.0;
                let open: f64;
                let high: f64;
                let low: f64;
                let close: f64;
                let volume = x.5;

                match logarithmic {
                    true => {
                        open = x.1.ln();
                        high = x.2.ln();
                        close = x.4.ln();
                        low = match x.3 {
                            _x if x.3 > 0. => x.3.ln(),
                            _x if x.3 <= 0. => 0.01,
                            _ => x.3.ln(),
                        };
                    }
                    false => {
                        open = x.1;
                        high = x.2;
                        close = x.4;
                        low = match x.3 {
                            _x if x.3 > 0. => x.3,
                            _x if x.3 <= 0. => 0.01,
                            _ => x.3,
                        };
                    }
                };

                // let low = match x.3 {
                //     _x if x.3 > 0. => x.3.ln(),
                //     _x if x.3 <= 0. => 0.01,
                //     _ => x.3.ln(),
                // };

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

                avg_volume.push(volume);
                self.peaks.highs.push(high);
                self.peaks.lows.push(low);
                self.peaks.close.push(close);
                let pre_0 = match id {
                    0 => id,
                    _ => id - 1,
                };

                let prev_1 = match pre_0 {
                    0 => id,
                    _ => id - 1,
                };

                let ohlc_indicators = match logarithmic {
                    true => (open.exp(), high.exp(), low.exp(), close.exp()),
                    false => (open, high, low, close),
                };

                self.indicators
                    .calculate_indicators(ohlc_indicators)
                    .unwrap();

                Candle::new()
                    .date(date)
                    .open(open)
                    .high(high)
                    .low(low)
                    .close(close)
                    .volume(volume)
                    .previous_candles(vec![data[pre_0], data[prev_1]])
                    .logarithmic(logarithmic)
                    .build()
                    .unwrap()
            })
            .collect();

        if candles.len() > 0 {
            self.peaks
                .calculate_peaks(&self.max_price, &self.min_price)
                .unwrap();

            let local_maxima = self.peaks.local_maxima();
            let local_minima = self.peaks.local_minima();
            // let extrema_maxima = self.peaks.extrema_maxima();
            // let extrema_minima = self.peaks.extrema_minima();

            self.patterns
                .detect_pattern(PatternSize::Local, local_maxima, local_minima, &candles);

            // self.patterns.detect_pattern(
            //     PatternSize::Extrema,
            //     extrema_maxima,
            //     extrema_minima,
            //     &candles,
            // );

            self.horizontal_levels
                .calculate_horizontal_highs(&self.current_price, &self.peaks)
                .unwrap();

            self.horizontal_levels
                .calculate_horizontal_lows(&self.current_price, &self.peaks)
                .unwrap();

            self.divergences.detect_divergences(
                &self.indicators,
                &self.patterns.local_patterns,
                &candles,
                &local_maxima,
            );

            self.data = candles
                .into_iter()
                .map(|candle| {
                    let data = match logarithmic {
                        true => candle.from_logarithmic_values(),
                        false => candle,
                    };

                    if self.min_price == -100. {
                        self.min_price = data.low();
                    }
                    if data.low() < self.min_price {
                        self.min_price = data.low();
                    }
                    if self.max_price == -100. {
                        self.max_price = data.low();
                    }
                    if data.high() > self.max_price {
                        self.max_price = data.high();
                    }
                    data
                })
                .collect();

            self.set_current_price(self.data.last().unwrap().close());

            self.current_candle = self.current_candle().candle_type().clone();
            self.avg_volume = average_f64(&avg_volume);
        }
        Ok(())
    }
}

pub struct InstrumentBuilder {
    symbol: Option<String>,
    time_frame: Option<TimeFrameType>,
    //indicators: Option<Indicators>,
}

impl InstrumentBuilder {
    pub fn new() -> InstrumentBuilder {
        Self {
            symbol: None,
            time_frame: None,
        }
    }
    pub fn symbol(mut self, val: &str) -> Self {
        self.symbol = Some(String::from(val));
        self
    }
    pub fn time_frame(mut self, val: TimeFrameType) -> Self {
        self.time_frame = Some(val);
        self
    }

    pub fn build(self) -> Result<Instrument> {
        if let (Some(symbol), Some(time_frame)) = (self.symbol, self.time_frame) {
            Ok(Instrument {
                symbol,
                time_frame: time_frame,
                current_price: 0.,
                date: to_dbtime(Local::now()), //FIXME
                current_candle: CandleType::Default,
                min_price: env::var("MIN_PRICE").unwrap().parse::<f64>().unwrap(),
                max_price: env::var("MIN_PRICE").unwrap().parse::<f64>().unwrap(),
                avg_volume: 0.,
                data: vec![],
                peaks: Peaks::new(),
                horizontal_levels: HorizontalLevels::new(),
                patterns: Patterns::new(),
                indicators: Indicators::new().unwrap(),
                divergences: Divergences::new().unwrap(),
            })
        } else {
            Err(RsAlgoError {
                err: RsAlgoErrorKind::WrongInstrumentConf,
            })
        }
    }
}
