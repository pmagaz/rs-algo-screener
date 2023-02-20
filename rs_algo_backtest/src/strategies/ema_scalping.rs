use super::strategy::*;
use crate::helpers::backtest::resolve_backtest;

use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::calc;
use rs_algo_shared::indicators::Indicator;
use rs_algo_shared::models::order::{Order, OrderDirection, OrderType};
use rs_algo_shared::models::pricing::Pricing;
use rs_algo_shared::models::stop_loss::*;
use rs_algo_shared::models::strategy::StrategyType;
use rs_algo_shared::models::time_frame::{TimeFrame, TimeFrameType};
use rs_algo_shared::models::trade::{Position, TradeIn, TradeOut};
use rs_algo_shared::models::{backtest_instrument::*, time_frame};
use rs_algo_shared::scanner::instrument::*;

#[derive(Clone)]
pub struct EmaScalping<'a> {
    name: &'a str,
    time_frame: TimeFrameType,
    higher_time_frame: Option<TimeFrameType>,
    strategy_type: StrategyType,
    order_size: f64,
    risk_reward_ratio: f64,
    profit_target: f64,
}

impl<'a> Strategy for EmaScalping<'a> {
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
            .unwrap()
            .clone();

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

        Ok(Self {
            name: "Ema_Scalping",
            time_frame,
            higher_time_frame,
            strategy_type,
            order_size,
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
        &&self.time_frame
    }

    fn higher_time_frame(&self) -> &Option<TimeFrameType> {
        &self.higher_time_frame
    }

    fn entry_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HTFInstrument,
        pricing: &Pricing,
    ) -> Position {
        let close_price = &instrument.data.get(index).unwrap().close();
        let spread = 0.;

        let anchor_htf = time_frame::get_htf_data(
            index,
            instrument,
            htf_instrument,
            |(idx, _prev_idx, htf_inst)| {
                let htf_ema_5 = htf_inst.indicators.ema_a.get_data_a().get(idx).unwrap();
                let htf_ema_13 = htf_inst.indicators.ema_c.get_data_a().get(idx).unwrap();
                htf_ema_5 > htf_ema_13 && close_price > htf_ema_13
            },
        );

        let prev_index = calc::get_prev_index(index);
        let data = &instrument.data();
        let candle = data.get(index).unwrap();
        let trigger_price = &candle.low();
        let low_price = &candle.low();
        let prev_close_price = &data.get(prev_index).unwrap().close();
        let ema_5 = instrument.indicators.ema_a.get_data_a().get(index).unwrap();
        let prev_ema_5 = instrument
            .indicators
            .ema_a
            .get_data_a()
            .get(prev_index)
            .unwrap();
        let ema_8 = instrument.indicators.ema_b.get_data_a().get(index).unwrap();
        let ema_13 = instrument.indicators.ema_c.get_data_a().get(index).unwrap();

        let entry_condition = anchor_htf
            && (low_price < ema_5
                && prev_close_price >= prev_ema_5
                && close_price > ema_13
                && ema_5 > ema_8
                && ema_8 > ema_13);

        let pips_margin = 5.;
        let previous_bars = 5;

        let highest_bar = data[index - previous_bars..index + 1]
            .iter()
            .max_by(|x, y| x.high().partial_cmp(&y.high()).unwrap())
            .map(|x| x.high())
            .unwrap();

        let buy_price = highest_bar + calc::to_pips(pips_margin, pricing);
        let stop_loss_price = trigger_price - calc::to_pips(pips_margin, pricing);
        let risk = buy_price + spread - stop_loss_price;
        let sell_price = buy_price + (risk * self.risk_reward_ratio);

        match entry_condition {
            true => Position::Order(vec![
                OrderType::BuyOrderLong(OrderDirection::Up, self.order_size, buy_price),
                OrderType::SellOrderLong(OrderDirection::Up, self.order_size, sell_price),
                //OrderType::StopLoss(OrderDirection::Down, StopLossType::Atr(atr_value)),
                OrderType::StopLossLong(OrderDirection::Down, StopLossType::Price(stop_loss_price)),
            ]),

            false => Position::None,
        }
    }

    fn exit_long(
        &mut self,
        _index: usize,
        _instrument: &Instrument,
        _htf_instrument: &HTFInstrument,
        _trade_in: &TradeIn,
        _pricing: &Pricing,
    ) -> Position {
        Position::None
    }

    fn entry_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HTFInstrument,
        pricing: &Pricing,
    ) -> Position {
        let close_price = &instrument.data.get(index).unwrap().close();
        let spread = 0.;
        let anchor_htf = time_frame::get_htf_data(
            index,
            instrument,
            htf_instrument,
            |(idx, _prev_idx, htf_inst)| {
                let htf_ema_5 = htf_inst.indicators.ema_a.get_data_a().get(idx).unwrap();
                let htf_ema_13 = htf_inst.indicators.ema_c.get_data_a().get(idx).unwrap();
                htf_ema_5 < htf_ema_13 && close_price < htf_ema_13
            },
        );

        let prev_index = calc::get_prev_index(index);
        let data = &instrument.data();
        let candle = &data.get(index).unwrap();
        let prev_candle = &data.get(prev_index).unwrap();
        let trigger_price = &candle.high();
        let close_price = &candle.close();
        let prev_close_price = &prev_candle.close();
        let ema_5 = instrument.indicators.ema_a.get_data_a().get(index).unwrap();
        let prev_ema_5 = instrument
            .indicators
            .ema_a
            .get_data_a()
            .get(prev_index)
            .unwrap();
        let ema_8 = instrument.indicators.ema_b.get_data_a().get(index).unwrap();
        let ema_8 = instrument.indicators.ema_c.get_data_a().get(index).unwrap();

        let entry_condition = anchor_htf
            && (trigger_price > ema_5
                && prev_close_price <= prev_ema_5
                && close_price < ema_8
                && ema_5 < ema_8
                && ema_8 < ema_8);

        let pips_margin = 5.;
        let previous_bars = 5;

        let lowest_bar = data[index - previous_bars..index + 1]
            .iter()
            .min_by(|x, y| x.low().partial_cmp(&y.low()).unwrap())
            .map(|x| x.low())
            .unwrap();

        let buy_price = lowest_bar - calc::to_pips(pips_margin, pricing);
        let stop_loss_price = trigger_price + calc::to_pips(pips_margin, pricing);
        let risk = stop_loss_price + spread - buy_price;
        let sell_price = buy_price - (risk * self.risk_reward_ratio);

        match entry_condition {
            true => Position::Order(vec![
                OrderType::BuyOrderShort(OrderDirection::Down, self.order_size, buy_price),
                OrderType::SellOrderShort(OrderDirection::Down, self.order_size, sell_price),
                OrderType::StopLossShort(OrderDirection::Up, StopLossType::Price(stop_loss_price)),
            ]),

            false => Position::None,
        }
    }

    fn exit_short(
        &mut self,
        _index: usize,
        _instrument: &Instrument,
        _htf_instrument: &HTFInstrument,
        _pricing: &Pricing,
    ) -> Position {
        Position::None
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
