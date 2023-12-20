use super::strategy::*;
use crate::helpers::backtest::resolve_backtest;

use rs_algo_shared::error::Result;
use rs_algo_shared::indicators::Indicator;
use rs_algo_shared::models::order::{Order, OrderDirection, OrderType};
use rs_algo_shared::models::stop_loss::*;
use rs_algo_shared::models::strategy::StrategyType;
use rs_algo_shared::models::tick::InstrumentTick;
use rs_algo_shared::models::time_frame::{TimeFrame, TimeFrameType};
use rs_algo_shared::models::trade::{Position, TradeDirection, TradeIn, TradeOut};
use rs_algo_shared::models::{backtest_instrument::*, time_frame};
use rs_algo_shared::scanner::candle::CandleType;
use rs_algo_shared::scanner::instrument::*;

#[derive(Clone)]
pub struct NumBars<'a> {
    name: &'a str,
    time_frame: TimeFrameType,
    higher_time_frame: Option<TimeFrameType>,
    strategy_type: StrategyType,
    trading_direction: TradeDirection,
    order_size: f64,
    risk_reward_ratio: f64,
    profit_target: f64,
}

impl<'a> Strategy for NumBars<'a> {
    fn new(
        time_frame: Option<&str>,
        higher_time_frame: Option<&str>,
        strategy_type: Option<StrategyType>,
    ) -> Result<Self> {
        let risk_reward_ratio = std::env::var("RISK_REWARD_RATIO")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let profit_target = std::env::var("PIPS_PROFIT_TARGET")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let base_time_frame = &std::env::var("TIME_FRAME")
            .unwrap()
            .parse::<String>()
            .unwrap();

        let order_size = std::env::var("ORDER_SIZE").unwrap().parse::<f64>().unwrap();

        let strategy_type = match strategy_type {
            Some(stype) => stype,
            None => StrategyType::OnlyLongMTF,
        };

        let time_frame = match time_frame {
            Some(tf) => TimeFrame::new(tf),
            None => TimeFrame::new(base_time_frame),
        };

        let higher_time_frame = match higher_time_frame {
            Some(htf) => Some(TimeFrame::new(htf)),
            None => match strategy_type.is_multi_timeframe() {
                true => Some(TimeFrame::new(
                    &std::env::var("HIGHER_TIME_FRAME")
                        .unwrap()
                        .parse::<String>()
                        .unwrap(),
                )),
                false => None,
            },
        };

        let trading_direction = TradeDirection::Long;

        Ok(Self {
            name: "Num_Bars_3_Atr68",
            time_frame,
            higher_time_frame,
            order_size,
            strategy_type,
            trading_direction,
            risk_reward_ratio,
            profit_target,
        })
    }

    fn name(&self) -> &str {
        self.name
    }

    fn strategy_type(&self) -> &StrategyType {
        &self.strategy_type
    }

    fn time_frame(&self) -> &TimeFrameType {
        &self.time_frame
    }

    fn higher_time_frame(&self) -> &Option<TimeFrameType> {
        &self.higher_time_frame
    }

    fn trading_direction(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HTFInstrument,
    ) -> &TradeDirection {
        self.trading_direction = time_frame::get_htf_trading_direction(
            index,
            instrument,
            htf_instrument,
            |(idx, _prev_idx, htf_inst)| {
                // let prev_htf_ema_a = if let Some(value) = htf_inst
                //     .indicators
                //     .ema_a
                //     .get_data_a()
                //     .get(idx.checked_sub(1).unwrap_or_default())
                // {
                //     *value
                // } else {
                //     0.
                // };

                // let prev_htf_ema_b = if let Some(value) = htf_inst
                //     .indicators
                //     .ema_b
                //     .get_data_a()
                //     .get(idx.checked_sub(1).unwrap_or_default())
                // {
                //     *value
                // } else {
                //     0.
                // };

                let ema_percentage_dis = std::env::var("EMA_PERCENTAGE_DIS")
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();

                let htf_ema_a = htf_inst.indicators.ema_a.get_data_a().get(idx).unwrap();
                let htf_ema_b = htf_inst.indicators.ema_b.get_data_a().get(idx).unwrap();

                let percentage_diff = {
                    let numerator = (htf_ema_a - htf_ema_b).abs();
                    let denominator = ((htf_ema_a + htf_ema_b) / 2.0).abs();
                    (numerator / denominator) * 100.0
                };

                let is_long = htf_ema_a > htf_ema_b;
                let is_short = htf_ema_a < htf_ema_b;

                if is_long {
                    TradeDirection::Long
                } else if is_short {
                    TradeDirection::Short
                } else {
                    TradeDirection::None
                }
            },
        );
        &self.trading_direction
    }

    fn entry_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        _htf_instrument: &HTFInstrument,
        tick: &InstrumentTick,
    ) -> Position {
        let atr_stop_value = std::env::var("ATR_STOP_LOSS")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let atr_profit_value = std::env::var("ATR_PROFIT_TARGET")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let atr_value = instrument.indicators.atr.get_data_a().get(index).unwrap();

        let data = &instrument.data();
        let candle = data.get(index).unwrap();
        let is_closed: bool = candle.is_closed();

        let buy_price = candle.close() + tick.spread();
        let sell_price = buy_price + (atr_profit_value * atr_value);
        let entry_condition = candle.candle_type() == &CandleType::BearishThreeInRow && is_closed;

        match entry_condition {
            true => Position::MarketIn(Some(vec![
                OrderType::SellOrderLong(OrderDirection::Up, self.order_size, sell_price),
                OrderType::StopLossLong(OrderDirection::Down, StopLossType::Atr(atr_stop_value)),
            ])),
            false => Position::None,
        }
    }

    fn exit_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        _htf_instrument: &HTFInstrument,
        trade_in: &TradeIn,
        _tick: &InstrumentTick,
    ) -> Position {
        let exit_condition = self.trading_direction == TradeDirection::Short;
        match exit_condition {
            true => Position::MarketOut(None),
            false => Position::None,
        }
    }

    fn entry_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        _htf_instrument: &HTFInstrument,
        tick: &InstrumentTick,
    ) -> Position {
        let atr_stop_value = std::env::var("ATR_STOP_LOSS")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let atr_profit_value = std::env::var("ATR_PROFIT_TARGET")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let atr_value = instrument.indicators.atr.get_data_a().get(index).unwrap();

        let data = instrument.data();
        let candle = data.get(index).unwrap();
        let is_closed: bool = candle.is_closed();

        let buy_price = candle.close();
        let sell_price = buy_price - (atr_profit_value * atr_value) - tick.spread();
        let entry_condition = candle.candle_type() == &CandleType::ThreeInRow && is_closed;

        match entry_condition {
            true => Position::MarketIn(Some(vec![
                OrderType::SellOrderShort(OrderDirection::Down, self.order_size, sell_price),
                OrderType::StopLossLong(OrderDirection::Up, StopLossType::Atr(atr_stop_value)),
            ])),
            false => Position::None,
        }
    }

    fn exit_short(
        &mut self,
        _index: usize,
        _instrument: &Instrument,
        _htf_instrument: &HTFInstrument,
        _trade_in: &TradeIn,
        _tick: &InstrumentTick,
    ) -> Position {
        let exit_condition = self.trading_direction == TradeDirection::Long;
        match exit_condition {
            true => Position::MarketOut(None),
            false => Position::None,
        }
    }

    fn backtest_result(
        &self,
        instrument: &Instrument,
        trades_in: Vec<TradeIn>,
        trades_out: Vec<TradeOut>,
        orders: Vec<Order>,
        equity: f64,
        commission: f64,
    ) -> BackTestResult {
        resolve_backtest(
            instrument,
            &self.time_frame,
            &self.higher_time_frame,
            &self.strategy_type,
            trades_in,
            trades_out,
            orders,
            self.name,
            equity,
            commission,
        )
    }
}
