use super::strategy::*;

use crate::helpers::calc::*;
use crate::trade::*;

use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::*;

pub struct MutiTimeFrameBollingerBands<'a> {
    name: &'a str,
    strategy_type: StrategyType,
}

#[async_trait]
impl<'a> Strategy for MutiTimeFrameBollingerBands<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            name: "Bollinger_Bands_Reversals2_MT_Macd",
            strategy_type: StrategyType::OnlyLongMultiTF,
        })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn strategy_type(&self) -> &StrategyType {
        &self.strategy_type
    }

    fn entry_long(
        &self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool {
        let first_weekly_entry = get_upper_timeframe_data(
            index,
            instrument,
            upper_tf_instrument,
            |(idx, prev_idx, upper_inst)| {
                let curr_upper_macd_a = upper_inst.indicators.macd.data_a.get(idx).unwrap();
                let curr_upper_macd_b = upper_inst.indicators.macd.data_b.get(idx).unwrap();

                let prev_upper_macd_a = upper_inst.indicators.macd.data_a.get(prev_idx).unwrap();
                let prev_upper_macd_b = upper_inst.indicators.macd.data_b.get(prev_idx).unwrap();
                curr_upper_macd_a > curr_upper_macd_b && prev_upper_macd_b >= prev_upper_macd_a
            },
        );

        let upper_macd = get_upper_timeframe_data(
            index,
            instrument,
            upper_tf_instrument,
            |(idx, prev_idx, upper_inst)| {
                let curr_upper_macd_a = upper_inst.indicators.macd.data_a.get(idx).unwrap();
                let curr_upper_macd_b = upper_inst.indicators.macd.data_b.get(idx).unwrap();
                curr_upper_macd_a > curr_upper_macd_b
            },
        );

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

        let entry_condition = first_weekly_entry
            || (upper_macd
                && prev_close < prev_open
                && prev_close < prev_low_band
                && close_price >= low_band
                && close_price >= open_price);

        entry_condition
    }

    fn exit_long(
        &self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool {
        let upper_macd = get_upper_timeframe_data(
            index,
            instrument,
            upper_tf_instrument,
            |(idx, prev_idx, upper_inst)| {
                let curr_upper_macd_a = upper_inst.indicators.macd.data_a.get(idx).unwrap();
                let curr_upper_macd_b = upper_inst.indicators.macd.data_b.get(idx).unwrap();

                let prev_upper_macd_a = upper_inst.indicators.macd.data_a.get(prev_idx).unwrap();
                let prev_upper_macd_b = upper_inst.indicators.macd.data_b.get(prev_idx).unwrap();
                curr_upper_macd_a < curr_upper_macd_b // && prev_upper_macd_a >= prev_upper_macd_b
            },
        );

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
        &self,
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
        &self,
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