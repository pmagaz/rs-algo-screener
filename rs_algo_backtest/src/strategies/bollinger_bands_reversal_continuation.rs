use super::strategy::*;

use crate::helpers::calc::*;
use crate::trade::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::Instrument;
use rs_algo_shared::models::pattern::*;

pub struct BollingerBands<'a> {
    name: &'a str,
    strategy_type: StrategyType,
}

#[async_trait]
impl<'a> Strategy for BollingerBands<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            name: "Bollinger_Bands_Reversal_Continuation",
            strategy_type: StrategyType::OnlyLong,
        })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn strategy_type(&self) -> &StrategyType {
        &self.strategy_type
    }

    // fn market_in_fn(&self, index: usize, instrument: &Instrument, stop_loss: f64) -> TradeResult {
    //     let entry_type: TradeType;

    //     if self.entry_long(index, instrument) {
    //         entry_type = TradeType::EntryLong
    //     } else if self.entry_short(index, instrument) {
    //         entry_type = TradeType::EntryShort
    //     } else {
    //         entry_type = TradeType::None
    //     }

    //     resolve_trade_in2(index, instrument, entry_type, stop_loss)
    // }

    // fn market_out_fn(
    //     &self,
    //     index: usize,
    //     instrument: &Instrument,
    //     trade_in: &TradeIn,
    // ) -> TradeResult {
    //     let exit_type: TradeType;

    //     if self.exit_long(index, instrument) {
    //         exit_type = TradeType::ExitLong
    //     } else if self.exit_short(index, instrument) {
    //         exit_type = TradeType::ExitShort
    //     } else {
    //         exit_type = TradeType::None
    //     }
    //     let stop_loss = true;
    //     resolve_trade_out2(index, instrument, trade_in, exit_type, stop_loss)
    // }
    fn entry_long(&self, index: usize, instrument: &Instrument) -> bool {
        let prev_index = get_prev_index(index);

        let close_price = &instrument.data.get(index).unwrap().close;
        let prev_close = &instrument.data.get(prev_index).unwrap().close;

        let low_band = instrument.indicators.bb.data_b.get(index).unwrap();
        let prev_low_band = instrument.indicators.bb.data_b.get(prev_index).unwrap();

        let entry_condition = close_price < low_band && prev_close >= prev_low_band;
        entry_condition
    }

    fn exit_long(&self, index: usize, instrument: &Instrument) -> bool {
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
            exit_condition = false;
        } else {
            exit_condition = close_price > top_band && prev_close <= prev_top_band;
        }
        exit_condition
    }

    fn entry_short(&self, index: usize, instrument: &Instrument) -> bool {
        match self.strategy_type {
            StrategyType::LongShort => self.exit_long(index, instrument),
            StrategyType::OnlyShort => self.exit_long(index, instrument),
            _ => false,
        }
    }

    fn exit_short(&self, index: usize, instrument: &Instrument) -> bool {
        match self.strategy_type {
            StrategyType::LongShort => self.entry_long(index, instrument),
            StrategyType::OnlyShort => self.entry_long(index, instrument),
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
            instrument, trades_in, trades_out, self.name, equity, commission,
        )
    }
}
