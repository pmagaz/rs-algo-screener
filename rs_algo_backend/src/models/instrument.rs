use std::collections::HashMap;

use chrono::DateTime;
use chrono::Local;
use serde::{Deserialize, Serialize};
use ta::indicators::ExponentialMovingAverage;
use ta::indicators::RelativeStrengthIndex;
use ta::indicators::SlowStochastic;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub symbol: String,
    #[serde(skip_deserializing)]
    pub updated: String,
    data: Vec<Candle>,
    current_price: f64,
    min_price: f64,
    max_price: f64,
    peaks: Peaks,
    //horizontal_levels: HorizontalLevels,
    patterns: Patterns,
    indicators: Indicators,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Indicators {
    pub macd: Macd,
    pub stoch: Stoch,
    pub rsi: Rsi,
    pub ema_a: Ema,
    pub ema_b: Ema,
    pub ema_c: Ema,
    pub ema_d: Ema,
    pub ema_e: Ema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Stoch {
    stoch: SlowStochastic,
    ema: ExponentialMovingAverage,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ema {
    ema: ExponentialMovingAverage,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rsi {
    rsi: RelativeStrengthIndex,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macd {
    ema26: ExponentialMovingAverage,
    ema12: ExponentialMovingAverage,
    ema9: ExponentialMovingAverage,
    data_a: Vec<f64>,
    data_b: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HorizontalLevelType {
    Resistance,
    Support,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizontalLevel {
    price: f64,
    min_value: f64,
    max_value: f64,
    level_type: HorizontalLevelType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizontalLevels {
    horizontal_levels: HashMap<usize, HorizontalLevel>,
}

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

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum IndicatorType {
//     MacD,
//     Stoch,
//     Rsi,
//     Ema_a,
//     Ema_b,
//     Ema_c,
//     Ema_d,
//     Ema_e,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum IndicatorStatus {
//     Bearish,
//     BearishBellowZero,
//     Bullish,
//     BullishOverZero,
//     Oversold,
//     Overbought,
//     Default,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct IndicatorReq {
//     indicator_type: IndicatorType,
//     status: IndicatorStatus,
//     data_a: Vec<f64>,
//     data_b: Vec<f64>,
// }

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct Instrument {
//     pub grant_type: String,
//     pub access_code: String,
//     pub redirect_url: String,
// }

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct InstrumentRes {
//     pub symbol: String,
//     pub updated: String,
//     pub candle: CandleType,
//     pub current_price: f64,
//     pub patterns: Patterns,
//     pub indicators: Vec<IndicatorReq>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    TriangleSymmetricalTop,
    TriangleSymmetricalTopActivated,
    TriangleSymmetricalBottom,
    TriangleSymmetricalBottomActivated,
    TriangleDescendantTop,
    TriangleDescendantTopActivated,
    TriangleDescendantBottom,
    TriangleDescendantBottomActivated,
    TriangleAscendantTop,
    TriangleAscendantTopActivated,
    TriangleAscendantBottom,
    TriangleAscendantBottomActivated,
    RectangleTop,
    RectangleTopActivatedUp,
    RectangleTopActivatedLow,
    RectangleBottom,
    RectangleBottomActivated,
    ChannelUpTop,
    ChannelUpTopActivated,
    ChannelUpBottom,
    ChannelUpBottomActivated,
    ChannelDownTop,
    ChannelDownTopActivated,
    ChannelDownBottom,
    ChannelDownBottomActivated,
    BroadeningTop,
    BroadeningTopActivated,
    BroadeningBottom,
    BroadeningBottomActivated,
    DoubleBottom,
    DoubleBottomActivated,
    DoubleTop,
    DoubleTopActivated,
    None,
}

type Point = (usize, f64);
pub type DataPoints = Vec<Point>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternSize {
    Local,
    Extrema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub pattern_type: PatternType,
    pub pattern_size: PatternSize,
    pub data_points: DataPoints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patterns {
    pub local_patterns: Vec<Pattern>,
    pub extrema_patterns: Vec<Pattern>,
}
