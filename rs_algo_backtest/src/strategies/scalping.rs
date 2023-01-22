use super::strategy::*;
use crate::helpers::backtest::resolve_backtest;

use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::calc;
use rs_algo_shared::indicators::Indicator;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::order::{Order, OrderCondition, OrderDirection, OrderType};
use rs_algo_shared::models::pricing::Pricing;
use rs_algo_shared::models::stop_loss::*;
use rs_algo_shared::models::strategy::StrategyType;
use rs_algo_shared::models::trade::{Operation, TradeIn, TradeOut};
use rs_algo_shared::scanner::instrument::*;

#[derive(Clone)]
pub struct Scalping<'a> {
    name: &'a str,
    strategy_type: StrategyType,
    risk_reward_ratio: f64,
    profit_target: f64,
    //stop_loss: StopLoss,
}

impl<'a> Strategy for Scalping<'a> {
    fn new() -> Result<Self> {
        let stop_loss = std::env::var("ATR_STOP_LOSS")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let risk_reward_ratio = std::env::var("RISK_REWARD_RATIO")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let profit_target = std::env::var("PIPS_PROFIT_TARGET")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        Ok(Self {
            //stop_loss: init_stop_loss(StopLossType::Atr, stop_loss),
            name: "Scalping",
            strategy_type: StrategyType::OnlyLongMultiTF,
            //strategy_type: StrategyType::OnlyShortMultiTF,
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
    ) -> Operation {
        let low_price = &instrument.data.get(index).unwrap().low();
        let close_price = &instrument.data.get(index).unwrap().close();
        let spread = pricing.spread();
        // let first_anchor_htf = calc::get_upper_timeframe_data(
        //     index,
        //     instrument,
        //     upper_tf_instrument,
        //     |(idx, prev_idx, upper_inst)| {
        //         let htf_ema_8 = upper_inst.indicators.ema_a.get_data_a().get(idx).unwrap();
        //         let prev_htf_ema_8 = upper_inst
        //             .indicators
        //             .ema_a
        //             .get_data_a()
        //             .get(prev_idx)
        //             .unwrap();
        //         let htf_ema_21 = upper_inst.indicators.ema_c.get_data_a().get(idx).unwrap();
        //         let prev_htf_ema_21 = upper_inst
        //             .indicators
        //             .ema_c
        //             .get_data_a()
        //             .get(prev_idx)
        //             .unwrap();
        //         htf_ema_8 > htf_ema_21 && prev_htf_ema_8 <= prev_htf_ema_21
        //     },
        // );
        let anchor_htf = calc::get_upper_timeframe_data(
            index,
            instrument,
            upper_tf_instrument,
            |(idx, prev_idx, upper_inst)| {
                let htf_ema_8 = upper_inst.indicators.ema_a.get_data_a().get(idx).unwrap();
                let prev_htf_ema_8 = upper_inst
                    .indicators
                    .ema_a
                    .get_data_a()
                    .get(prev_idx)
                    .unwrap();
                let htf_ema_21 = upper_inst.indicators.ema_c.get_data_a().get(idx).unwrap();
                let prev_htf_ema_21 = upper_inst
                    .indicators
                    .ema_c
                    .get_data_a()
                    .get(prev_idx)
                    .unwrap();
                htf_ema_8 > htf_ema_21 && close_price > htf_ema_21
            },
        );

        let prev_index = calc::get_prev_index(index);
        let next_index = index + 1;
        let data = &instrument.data();
        let low_price = &data.get(index).unwrap().low();
        let close_price = &data.get(index).unwrap().close();
        let prev_close_price = &data.get(prev_index).unwrap().close();
        let open_price = data.get(next_index).unwrap().close();
        let ema_8 = instrument.indicators.ema_a.get_data_a().get(index).unwrap();
        let prev_ema_8 = instrument
            .indicators
            .ema_a
            .get_data_a()
            .get(prev_index)
            .unwrap();
        let ema_13 = instrument.indicators.ema_b.get_data_a().get(index).unwrap();
        let ema_21 = instrument.indicators.ema_c.get_data_a().get(index).unwrap();

        let entry_condition = anchor_htf
            && (low_price < ema_8
                && prev_close_price >= prev_ema_8
                && close_price > ema_21
                && ema_8 > ema_13
                && ema_13 > ema_21);

        let pips_margin = 1.;

        let stop_loss_pips = std::env::var("PIPS_STOP_LOSS")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let trigger_price = data[index - 5..index]
            .iter()
            .max_by(|x, y| x.high().partial_cmp(&y.high()).unwrap())
            .map(|x| x.close())
            .unwrap()
            + calc::to_pips(&pips_margin, pricing);

        let buy_price = trigger_price + spread;

        let risk = buy_price - close_price - calc::to_pips(&pips_margin, pricing);
        let target_price = buy_price + calc::to_pips(&self.profit_target, pricing);
        match entry_condition {
            true => Operation::Order(vec![
                OrderType::BuyOrderLong(OrderDirection::Up, *close_price, trigger_price),
                OrderType::SellOrderLong(OrderDirection::Up, *close_price, target_price),
                OrderType::StopLoss(OrderDirection::Down, StopLossType::Pips(stop_loss_pips)),
            ]),

            false => Operation::None,
        }
    }

    fn exit_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        pricing: &Pricing,
    ) -> Operation {
        Operation::None
    }

    fn entry_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        pricing: &Pricing,
    ) -> Operation {
        let close_price = &instrument.data.get(index).unwrap().close();
        let spread = pricing.spread();
        // let first_anchor_htf = calc::get_upper_timeframe_data(
        //     index,
        //     instrument,
        //     upper_tf_instrument,
        //     |(idx, prev_idx, upper_inst)| {
        //         let htf_ema_8 = upper_inst.indicators.ema_a.get_data_a().get(idx).unwrap();
        //         let prev_htf_ema_8 = upper_inst
        //             .indicators
        //             .ema_a
        //             .get_data_a()
        //             .get(prev_idx)
        //             .unwrap();
        //         let htf_ema_21 = upper_inst.indicators.ema_c.get_data_a().get(idx).unwrap();
        //         let prev_htf_ema_21 = upper_inst
        //             .indicators
        //             .ema_c
        //             .get_data_a()
        //             .get(prev_idx)
        //             .unwrap();
        //         htf_ema_8 > htf_ema_21 && prev_htf_ema_8 <= prev_htf_ema_21
        //     },
        // );
        let anchor_htf = calc::get_upper_timeframe_data(
            index,
            instrument,
            upper_tf_instrument,
            |(idx, prev_idx, upper_inst)| {
                let htf_ema_8 = upper_inst.indicators.ema_a.get_data_a().get(idx).unwrap();
                let prev_htf_ema_8 = upper_inst
                    .indicators
                    .ema_a
                    .get_data_a()
                    .get(prev_idx)
                    .unwrap();
                let htf_ema_21 = upper_inst.indicators.ema_c.get_data_a().get(idx).unwrap();
                let prev_htf_ema_21 = upper_inst
                    .indicators
                    .ema_c
                    .get_data_a()
                    .get(prev_idx)
                    .unwrap();
                htf_ema_8 < htf_ema_21 && close_price < htf_ema_21
            },
        );

        let prev_index = calc::get_prev_index(index);
        let next_index = index + 1;
        let data = &instrument.data();
        let high_price = &data.get(index).unwrap().high();
        let close_price = &data.get(index).unwrap().close();
        let prev_close_price = &data.get(prev_index).unwrap().close();
        let open_price = data.get(next_index).unwrap().close();
        let ema_8 = instrument.indicators.ema_a.get_data_a().get(index).unwrap();
        let prev_ema_8 = instrument
            .indicators
            .ema_a
            .get_data_a()
            .get(prev_index)
            .unwrap();
        let ema_13 = instrument.indicators.ema_b.get_data_a().get(index).unwrap();
        let ema_21 = instrument.indicators.ema_c.get_data_a().get(index).unwrap();

        let entry_condition = anchor_htf
            && (high_price > ema_8
                && prev_close_price <= prev_ema_8
                && close_price < ema_21
                && ema_8 < ema_13
                && ema_13 < ema_21);

        let pips_margin = 1.;

        let stop_loss_pips = std::env::var("PIPS_STOP_LOSS")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let trigger_price = data[index - 5..index]
            .iter()
            .min_by(|x, y| x.high().partial_cmp(&y.high()).unwrap())
            .map(|x| x.close())
            .unwrap()
            - calc::to_pips(&pips_margin, pricing);

        let buy_price = trigger_price - spread;

        let risk = buy_price + close_price + calc::to_pips(&pips_margin, pricing);

        let target_price = match self.strategy_type().is_long_only() {
            true => buy_price + risk * self.risk_reward_ratio,
            false => buy_price - risk * self.risk_reward_ratio,
        };

        let target_price = buy_price - calc::to_pips(&self.profit_target, pricing);
        if index < 250 {
            //log::info!("22222222 {} {}", index, entry_condition);
        }
        match entry_condition {
            true => Operation::Order(vec![
                OrderType::BuyOrderShort(OrderDirection::Down, *close_price, trigger_price),
                OrderType::SellOrderShort(OrderDirection::Down, *close_price, target_price),
                OrderType::StopLoss(OrderDirection::Up, StopLossType::Pips(stop_loss_pips)),
            ]),

            false => Operation::None,
        }
    }

    fn exit_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        pricing: &Pricing,
    ) -> Operation {
        Operation::None
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
