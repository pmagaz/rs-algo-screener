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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndicatorStatus {
    Bearish,
    BearishBellowZero,
    Bullish,
    BullishOverZero,
    Oversold,
    Overbought,
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentRes {
    pub symbol: String,
    pub candle: CandleType,
    pub current_price: f64,
    pub patterns: Patterns,
    pub indicators: Vec<IndicatorStatus>,
}
