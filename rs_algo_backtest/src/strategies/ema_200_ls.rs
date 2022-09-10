use super::strategy::*;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::*;

#[derive(Clone)]
pub struct Ema<'a> {
    name: &'a str,
     strategy_type: StrategyType,
     stop_loss: f64
}

#[async_trait]
impl<'a> Strategy for Ema<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            stop_loss: 0.,
            name: "EMA_200",
            strategy_type: StrategyType::LongShort,
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

        let close_price = &instrument.data.get(index).unwrap().close;
        let current_ema_200 = instrument.indicators.ema_c.data_a.get(index).unwrap();

        let prev_close = &instrument.data.get(prev_index).unwrap().close;
        let prev_ema_200 = instrument.indicators.ema_c.data_a.get(prev_index).unwrap();

        let entry_condition = close_price > current_ema_200 && prev_close <= prev_ema_200;

        entry_condition
    }

    fn exit_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool {
        let prev_index = get_prev_index(index);
        let close_price = &instrument.data.get(index).unwrap().close;
        let current_ema_200 = instrument.indicators.ema_c.data_a.get(index).unwrap();

        let prev_close = &instrument.data.get(prev_index).unwrap().close;
        let prev_ema_200 = instrument.indicators.ema_c.data_a.get(prev_index).unwrap();

        let exit_condition = close_price < current_ema_200 && prev_close >= prev_ema_200;

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
