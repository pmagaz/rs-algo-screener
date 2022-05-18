use super::strategy::Strategy;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::Instrument;

pub struct BollingerBands<'a> {
    name: &'a str,
}

#[async_trait]
impl<'a> Strategy for BollingerBands<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            name: "BollingerBands",
        })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn market_in_fn(&self, index: usize, instrument: &Instrument, stop_loss: f64) -> TradeResult {
        let prev_index = get_prev_index(index);

        let current_price = &instrument.data.get(index).unwrap().close;
        let prev_price = &instrument.data.get(prev_index).unwrap().close;

        let lower_band = instrument.indicators.bb.data_b.get(index).unwrap();
        let prev_lower_band = instrument.indicators.bb.data_b.get(prev_index).unwrap();

        let entry_condition = current_price < lower_band && prev_price >= prev_lower_band;

        resolve_trade_in(index, instrument, entry_condition, stop_loss)
    }

    fn market_out_fn(
        &self,
        index: usize,
        instrument: &Instrument,
        trade_in: &TradeIn,
    ) -> TradeResult {
        let prev_index = get_prev_index(index);

        let current_price = &instrument.data.get(index).unwrap().close;
        let prev_price = &instrument.data.get(prev_index).unwrap().close;

        let upper_band = instrument.indicators.bb.data_a.get(index).unwrap();
        let prev_upper_band = instrument.indicators.bb.data_a.get(prev_index).unwrap();

        let exit_condition = current_price > upper_band && prev_price <= prev_upper_band;

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
