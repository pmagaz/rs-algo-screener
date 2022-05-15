use super::strategy::Strategy;

use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::Instrument;

pub struct Stoch<'a> {
    name: &'a str,
}

#[async_trait]
impl<'a> Strategy for Stoch<'a> {
    fn new() -> Result<Self> {
        Ok(Self { name: "Stoch" })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn market_in_fn(&self, index: usize, instrument: &Instrument, stop_loss: f64) -> TradeResult {
        let prev_index = index - 1;
        let current_stoch_a = instrument.indicators.stoch.data_a.get(index).unwrap();
        let prev_stoch_a = instrument.indicators.stoch.data_a.get(prev_index).unwrap();

        let current_stoch_b = instrument.indicators.stoch.data_b.get(index).unwrap();
        let prev_stoch_b = instrument.indicators.stoch.data_b.get(prev_index).unwrap();

        let entry_condition = current_stoch_a <= &20.
            && current_stoch_a > current_stoch_b
            && prev_stoch_a <= prev_stoch_b;

        resolve_trade_in(index, instrument, entry_condition, stop_loss)
    }

    fn market_out_fn(
        &self,
        index: usize,
        instrument: &Instrument,
        trade_in: &TradeIn,
    ) -> TradeResult {
        let prev_index = index - 1;
        let current_stoch_a = instrument.indicators.stoch.data_a.get(index).unwrap();
        let prev_stoch_a = instrument.indicators.stoch.data_a.get(prev_index).unwrap();

        let current_stoch_b = instrument.indicators.stoch.data_b.get(index).unwrap();
        let prev_stoch_b = instrument.indicators.stoch.data_b.get(prev_index).unwrap();

        let exit_condition = current_stoch_a >= &70.
            && current_stoch_a < current_stoch_b
            && prev_stoch_a >= prev_stoch_b;

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
