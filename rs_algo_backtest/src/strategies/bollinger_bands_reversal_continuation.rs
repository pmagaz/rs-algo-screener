use super::strategy::Strategy;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::Instrument;
use rs_algo_shared::models::pattern::*;

pub struct BollingerBands<'a> {
    name: &'a str,
}

#[async_trait]
impl<'a> Strategy for BollingerBands<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            name: "Bollinger_Bands_Reversal_Continuation",
        })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn market_in_fn(&self, index: usize, instrument: &Instrument, stop_loss: f64) -> TradeResult {
        let prev_index = get_prev_index(index);

        let close_price = &instrument.data.get(index).unwrap().close;
        let prev_close = &instrument.data.get(prev_index).unwrap().close;

        let low_band = instrument.indicators.bb.data_b.get(index).unwrap();
        let prev_low_band = instrument.indicators.bb.data_b.get(prev_index).unwrap();

        let entry_condition = close_price < low_band && prev_close >= prev_low_band;

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
        let prev_close = &instrument.data.get(prev_index).unwrap().close;

        let top_band = instrument.indicators.bb.data_a.get(index).unwrap();
        let prev_top_band = instrument.indicators.bb.data_a.get(prev_index).unwrap();

        let patterns = &instrument.patterns.local_patterns;
        let current_pattern = get_current_pattern(index, patterns);
        let _low_band = instrument.indicators.bb.data_b.get(index).unwrap();
        let _prev_low_band = instrument.indicators.bb.data_b.get(prev_index).unwrap();
        let mut exit_condition: bool = false;

        if current_pattern == PatternType::ChannelUp
            || current_pattern == PatternType::HigherHighsHigherLows
        {
            //} || current_pattern == PatternType::TriangleUp {
            exit_condition = false;
        } else {
            exit_condition = close_price > top_band && prev_close <= prev_top_band;
        }

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
