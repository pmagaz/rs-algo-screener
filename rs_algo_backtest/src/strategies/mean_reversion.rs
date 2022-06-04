use super::strategy::Strategy;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::Instrument;

pub struct MeanReversion<'a> {
    name: &'a str,
}

#[async_trait]
impl<'a> Strategy for MeanReversion<'a> {
    fn new() -> Result<Self> {
        Ok(Self { name: "MEAN_R" })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn market_in_fn(&self, index: usize, instrument: &Instrument, stop_loss: f64) -> TradeResult {
        let prev_index = get_prev_index(index);

        let close_price = &instrument.data.get(index).unwrap().close;
        let current_ema_200 = instrument.indicators.ema_c.data_a.get(index).unwrap();
        let _prev_close = &instrument.data.get(prev_index).unwrap().close;
        let _prev_close = &instrument.data.get(prev_index).unwrap().close;
        let _prev_ema_200 = instrument.indicators.ema_c.data_a.get(prev_index).unwrap();

        let entry_condition = close_price > current_ema_200;

        resolve_trade_in(index, instrument, entry_condition, stop_loss)
    }

    fn market_out_fn(
        &self,
        index: usize,
        instrument: &Instrument,
        trade_in: &TradeIn,
    ) -> TradeResult {
        let prev_index = get_prev_index(index);

        let close_price = &instrument.data.get(index).unwrap().close;
        let current_ema_200 = instrument.indicators.ema_c.data_a.get(index).unwrap();
        let _prev_close = &instrument.data.get(prev_index).unwrap().close;
        let _prev_ema_200 = instrument.indicators.ema_c.data_a.get(prev_index).unwrap();

        let exit_condition = close_price < current_ema_200;
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
