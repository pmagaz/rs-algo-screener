use super::strategy::*;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::Instrument;
use rs_algo_shared::models::pattern::*;

pub struct BollingerBands<'a> {
    name: &'a str,
    strategy_type: StrategyType,
}

#[async_trait]
impl<'a> Strategy for BollingerBands<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            name: "Bollinger_Bands_Reversal_Riding_BandWidth",
            strategy_type: StrategyType::OnlyLong,
        })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn strategy_type(&self) -> &StrategyType {
        &self.strategy_type
    }

    fn market_in_fn(&self, index: usize, instrument: &Instrument, stop_loss: f64) -> TradeResult {
        let prev_index = get_prev_index(index);

        let close_price = &instrument.data.get(index).unwrap().close;
        let prev_close = &instrument.data.get(prev_index).unwrap().close;

        let patterns = &instrument.patterns.local_patterns;
        let current_pattern = get_current_pattern(index, patterns);

        let top_band = instrument.indicators.bb.data_a.get(index).unwrap();
        let mid_band = instrument.indicators.bb.data_c.get(index).unwrap();
        let low_band = instrument.indicators.bb.data_b.get(index).unwrap();
        let prev_low_band = instrument.indicators.bb.data_b.get(prev_index).unwrap();

        let backwards_candles = 5;
        let _max_band_hits = 3;
        let mut hits_over_top_band: usize = 0;
        let mut hits_over_low_band: usize = 0;
        let mut hits_above_mid_band: usize = 0;

        if index >= 5 {
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
        }

        let entry_condition = current_pattern != PatternType::ChannelDown
            && current_pattern != PatternType::LowerHighsLowerLows
          //  && (prev_bbw < &20. && bbw >= &20. && close_price > up_band
                || (close_price > low_band && prev_close <= prev_low_band);
        //|| (close_price < low_band && prev_close >= prev_low_band));

        resolve_trade_in(index, instrument, entry_condition, stop_loss)
    }

    fn market_out_fn(
        &self,
        index: usize,
        instrument: &Instrument,
        trade_in: &TradeIn,
    ) -> TradeResult {
        let prev_index = get_prev_index(index);
        let _candle_type = &instrument.data.get(index).unwrap().candle_type;

        let top_band = instrument.indicators.bb.data_a.get(index).unwrap();
        let mid_band = instrument.indicators.bb.data_c.get(index).unwrap();
        let low_band = instrument.indicators.bb.data_b.get(index).unwrap();

        let _prev_top_band = instrument.indicators.bb.data_a.get(prev_index).unwrap();

        let patterns = &instrument.patterns.local_patterns;
        let current_pattern = get_current_pattern(index, patterns);
        let _close_price = &instrument.data.get(index).unwrap().close;
        let _prev_close = &instrument.data.get(prev_index).unwrap().close;
        let _bbw = instrument.indicators.bbw.data_a.get(index).unwrap();
        let _prev_bbw = instrument.indicators.bbw.data_a.get(prev_index).unwrap();

        let backwards_candles = 5;
        let _max_band_hits = 3;
        let mut hits_over_top_band: usize = 0;
        let mut hits_over_low_band: usize = 0;
        let mut hits_above_mid_band: usize = 0;

        if index >= 5 {
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
        }

        let exit_condition = (current_pattern != PatternType::ChannelUp
            && current_pattern != PatternType::HigherHighsHigherLows
            && (hits_over_top_band <= 5 && hits_above_mid_band > 5))
            //&& (close_price > top_band && prev_close <= prev_top_band ))
            || (hits_over_low_band >= 3 );

        let stop_loss = true;
        resolve_trade_out(index, instrument, trade_in, exit_condition, stop_loss)
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
