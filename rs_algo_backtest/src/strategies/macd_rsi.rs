use super::strategy::*;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::*;

pub struct Macd<'a> {
    name: &'a str,
    strategy_type: StrategyType,
}

#[async_trait]
impl<'a> Strategy for Macd<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            name: "MACD_RSI",
            strategy_type: StrategyType::OnlyLong,
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
        higher_tm_instrument: &HigherTMInstrument,
    ) -> bool {
        let prev_index = get_prev_index(index);

        let current_rsi = instrument.indicators.rsi.data_a.get(index).unwrap();
        let _prev_rsi = instrument.indicators.rsi.data_a.get(prev_index).unwrap();

        let current_macd_a = instrument.indicators.macd.data_a.get(index).unwrap();
        let current_macd_b = instrument.indicators.macd.data_b.get(index).unwrap();

        let _prev_macd_a = instrument.indicators.macd.data_a.get(prev_index).unwrap();
        let _prev_macd_b = instrument.indicators.macd.data_b.get(prev_index).unwrap();

        let entry_condition = current_rsi <= &30. && current_macd_a > current_macd_b;

        entry_condition
    }

    fn exit_long(
        &self,
        index: usize,
        instrument: &Instrument,
        higher_tm_instrument: &HigherTMInstrument,
    ) -> bool {
        let prev_index = get_prev_index(index);

        let current_rsi = instrument.indicators.rsi.data_a.get(index).unwrap();
        let _prev_rsi = instrument.indicators.rsi.data_a.get(prev_index).unwrap();

        let current_macd_a = instrument.indicators.macd.data_a.get(index).unwrap();
        let current_macd_b = instrument.indicators.macd.data_b.get(index).unwrap();

        let _prev_macd_a = instrument.indicators.macd.data_a.get(prev_index).unwrap();
        let _prev_macd_b = instrument.indicators.macd.data_b.get(prev_index).unwrap();

        let exit_condition = current_rsi >= &70. && current_macd_a < current_macd_b;

        exit_condition
    }

    fn entry_short(
        &self,
        index: usize,
        instrument: &Instrument,
        higher_tm_instrument: &HigherTMInstrument,
    ) -> bool {
        match self.strategy_type {
            StrategyType::LongShort => self.exit_long(index, instrument, higher_tm_instrument),
            StrategyType::OnlyShort => self.exit_long(index, instrument, higher_tm_instrument),
            _ => false,
        }
    }

    fn exit_short(
        &self,
        index: usize,
        instrument: &Instrument,
        higher_tm_instrument: &HigherTMInstrument,
    ) -> bool {
        match self.strategy_type {
            StrategyType::LongShort => self.entry_long(index, instrument, higher_tm_instrument),
            StrategyType::OnlyShort => self.entry_long(index, instrument, higher_tm_instrument),
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
