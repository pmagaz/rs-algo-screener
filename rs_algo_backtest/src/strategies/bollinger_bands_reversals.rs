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
pub struct BollingerBandsReversals<'a> {
    name: &'a str,
    time_frame: TimeFrameType,
    higher_time_frame: Option<TimeFrameType>,
    strategy_type: StrategyType,
    risk_reward_ratio: f64,
    profit_target: f64,
}

impl<'a> Strategy for BollingerBandsReversals<'a> {
    fn new() -> Result<Self> {
        let risk_reward_ratio = std::env::var("RISK_REWARD_RATIO")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let profit_target = std::env::var("PIPS_PROFIT_TARGET")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let time_frame = std::env::var("BASE_TIME_FRAME")
            .unwrap()
            .parse::<String>()
            .unwrap();

        let higher_time_frame = std::env::var("HIGHER_TIME_FRAME")
            .unwrap()
            .parse::<String>()
            .unwrap();

        let strategy_type = StrategyType::OnlyLongMTF;

        let higher_time_frame = match strategy_type.is_multi_timeframe() {
            true => Some(TimeFrame::new(&higher_time_frame)),
            false => None,
        };

        Ok(Self {
            name: "BollingerBandsReversals",
            time_frame: TimeFrame::new(&time_frame),
            higher_time_frame: higher_time_frame,
            strategy_type,
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

    fn entry_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HigherTMInstrument,
        pricing: &Pricing,
    ) -> Position {
        let spread = pricing.spread();
        let close_price = &instrument.data.get(index).unwrap().close();
        let anchor_htf = time_frame::get_htf_data(
            index,
            instrument,
            htf_instrument,
            |(idx, _prev_idx, upper_inst)| {
                let htf_ema_5 = upper_inst.indicators.ema_a.get_data_a().get(idx).unwrap();
                let htf_ema_13 = upper_inst.indicators.ema_c.get_data_a().get(idx).unwrap();
                htf_ema_5 > htf_ema_13 && close_price > htf_ema_13
            },
        );

        let prev_index = calc::get_prev_index(index);
        let data = &instrument.data();
        let candle = data.get(index).unwrap();
        let prev_candle = &data.get(prev_index).unwrap();
        let close_price = &candle.close();
        let prev_close = &prev_candle.close();
        let date = &candle.date();
        let pips_margin = 3.;
        let previous_bars = 5;
        let top_band = instrument.indicators.bb.get_data_a().get(index).unwrap();
        let prev_top_band = instrument
            .indicators
            .bb
            .get_data_a()
            .get(prev_index)
            .unwrap();

        let low_band = instrument.indicators.bb.get_data_b().get(index).unwrap();
        let prev_low_band = instrument
            .indicators
            .bb
            .get_data_b()
            .get(prev_index)
            .unwrap();

        let highest_bar = data[prev_index - previous_bars..prev_index]
            .iter()
            .max_by(|x, y| x.high().partial_cmp(&y.high()).unwrap())
            .map(|x| x.high())
            .unwrap();

        let entry_condition = anchor_htf && close_price < low_band && prev_close >= prev_low_band;
        let buy_price = close_price + calc::to_pips(pips_margin, pricing);
        let stop_loss_price = candle.low() - calc::to_pips(pips_margin, pricing);
        let risk = buy_price + spread - stop_loss_price;
        let sell_price = buy_price + (risk * self.risk_reward_ratio);
        //let sell_price = *top_band;

        match entry_condition {
            true => Position::Order(vec![
                OrderType::BuyOrderLong(OrderDirection::Up, *close_price, buy_price),
                //OrderType::SellOrderLong(OrderDirection::Up, *close_price, sell_price),
                OrderType::StopLoss(OrderDirection::Down, StopLossType::Price(stop_loss_price)),
            ]),

            false => Position::None,
        }
    }

    fn exit_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HigherTMInstrument,
        pricing: &Pricing,
    ) -> Position {
        let spread = pricing.spread();
        let close_price = &instrument.data.get(index).unwrap().close();
        // let anchor_htf = time_frame::get_htf_data(
        //     index,
        //     instrument,
        //     htf_instrument,
        //     |(idx, _prev_idx, upper_inst)| {
        //         let macd_a = upper_inst.indicators.macd.get_data_a().get(idx).unwrap();
        //         let macd_b = upper_inst.indicators.macd.get_data_b().get(idx).unwrap();
        //         macd_a < macd_b
        //     },
        // );

        let anchor_htf = time_frame::get_htf_data(
            index,
            instrument,
            htf_instrument,
            |(idx, _prev_idx, upper_inst)| {
                let htf_ema_5 = upper_inst.indicators.ema_a.get_data_a().get(idx).unwrap();
                let htf_ema_13 = upper_inst.indicators.ema_c.get_data_a().get(idx).unwrap();
                htf_ema_5 < htf_ema_13 && close_price < htf_ema_13
            },
        );

        let prev_index = calc::get_prev_index(index);
        let data = &instrument.data();
        let candle = data.get(index).unwrap();
        let prev_candle = &data.get(prev_index).unwrap();
        let close_price = &candle.close();
        let prev_close = &prev_candle.close();
        let prev_high = &prev_candle.high();
        let date = &candle.date();
        let pips_margin = 5.;
        let previous_bars = 5;
        let top_band = instrument.indicators.bb.get_data_a().get(index).unwrap();
        let prev_top_band = instrument
            .indicators
            .bb
            .get_data_a()
            .get(prev_index)
            .unwrap();

        let low_band = instrument.indicators.bb.get_data_b().get(index).unwrap();
        let prev_low_band = instrument
            .indicators
            .bb
            .get_data_b()
            .get(prev_index)
            .unwrap();

        let mut ridding_bars = 0;
        for candle in data[prev_index - previous_bars..prev_index].iter() {
            if candle.close() > *top_band {
                ridding_bars += 1;
            }
        }

        let exit_condition =
            anchor_htf || (ridding_bars < 3 && close_price < top_band && prev_high > prev_top_band);
        //let buy_price = highest_bar + calc::to_pips(pips_margin, pricing);
        // let stop_loss_price = candle.low() - calc::to_pips(pips_margin, pricing);
        // let risk = buy_price + spread - stop_loss_price;
        // let sell_price = buy_price + (risk * self.risk_reward_ratio);
        let sell_price = *top_band;

        match exit_condition {
            true => Position::MarketOut(None),
            // true => Position::Order(vec![
            //     OrderType::BuyOrderLong(OrderDirection::Up, *close_price, buy_price),
            //     //OrderType::SellOrderLong(OrderDirection::Up, *close_price, sell_price),
            //     //OrderType::StopLoss(OrderDirection::Down, StopLossType::Atr(atr_value)),
            //     //OrderType::StopLoss(OrderDirection::Down, StopLossType::Price(stop_loss_price)),
            // ]),
            false => Position::None,
        }
    }

    fn entry_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HigherTMInstrument,
        pricing: &Pricing,
    ) -> Position {
        // match self.strategy_type {
        //     StrategyType::LongShort => self.exit_long(index, instrument, htf_instrument),
        //     StrategyType::LongShortMTF => {
        //         self.exit_long(index, instrument, htf_instrument)
        //     }
        //     StrategyType::OnlyShort => self.exit_long(index, instrument, htf_instrument),
        //     _ => false,
        // }
        Position::None
    }

    fn exit_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HigherTMInstrument,
        pricing: &Pricing,
    ) -> Position {
        // match self.strategy_type {
        //     StrategyType::LongShort => self.entry_long(index, instrument, htf_instrument),
        //     StrategyType::LongShortMTF => {
        //         self.entry_long(index, instrument, htf_instrument)
        //     }
        //     StrategyType::OnlyShort => self.entry_long(index, instrument, htf_instrument),
        //     _ => false,
        // }
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
