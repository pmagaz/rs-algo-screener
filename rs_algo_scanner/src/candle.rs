use crate::error::{Result, RsAlgoError, RsAlgoErrorKind};
use crate::helpers::comp::percentage_change;

use rs_algo_shared::helpers::date::{DateTime, Local};
pub type OHLCV = (f64, f64, f64, f64);
pub type DOHLCV = (DateTime<Local>, f64, f64, f64, f64, f64);

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CandleType {
    Default,
    Doji,
    Karakasa,
    BearishKarakasa,
    Marubozu,
    BearishMarubozu,
    Harami,
    BearishHarami,
    Engulfing,
    BearishEngulfing,
    HangingMan,
    BullishCrows,
    BearishCrows,
    BullishGap,
    BearishGap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    candle_type: CandleType,
    date: DateTime<Local>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl Candle {
    pub fn new() -> CandleBuilder {
        CandleBuilder::new()
    }
    pub fn date(&self) -> DateTime<Local> {
        self.date
    }

    pub fn open(&self) -> f64 {
        self.open
    }

    pub fn high(&self) -> f64 {
        self.high
    }

    pub fn low(&self) -> f64 {
        self.low
    }

    pub fn close(&self) -> f64 {
        self.close
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }
    pub fn candle_type(&self) -> &CandleType {
        &self.candle_type
    }
}

pub struct CandleBuilder {
    date: Option<DateTime<Local>>,
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
    volume: Option<f64>,
    previous_candles: Option<Vec<DOHLCV>>,
}

impl CandleBuilder {
    pub fn new() -> Self {
        Self {
            date: None,
            open: None,
            high: None,
            low: None,
            close: None,
            volume: None,
            previous_candles: None,
        }
    }

    pub fn date(mut self, val: DateTime<Local>) -> Self {
        self.date = Some(val);
        self
    }

    pub fn open(mut self, val: f64) -> Self {
        self.open = Some(val);
        self
    }

    pub fn high(mut self, val: f64) -> Self {
        self.high = Some(val);
        self
    }

    pub fn low(mut self, val: f64) -> Self {
        self.low = Some(val);
        self
    }

    pub fn close(mut self, val: f64) -> Self {
        self.close = Some(val);
        self
    }

    pub fn volume(mut self, val: f64) -> Self {
        self.volume = Some(val);
        self
    }

    pub fn previous_candles(mut self, val: Vec<DOHLCV>) -> Self {
        self.previous_candles = Some(val);
        self
    }

    fn get_current_ohlc(&self) -> OHLCV {
        (
            self.open.unwrap(),
            self.high.unwrap(),
            self.low.unwrap(),
            self.close.unwrap(),
        )
    }

    fn get_previous_ohlc(&self, index: usize) -> OHLCV {
        (
            self.previous_candles.as_ref().unwrap()[index].1,
            self.previous_candles.as_ref().unwrap()[index].2,
            self.previous_candles.as_ref().unwrap()[index].3,
            self.previous_candles.as_ref().unwrap()[index].4,
        )
    }

    fn is_doji(&self) -> bool {
        // (O = C ) || (ABS(O – C ) <= ((H – L ) * 0.1))
        let (open, high, low, close) = &self.get_current_ohlc();
        (open.floor() == close.floor()) || (open - close).abs() <= ((high - low) * 0.1)
    }

    fn is_karakasa(&self) -> bool {
        // ((H-L)>3*(O-C)AND((C-L)/(.001+H-L)>0.6)AND((O-L)/(.001+H-L)>0.6))
        let (open, high, low, close) = &self.get_current_ohlc();
        (high - low) > 3. * (open - close)
            && ((close - low) / (0.001 + high - low) > 0.6)
            && ((open - low) / (0.001 + high - low) > 0.6)
    }

    fn is_bearish_karakasa(&self) -> bool {
        // (((H – L) > 3 * (O – C)) AND ((H – C) / (.001 + H – L) > 0.6) AND ((H – O) / (.001 + H – L) > 0.6))
        let (open, high, low, close) = &self.get_current_ohlc();
        ((high - low) > 3. * (open - close))
            && ((high - close) / (0.001 + high - low) > 0.6)
            && ((high - open) / (0.001 + high - low) > 0.6)
    }

    fn is_marubozu(&self) -> bool {
        //O = L AND H = C.
        let (open, high, low, close) = &self.get_current_ohlc();
        let high_shadow = (high - close) / close;
        let low_shadow = (low - open) / open;
        (open <= low && low_shadow < 0.1) && (high >= close && high_shadow < 0.1)
    }

    fn is_bearish_marubozu(&self) -> bool {
        //O = H AND C = L.
        let (open, high, low, close) = &self.get_current_ohlc();
        let high_shadow = (high - open) / open;
        let low_shadow = (low - close) / close;
        (open >= high && high_shadow < 0.1) && (low <= close && high_shadow < 0.1)
    }

    fn is_hanging_man(&self) -> bool {
        // (((H – L) > 4 * (O – C)) AND ((C – L) / (.001 + H – L) >= 0.75) AND ((O – L) / (.001 + H – L) >= .075)))
        let (open, high, low, close) = &self.get_current_ohlc();
        ((high - low) > 4. * (open - close))
            && ((close - low) / (0.001 + high - low) > 0.75)
            && ((open - low) / (0.001 + high - low) > 0.75)
    }

    fn is_engulfing(&self) -> bool {
        //(O1 > C1) AND (C > O) AND (C >= O1) AND (C1 >= O) AND ((C – O) > (O1 – C1))
        let (open, _high, _low, close) = &self.get_current_ohlc();
        let (prev_open, _prev_high, _prev_low, prev_close) = &self.get_previous_ohlc(0);
        (prev_open > prev_close)
            && (close > open)
            && (close >= prev_open)
            && (prev_close >= open)
            && ((close - open) > (prev_open - prev_close))
    }

    fn is_bearish_engulfing(&self) -> bool {
        //(C1 > O1) AND (O > C) AND (O >= C1) AND (O1 >= C) AND ((O – C) > (C1 – O1))
        let (open, _high, _low, close) = &self.get_current_ohlc();
        let (prev_open, _prev_high, _prev_low, prev_close) = &self.get_previous_ohlc(0);
        //println!("5555555 {:?} {:?}", prev_open, open);
        (prev_close > prev_open)
            && (open > close)
            && (open >= prev_close)
            && (prev_open >= close)
            && ((open - close) > (prev_close - prev_open))
    }

    fn is_harami(&self) -> bool {
        //((O1 > C1) AND (C > O) AND (C <= O1) AND (C1 <= O) AND ((C – O) < (O1 – C1)))
        let (open, _high, _low, close) = &self.get_current_ohlc();
        let (prev_open, _prev_high, _prev_low, prev_close) = &self.get_previous_ohlc(0);
        (prev_open > prev_close)
            && (close > open)
            && (close <= prev_open)
            && (prev_close <= open)
            && ((close - open) < (prev_open - prev_close))
    }

    fn is_bearish_harami(&self) -> bool {
        //((C1 > O1) AND (O > C) AND (O <= C1) AND (O1 <= C) AND ((O – C) < (C1 – O1)))
        let (open, _high, _low, close) = &self.get_current_ohlc();
        let (prev_open, _prev_high, _prev_low, prev_close) = &self.get_previous_ohlc(0);
        (prev_close > prev_open)
            && (open > close)
            && (open <= prev_close)
            && (prev_open <= close)
            && ((open - close) < (prev_close - prev_open))
    }

    fn is_bullish_gap(&self) -> bool {
        //FIXMW
        //((C1 > O1) AND (O > C) AND (O <= C1) AND (O1 <= C) AND ((O – C) < (C1 – O1)))
        let (open, _high, _low, close) = &self.get_current_ohlc();
        let (_prev_open, prev_high, _prev_low, _prev_close) = &self.get_previous_ohlc(0);
        let percentage_diff = percentage_change(*open, *prev_high);
        println!("11111 {} {}", open, prev_high);
        open > prev_high && percentage_diff > 2. && close > prev_high
    }

    fn is_bearish_gap(&self) -> bool {
        //FIXME
        //((C1 > O1) AND (O > C) AND (O <= C1) AND (O1 <= C) AND ((O – C) < (C1 – O1)))
        let (open, _high, _low, close) = &self.get_current_ohlc();
        let (_a, _prev_high, prev_low, _prev_close) = &self.get_previous_ohlc(0);
        let percentage_diff = percentage_change(*open, *prev_low);
        open < prev_low && percentage_diff > 2. && close < prev_low
    }

    fn is_bullish_crows(&self) -> bool {
        //(C>O*1.01) AND(C1>O1*1.01) AND(C2>O2*1.01) AND(C>C1) AND
        // (C1>C2) AND(OO1) AND(O1O2) AND (((H-C)/(H-L))<.2) AND(((H1-C1)/(H1-L1))<.2)AND(((H2-C2)/(H2-L2))<.2)
        let (open, high, low, close) = &self.get_current_ohlc();
        let (prev_open, prev_high, prev_low, prev_close) = &self.get_previous_ohlc(0);
        let (prev_open1, prev_high1, prev_low1, prev_close1) = &self.get_previous_ohlc(1);

        (close > &(open * 1.01))
            && (prev_close > &(prev_open * 1.01))
            && (prev_close1 > &(prev_open1 * 1.01))
            && (close > prev_close)
            && (prev_close > prev_close1)
            && (((high - close) / (high - low) < 0.2)
                && ((prev_high - prev_close) / (prev_high - prev_low) < 0.2)
                && ((prev_high1 - prev_close1) / (prev_high1 - prev_low1) < 0.2))
    }

    fn is_bearish_crows(&self) -> bool {
        //(C>O*1.01) AND(C1>O1*1.01) AND(C2>O2*1.01) AND(C>C1) AND
        // (C1>C2) AND(OO1) AND(O1O2) AND (((H-C)/(H-L))<.2) AND(((H1-C1)/(H1-L1))<.2)AND(((H2-C2)/(H2-L2))<.2)
        let (open, high, low, close) = &self.get_current_ohlc();
        let (prev_open, prev_high, prev_low, prev_close) = &self.get_previous_ohlc(0);
        let (prev_open1, prev_high1, prev_low1, prev_close1) = &self.get_previous_ohlc(1);

        (open > &(close * 1.01))
            && (prev_open > &(prev_close * 1.01))
            && (prev_open1 > &(prev_close1 * 1.01))
            && (close < prev_close)
            && (prev_close < prev_close1)
            && (open > prev_close)
            && (open < prev_open)
            && (prev_open > prev_close1)
            && (prev_open < prev_open1)
            && (((close - low) / (high - low) < 0.2)
                && ((prev_close - prev_low) / (prev_high - prev_low) < 0.2)
                && ((prev_close1 - prev_low1) / (prev_high1 - prev_low1) < 0.2))
    }

    fn identify_candle_type(&self) -> CandleType {
        if self.is_doji() {
            CandleType::Doji
        } else if self.is_karakasa() {
            CandleType::Karakasa
        } else if self.is_bearish_karakasa() {
            CandleType::BearishKarakasa
        } else if self.is_hanging_man() {
            CandleType::HangingMan
        } else if self.is_bullish_gap() {
            CandleType::BullishGap
        } else if self.is_bearish_gap() {
            CandleType::BearishGap
        } else if self.is_bullish_crows() {
            CandleType::BullishCrows
        } else if self.is_bearish_crows() {
            CandleType::BearishCrows
        } else if self.is_marubozu() {
            CandleType::Marubozu
        } else if self.is_bearish_marubozu() {
            CandleType::BearishMarubozu
        } else if self.is_engulfing() {
            CandleType::Engulfing
        } else if self.is_bearish_engulfing() {
            CandleType::BearishEngulfing
        } else if self.is_harami() {
            CandleType::Harami
        } else if self.is_bearish_harami() {
            CandleType::BearishHarami
        } else {
            CandleType::Default
        }
    }

    pub fn build(self) -> Result<Candle> {
        if let (
            Some(date),
            Some(open),
            Some(high),
            Some(low),
            Some(close),
            Some(volume),
            Some(previous_candles),
        ) = (
            self.date,
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume,
            self.previous_candles.as_ref(),
        ) {
            Ok(Candle {
                candle_type: self.identify_candle_type(),
                date,
                open,
                close,
                high,
                low,
                volume,
            })
        } else {
            Err(RsAlgoError {
                err: RsAlgoErrorKind::InvalidCandle,
            })
        }
    }
}
