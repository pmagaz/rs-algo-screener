use super::strategy::*;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::*;

#[derive(Clone)]
pub struct Macd<'a> {
    name: &'a str,
    strategy_type: StrategyType,
    stop_loss: StopLoss,
}

#[async_trait]
impl<'a> Strategy for Macd<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            stop_loss: init_stop_loss(),
            name: "MACD",
            strategy_type: StrategyType::OnlyLong,
        })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn strategy_type(&self) -> &StrategyType {
        &self.strategy_type
    }

    fn update_stop_loss(&mut self, stop_type: StopLossType, price: f64) -> &StopLoss {
        self.stop_loss = update_stop_loss_values(&self.stop_loss, stop_type, price);
        &self.stop_loss
    }
    fn stop_loss(&self) -> &StopLoss {
        &self.stop_loss
    }

    fn entry_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        _upper_tf_instrument: &HigherTMInstrument,
    ) -> bool {
        let prev_index = get_prev_index(index);

        let current_macd_a = instrument.indicators.macd.data_a.get(index).unwrap();
        let current_macd_b = instrument.indicators.macd.data_b.get(index).unwrap();

        let prev_macd_a = instrument.indicators.macd.data_a.get(prev_index).unwrap();
        let prev_macd_b = instrument.indicators.macd.data_a.get(prev_index).unwrap();

        current_macd_a > current_macd_b && prev_macd_b >= prev_macd_a
    }

    fn exit_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        _upper_tf_instrument: &HigherTMInstrument,
    ) -> bool {
        let prev_index = get_prev_index(index);

        let current_macd_a = instrument.indicators.macd.data_a.get(index).unwrap();
        let current_macd_b = instrument.indicators.macd.data_b.get(index).unwrap();

        let prev_macd_a = instrument.indicators.macd.data_a.get(prev_index).unwrap();
        let prev_macd_b = instrument.indicators.macd.data_a.get(prev_index).unwrap();

        current_macd_a < current_macd_b && prev_macd_a >= prev_macd_b
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
