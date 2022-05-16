use super::strategy::Strategy;

use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::Instrument;

pub struct Tema<'a> {
    name: &'a str,
}

#[async_trait]
impl<'a> Strategy for Tema<'a> {
    fn new() -> Result<Self> {
        Ok(Self { name: "TEMA_821" })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn market_in_fn(&self, index: usize, instrument: &Instrument, stop_loss: f64) -> TradeResult {
        let prev_index = index - 1;

        let tema_a = instrument.indicators.tema_a.data_a.get(index).unwrap();
        let tema_b = instrument.indicators.tema_b.data_a.get(index).unwrap();

        let prev_tema_b = instrument.indicators.tema_b.data_a.get(prev_index).unwrap();
        let prev_tema_a = instrument.indicators.tema_a.data_a.get(prev_index).unwrap();

        let entry_condition = tema_a > tema_b && prev_tema_a <= prev_tema_b;

        resolve_trade_in(index, instrument, entry_condition, stop_loss)
    }

    fn market_out_fn(
        &self,
        index: usize,
        instrument: &Instrument,
        trade_in: &TradeIn,
    ) -> TradeResult {
        let prev_index = index - 1;

        let tema_a = instrument.indicators.tema_a.data_a.get(index).unwrap();
        let tema_b = instrument.indicators.tema_b.data_a.get(index).unwrap();

        let prev_tema_b = instrument.indicators.tema_b.data_a.get(prev_index).unwrap();
        let prev_tema_a = instrument.indicators.tema_a.data_a.get(prev_index).unwrap();

        let exit_condition = tema_a < tema_b && prev_tema_a >= prev_tema_b;

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
