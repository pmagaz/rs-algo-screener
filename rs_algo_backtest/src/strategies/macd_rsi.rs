use super::strategy::*;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::Instrument;

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

    fn market_in_fn(&self, index: usize, instrument: &Instrument, stop_loss: f64) -> TradeResult {
        let prev_index = get_prev_index(index);

        let current_rsi = instrument.indicators.rsi.data_a.get(index).unwrap();
        let _prev_rsi = instrument.indicators.rsi.data_a.get(prev_index).unwrap();

        let current_macd_a = instrument.indicators.macd.data_a.get(index).unwrap();
        let current_macd_b = instrument.indicators.macd.data_b.get(index).unwrap();

        let _prev_macd_a = instrument.indicators.macd.data_a.get(prev_index).unwrap();
        let _prev_macd_b = instrument.indicators.macd.data_b.get(prev_index).unwrap();

        let entry_condition = current_rsi <= &30. && current_macd_a > current_macd_b;

        resolve_trade_in(index, instrument, entry_condition, stop_loss)
    }

    fn market_out_fn(
        &self,
        index: usize,
        instrument: &Instrument,
        trade_in: &TradeIn,
    ) -> TradeResult {
        let prev_index = get_prev_index(index);

        let current_rsi = instrument.indicators.rsi.data_a.get(index).unwrap();
        let _prev_rsi = instrument.indicators.rsi.data_a.get(prev_index).unwrap();

        let current_macd_a = instrument.indicators.macd.data_a.get(index).unwrap();
        let current_macd_b = instrument.indicators.macd.data_b.get(index).unwrap();

        let _prev_macd_a = instrument.indicators.macd.data_a.get(prev_index).unwrap();
        let _prev_macd_b = instrument.indicators.macd.data_b.get(prev_index).unwrap();

        let exit_condition = current_rsi >= &70. && current_macd_a < current_macd_b;
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
