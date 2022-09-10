use super::strategy::*;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::*;

#[derive(Clone)]
pub struct BollingerBands<'a> {
    name: &'a str,
     strategy_type: StrategyType,
     stop_loss: f64
}

#[async_trait]
impl<'a> Strategy for BollingerBands<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            stop_loss: 0.,
            name: "Bollinger_Bands_Reversal_2",
            strategy_type: StrategyType::OnlyLong,
        })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn strategy_type(&self) -> &StrategyType {
        &self.strategy_type
    }

    fn update_stop_loss(&mut self, price: f64) -> bool {
        self.stop_loss = price;
        true
    }

    fn stop_loss(&self) -> f64 {
        self.stop_loss
    }

    fn entry_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool {
        let prev_index = get_prev_index(index);

        let patterns = &instrument.patterns.local_patterns;
        let current_pattern = get_current_pattern(index, patterns);

        let open_price = &instrument.data.get(index).unwrap().open;
        let close_price = &instrument.data.get(index).unwrap().close;
        let prev_open = &instrument.data.get(prev_index).unwrap().open;
        let prev_close = &instrument.data.get(prev_index).unwrap().close;
        let prev_high = &instrument.data.get(prev_index).unwrap().close;

        let low_band = instrument.indicators.bb.data_b.get(index).unwrap();
        let mid_band = instrument.indicators.bb.data_c.get(index).unwrap();
        let prev_mid_band = instrument.indicators.bb.data_c.get(prev_index).unwrap();
        let prev_low_band = instrument.indicators.bb.data_b.get(prev_index).unwrap();

        let entry_condition = prev_close < prev_open
            && prev_close < prev_low_band
            && close_price >= low_band
            && close_price >= open_price;

        entry_condition
    }

    fn exit_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool {
        let prev_index = get_prev_index(index);

        let open_price = &instrument.data.get(index).unwrap().open;
        let close_price = &instrument.data.get(index).unwrap().close;
        let prev_open = &instrument.data.get(prev_index).unwrap().open;
        let prev_close = &instrument.data.get(prev_index).unwrap().close;
        let prev_high = &instrument.data.get(prev_index).unwrap().close;

        let top_band = instrument.indicators.bb.data_a.get(index).unwrap();
        let prev_top_band = instrument.indicators.bb.data_a.get(prev_index).unwrap();

        let exit_condition = prev_close > prev_open
            && prev_close > prev_top_band
            && close_price <= top_band
            && close_price <= open_price;

        exit_condition
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
