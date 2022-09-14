use super::strategy::*;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::*;
use rs_algo_shared::models::pattern::*;

#[derive(Clone)]
pub struct BollingerBands<'a> {
    name: &'a str,
    strategy_type: StrategyType,
    stop_loss: StopLoss,
}

#[async_trait]
impl<'a> Strategy for BollingerBands<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            stop_loss: init_stop_loss(),
            name: "Bollinger_Bands_Reversal_Riding_RSI",
            strategy_type: StrategyType::LongShort,
        })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn strategy_type(&self) -> &StrategyType {
        &self.strategy_type
    }

    fn update_stop_loss(&mut self, stop_type: StopLossType, price: f64) -> &StopLoss {
        self.stop_loss = update_stop_loss_values(&self.stop_loss, stop_type, price);
        &self.stop_loss
    }
    fn stop_loss(&self) -> &StopLoss {
        &self.stop_loss
    }

    fn entry_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        _upper_tf_instrument: &HigherTMInstrument,
    ) -> bool {
        let prev_index = get_prev_index(index);

        let close_price = &instrument.data.get(index).unwrap().close;
        let prev_close = &instrument.data.get(prev_index).unwrap().close;

        let patterns = &instrument.patterns.local_patterns;
        let current_pattern = get_current_pattern(index, patterns);

        let low_band = instrument.indicators.bb.data_b.get(index).unwrap();
        let prev_low_band = instrument.indicators.bb.data_b.get(prev_index).unwrap();
        let rsi = instrument.indicators.rsi.data_a.get(index).unwrap();
        let _prev_rsi = instrument.indicators.rsi.data_a.get(prev_index).unwrap();

        current_pattern != PatternType::ChannelDown
            && current_pattern != PatternType::LowerHighsLowerLows
            && rsi >= &30.
            && rsi <= &40.
            && close_price < low_band
            && prev_close >= prev_low_band
    }

    fn exit_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        _upper_tf_instrument: &HigherTMInstrument,
    ) -> bool {
        let prev_index = get_prev_index(index);
        let _candle_type = &instrument.data.get(index).unwrap().candle_type;

        let top_band = instrument.indicators.bb.data_a.get(index).unwrap();
        let mid_band = instrument.indicators.bb.data_c.get(index).unwrap();
        let low_band = instrument.indicators.bb.data_b.get(index).unwrap();

        let _prev_top_band = instrument.indicators.bb.data_a.get(prev_index).unwrap();
        let _prev_low_band = instrument.indicators.bb.data_b.get(prev_index).unwrap();

        let patterns = &instrument.patterns.local_patterns;
        let current_pattern = get_current_pattern(index, patterns);
        let _close_price = &instrument.data.get(index).unwrap().close;
        let _prev_close = &instrument.data.get(prev_index).unwrap().close;

        let backwards_candles = 5;
        let _max_band_hits = 3;
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

        (current_pattern != PatternType::ChannelUp
            && current_pattern != PatternType::HigherHighsHigherLows
            && (hits_over_top_band <= 5 && hits_above_mid_band > 5))
            //&& (close_price > top_band && prev_close <= prev_top_band ))
        || (hits_over_low_band >= 3)
    }

    fn entry_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool {
        match self.strategy_type {
            StrategyType::LongShort => self.exit_long(index, instrument, upper_tf_instrument),
            StrategyType::LongShortMultiTF => {
                self.exit_long(index, instrument, upper_tf_instrument)
            }
            StrategyType::OnlyShort => self.exit_long(index, instrument, upper_tf_instrument),
            _ => false,
        }
    }

    fn exit_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool {
        match self.strategy_type {
            StrategyType::LongShort => self.entry_long(index, instrument, upper_tf_instrument),
            StrategyType::LongShortMultiTF => {
                self.entry_long(index, instrument, upper_tf_instrument)
            }
            StrategyType::OnlyShort => self.entry_long(index, instrument, upper_tf_instrument),
            StrategyType::OnlyShort => self.exit_long(index, instrument, upper_tf_instrument),
            _ => false,
        }
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
            instrument,
            &self.strategy_type,
            trades_in,
            trades_out,
            self.name,
            equity,
            commission,
        )
    }
}
