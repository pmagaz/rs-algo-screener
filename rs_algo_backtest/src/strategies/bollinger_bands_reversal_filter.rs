use super::strategy::Strategy;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::candle::*;
use rs_algo_shared::models::instrument::Instrument;
use rs_algo_shared::models::pattern::*;
use rs_algo_shared::models::status::*;

pub struct BollingerBands<'a> {
    name: &'a str,
}

#[async_trait]
impl<'a> Strategy for BollingerBands<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            name: "Bollinger_Bands_Reversal_Filter",
        })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn market_in_fn(&self, index: usize, instrument: &Instrument, stop_loss: f64) -> TradeResult {
        let prev_index = get_prev_index(index);

        let close_price = &instrument.data.get(index).unwrap().close;
        let prev_close = &instrument.data.get(prev_index).unwrap().close;

        let patterns = &instrument.patterns.local_patterns;
        let current_pattern = get_current_pattern(index, patterns);

        let low_band = instrument.indicators.bb.data_b.get(index).unwrap();
        let prev_low_band = instrument.indicators.bb.data_b.get(prev_index).unwrap();
        let ema_8 = instrument.indicators.ema_a.data_a.get(index).unwrap();
        let ema_20 = instrument.indicators.bb.data_c.get(index).unwrap();
        let ema_50 = instrument.indicators.ema_c.data_a.get(index).unwrap();

        let market_filter = match ema_8 {
            _x if ema_8 > ema_50 && ema_50 > ema_50 => Status::Bullish,
            _x if ema_8 < ema_50 && ema_50 > ema_50 => Status::Neutral,
            _x if ema_8 < ema_50 && ema_50 < ema_50 => Status::Bearish,
            _ => Status::Neutral,
        };

        let entry_condition = market_filter != Status::Bearish
            && current_pattern != PatternType::ChannelDown
            && current_pattern != PatternType::LowerHighsLowerLows
            && close_price < low_band
            && prev_close >= prev_low_band;

        resolve_trade_in(index, instrument, entry_condition, stop_loss)
    }

    fn market_out_fn(
        &self,
        index: usize,
        instrument: &Instrument,
        trade_in: &TradeIn,
    ) -> TradeResult {
        let prev_index = get_prev_index(index);
        let candle_type = &instrument.data.get(index).unwrap().candle_type;

        let top_band = instrument.indicators.bb.data_a.get(index).unwrap();
        let mid_band = instrument.indicators.bb.data_c.get(index).unwrap();
        let low_band = instrument.indicators.bb.data_b.get(index).unwrap();

        let prev_top_band = instrument.indicators.bb.data_a.get(prev_index).unwrap();

        let patterns = &instrument.patterns.local_patterns;
        let current_pattern = get_current_pattern(index, patterns);
        let close_price = &instrument.data.get(index).unwrap().close;
        let prev_close = &instrument.data.get(prev_index).unwrap().close;

        let backwards_candles = 5;
        let max_band_hits = 3;
        let mut hits_over_top_band: usize = 0;
        let mut hits_over_low_band: usize = 0;
        let mut hits_above_mid_band: usize = 0;

        for x in (index - backwards_candles..index).rev() {
            let highest_price = instrument.data.get(x).unwrap().high;
            if highest_price > *top_band {
                hits_over_top_band += 1;
            }

            let mid_price = instrument.data.get(x).unwrap().close;
            if mid_price < *mid_band {
                hits_above_mid_band += 1;
            }

            let lowest_price = instrument.data.get(x).unwrap().low;
            if lowest_price < *low_band {
                hits_over_low_band += 1;
            }
        }

        let ema_8 = instrument.indicators.ema_a.data_a.get(index).unwrap();
        let ema_50 = instrument.indicators.ema_b.data_a.get(index).unwrap();
        let ema_80 = instrument.indicators.ema_c.data_a.get(index).unwrap();

        let market_filter = match ema_8 {
            _x if ema_8 > ema_50 && ema_50 > ema_80 => Status::Bullish,
            _x if ema_8 < ema_50 && ema_50 > ema_80 => Status::Neutral,
            _x if ema_8 < ema_50 && ema_50 < ema_80 => Status::Bearish,
            _ => Status::Neutral,
        };

        let exit_condition = (market_filter == Status::Bearish)
            || (current_pattern != PatternType::ChannelUp
                && current_pattern != PatternType::HigherHighsHigherLows
                && (close_price > top_band && prev_close <= prev_top_band))
            || (hits_over_low_band >= 3 && hits_above_mid_band >= 5);

        resolve_trade_out(index, instrument, trade_in, exit_condition)
    }

    fn backtest_result(
        &self,
        instrument: &Instrument,
        trades_in: Vec<TradeIn>,
        trades_out: Vec<TradeOut>,
        equity: f64,
        commission: f64,
    ) -> BackTestResult {
        resolve_backtest(
            instrument, trades_in, trades_out, self.name, equity, commission,
        )
    }
}
