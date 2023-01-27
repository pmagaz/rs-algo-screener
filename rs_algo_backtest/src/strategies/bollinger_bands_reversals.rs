use super::strategy::*;
use crate::helpers::backtest::resolve_backtest;

use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::calc;
use rs_algo_shared::indicators::Indicator;
use rs_algo_shared::models::order::{Order, OrderDirection, OrderType};
use rs_algo_shared::models::pricing::Pricing;
use rs_algo_shared::models::stop_loss::*;
use rs_algo_shared::models::strategy::StrategyType;
use rs_algo_shared::models::trade::{Position, TradeIn, TradeOut};
use rs_algo_shared::models::{backtest_instrument::*, time_frame};
use rs_algo_shared::scanner::candle::Candle;
use rs_algo_shared::scanner::instrument::*;

#[derive(Clone)]
pub struct BollingerBandsReversals<'a> {
    name: &'a str,
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

        Ok(Self {
            name: "BollingerBandsReversals",
            strategy_type: StrategyType::OnlyLongMultiTF,
            //strategy_type: StrategyType::LongShortMultiTF,
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
        upper_tf_instrument: &HigherTMInstrument,
        pricing: &Pricing,
    ) -> Position {
        let spread = pricing.spread();

        let anchor_htf = time_frame::get_htf_data(
            index,
            instrument,
            upper_tf_instrument,
            |(idx, _prev_idx, upper_inst)| {
                let macd_a = upper_inst.indicators.macd.get_data_a().get(idx).unwrap();
                let macd_b = upper_inst.indicators.macd.get_data_b().get(idx).unwrap();
                macd_a > macd_b
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
        let buy_price = highest_bar + calc::to_pips(pips_margin, pricing);
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
        upper_tf_instrument: &HigherTMInstrument,
        pricing: &Pricing,
    ) -> Position {
        // let _upper_macd = get_htf_data(
        //     index,
        //     instrument,
        //     upper_tf_instrument,
        //     |(idx, prev_idx, upper_inst)| {
        //         let curr_upper_macd_a = upper_inst.indicators.macd.get_data_a().get(idx).unwrap();
        //         let curr_upper_macd_b = upper_inst.indicators.macd.get_data_b().get(idx).unwrap();

        //         let _prev_upper_macd_a = upper_inst
        //             .indicators
        //             .macd
        //             .get_data_a()
        //             .get(prev_idx)
        //             .unwrap();
        //         let _prev_upper_macd_b = upper_inst
        //             .indicators
        //             .macd
        //             .get_data_b()
        //             .get(prev_idx)
        //             .unwrap();
        //         curr_upper_macd_a < curr_upper_macd_b // && prev_upper_macd_a >= prev_upper_macd_b
        //     },
        // );

        // let prev_index = get_prev_index(index);
        // let low_price = &instrument.data.get(index).unwrap().low;
        // let date = &instrument.data.get(index).unwrap().date;

        // let close_price = &instrument.data.get(index).unwrap().close;
        // let prev_close = &instrument.data.get(prev_index).unwrap().close;

        // let top_band = instrument.indicators.bb.get_data_a().get(index).unwrap();
        // let prev_top_band = instrument
        //     .indicators
        //     .bb
        //     .get_data_a()
        //     .get(prev_index)
        //     .unwrap();

        // let exit_condition = close_price > top_band && prev_close <= prev_top_band;

        // if exit_condition {
        //     self.update_stop_loss(StopLossType::Trailing, *low_price);
        // }

        // false
        let spread = pricing.spread();

        let anchor_htf = time_frame::get_htf_data(
            index,
            instrument,
            upper_tf_instrument,
            |(idx, _prev_idx, upper_inst)| {
                let macd_a = upper_inst.indicators.macd.get_data_a().get(idx).unwrap();
                let macd_b = upper_inst.indicators.macd.get_data_b().get(idx).unwrap();
                macd_a < macd_b
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
        upper_tf_instrument: &HigherTMInstrument,
        pricing: &Pricing,
    ) -> Position {
        // match self.strategy_type {
        //     StrategyType::LongShort => self.exit_long(index, instrument, upper_tf_instrument),
        //     StrategyType::LongShortMultiTF => {
        //         self.exit_long(index, instrument, upper_tf_instrument)
        //     }
        //     StrategyType::OnlyShort => self.exit_long(index, instrument, upper_tf_instrument),
        //     _ => false,
        // }
        Position::None
    }

    fn exit_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        pricing: &Pricing,
    ) -> Position {
        // match self.strategy_type {
        //     StrategyType::LongShort => self.entry_long(index, instrument, upper_tf_instrument),
        //     StrategyType::LongShortMultiTF => {
        //         self.entry_long(index, instrument, upper_tf_instrument)
        //     }
        //     StrategyType::OnlyShort => self.entry_long(index, instrument, upper_tf_instrument),
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
