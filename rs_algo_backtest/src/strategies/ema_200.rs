use super::strategy::Strategy;

use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::Instrument;

pub struct Ema<'a> {
    name: &'a str,
}

#[async_trait]
impl<'a> Strategy for Ema<'a> {
    fn new() -> Result<Self> {
        Ok(Self { name: "EMA_200" })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn market_in_fn(&self, index: usize, instrument: &Instrument, stop_loss: f64) -> TradeResult {
        let prev_index = index - 1;
        let current_price = &instrument.data.get(index).unwrap().close;
        let current_ema_200 = instrument.indicators.ema_c.data_a.get(index).unwrap();
        let prev_price = &instrument.data.get(prev_index).unwrap().close;
        let prev_price = &instrument.data.get(prev_index).unwrap().close;
        let prev_ema_200 = instrument.indicators.ema_c.data_a.get(prev_index).unwrap();

        let entry_condition = current_price > current_ema_200;

        resolve_trade_in(index, instrument, entry_condition, stop_loss)
    }

    fn market_out_fn(
        &self,
        index: usize,
        instrument: &Instrument,
        trade_in: &TradeIn,
    ) -> TradeResult {
        let prev_index = index - 1;
        let current_price = &instrument.data.get(index).unwrap().close;
        let current_ema_200 = instrument.indicators.ema_c.data_a.get(index).unwrap();
        let prev_price = &instrument.data.get(prev_index).unwrap().close;
        let prev_ema_200 = instrument.indicators.ema_c.data_a.get(prev_index).unwrap();

        let exit_condition = current_price < current_ema_200;
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
