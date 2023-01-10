use chrono::Local;
use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::order::*;
use rs_algo_shared::models::strategy::{is_long_only, StrategyType};
use rs_algo_shared::models::trade::*;
use rs_algo_shared::models::trade::{Operation, TradeIn, TradeOut};
use rs_algo_shared::scanner::instrument::*;

use async_trait::async_trait;
use dyn_clone::DynClone;
use std::env;

#[async_trait(?Send)]
pub trait Strategy: DynClone {
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn name(&self) -> &str;
    fn strategy_type(&self) -> &StrategyType;
    fn entry_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        spread: f64,
    ) -> Operation;
    fn exit_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        spread: f64,
    ) -> Operation;
    fn entry_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        spread: f64,
    ) -> Operation;
    fn exit_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        spread: f64,
    ) -> Operation;
    fn backtest_result(
        &self,
        instrument: &Instrument,
        trades_in: Vec<TradeIn>,
        trades_out: Vec<TradeOut>,
        orders: Vec<Order>,
        equity: f64,
        commision: f64,
    ) -> BackTestResult;
    async fn get_upper_tf_instrument(
        &self,
        symbol: &str,
        uppertimeframe: &str,
    ) -> HigherTMInstrument {
        let uppertime_frame = match self.strategy_type() {
            StrategyType::OnlyLongMultiTF => true,
            StrategyType::LongShortMultiTF => true,
            StrategyType::OnlyShortMultiTF => true,
            _ => false,
        };

        if uppertime_frame {
            let endpoint = env::var("BACKEND_BACKTEST_INSTRUMENTS_ENDPOINT").unwrap();

            let url = [&endpoint, "/", symbol, "/", uppertimeframe].concat();

            log::info!(
                "[BACKTEST UPPER TIMEFRAME] {} instrument for {}",
                &uppertimeframe,
                &symbol
            );

            let instrument: Instrument = request(&url, &String::from("all"), HttpMethod::Get)
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            HigherTMInstrument::HigherTMInstrument(instrument)
        } else {
            HigherTMInstrument::None
        }
    }
    async fn test(
        &mut self,
        instrument: &Instrument,
        order_size: f64,
        equity: f64,
        commission: f64,
        spread: f64,
    ) -> BackTestResult {
        let mut orders: Vec<Order> = vec![];
        let mut trades_in: Vec<TradeIn> = vec![];
        let mut trades_out: Vec<TradeOut> = vec![];

        let mut open_positions = false;
        let data = &instrument.data;
        let len = data.len();
        let start_date = match data.first().map(|x| x.date) {
            Some(date) => date.to_string(),
            None => "".to_string(),
        };

        log::info!(
            "[BACKTEST] Starting {} backtest for {} from {} using {} spread",
            self.name(),
            &instrument.symbol,
            start_date,
            spread
        );

        let uppertimeframe = env::var("UPPER_TIME_FRAME").unwrap();

        let upper_tf_instrument = &self
            .get_upper_tf_instrument(&instrument.symbol, &uppertimeframe)
            .await;

        for (index, _candle) in data.iter().enumerate() {
            if index < len - 1 && index >= 5 {
                let active_orders_result = self.resolve_pending_orders(
                    index, instrument, order_size, spread, &orders, &trades_in,
                );

                match active_orders_result {
                    OperationResult::MarketInOrder(TradeResult::TradeIn(trade_in), order) => {
                        let order_position = orders.iter().position(|x| x.id == order.id);
                        match order_position {
                            Some(x) => {
                                log::info!("Order executed {:?}", order);
                                open_positions = true;
                                trades_in.push(trade_in);
                                orders.get_mut(x).unwrap().update_order();
                            }
                            None => {
                                log::warn!("Order not executed {:?}", order);
                            }
                        }
                    }
                    OperationResult::MarketOutOrder(TradeResult::TradeOut(trade_out), order) => {
                        let order_position = orders.iter().position(|x| x.id == order.id);
                        match order_position {
                            Some(x) => {
                                open_positions = false;
                                trades_out.push(trade_out);
                                orders.get_mut(x).unwrap().update_order();
                            }
                            None => {
                                log::warn!("Order not executed {:?}", order);
                            }
                        }
                    }
                    _ => (),
                };

                if open_positions {
                    let trade_in = trades_in.last().unwrap().to_owned();
                    let exit_type = match self.strategy_type().is_long_only() {
                        true => TradeType::MarketOutLong,
                        false => TradeType::MarketOutShort,
                    };
                    let trade_out_result = self.resolve_trades_out(
                        index,
                        instrument,
                        upper_tf_instrument,
                        &trade_in,
                        &exit_type,
                        spread,
                    );

                    match trade_out_result {
                        OperationResult::MarketOut(TradeResult::TradeOut(trade_out)) => {
                            open_positions = false;
                            trades_out.push(trade_out);
                        }
                        OperationResult::PendingOrder(new_orders) => {
                            orders = [orders.clone(), new_orders].concat();
                        }
                        _ => (),
                    };
                }

                if !open_positions && self.there_are_funds(&trades_out) {
                    let operation_in_result = self.resolve_trades_in(
                        index,
                        instrument,
                        upper_tf_instrument,
                        order_size,
                        spread,
                    );

                    match operation_in_result {
                        OperationResult::MarketIn(TradeResult::TradeIn(trade_in), new_orders) => {
                            open_positions = true;
                            trades_in.push(trade_in);
                            match new_orders {
                                Some(ord) => orders = [orders.clone(), ord].concat(),
                                None => (),
                            }
                        }
                        OperationResult::PendingOrder(new_orders) => {
                            orders = [orders.clone(), new_orders].concat();
                        }
                        _ => (),
                    };
                }
            }
        }

        self.backtest_result(
            instrument, trades_in, trades_out, orders, equity, commission,
        )
    }
    fn resolve_trades_in(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        order_size: f64,
        spread: f64,
    ) -> OperationResult {
        let trade_type = match self.strategy_type().is_long_only() {
            true => TradeType::MarketInLong,
            false => TradeType::MarketInShort,
        };

        match self.entry_long(index, instrument, upper_tf_instrument, spread) {
            Operation::MarketIn(order_types) => {
                let trade_in_result =
                    resolve_trade_in(index, order_size, instrument, &trade_type, spread);

                let orders = match order_types {
                    Some(orders) => Some(prepare_orders(index, instrument, &trade_type, &orders)),
                    None => None,
                };
                //UPDATE ORDER INDEXIN

                OperationResult::MarketIn(trade_in_result, orders)
            }
            Operation::Order(order_types) => {
                let orders = prepare_orders(index, instrument, &trade_type, &order_types);

                OperationResult::PendingOrder(orders)
            }
            _ => OperationResult::None,
        }
    }

    fn resolve_trades_out(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        trade_in: &TradeIn,
        exit_type: &TradeType,
        spread: f64,
    ) -> OperationResult {
        match self.exit_long(index, instrument, upper_tf_instrument, spread) {
            Operation::MarketOut(_) => {
                let trade_out_result = resolve_trade_out(index, instrument, trade_in, exit_type);

                OperationResult::MarketOut(trade_out_result)
            }
            Operation::Order(order_types) => {
                let orders = prepare_orders(index, instrument, &exit_type, &order_types);

                OperationResult::PendingOrder(orders)
            }
            _ => OperationResult::None,
        }
    }

    fn resolve_pending_orders(
        &mut self,
        index: usize,
        instrument: &Instrument,
        order_size: f64,
        spread: f64,
        orders: &Vec<Order>,
        trades_in: &Vec<TradeIn>,
    ) -> OperationResult {
        let trade_type = match self.strategy_type().is_long_only() {
            true => TradeType::OrderInLong,
            false => TradeType::OrderInShort,
        };

        match resolve_pending_orders(index, instrument, orders.clone()) {
            Operation::MarketInOrder(mut order) => {
                let trade_in_result =
                    resolve_trade_in(index, order_size, instrument, &trade_type, spread);

                let trade_in_index = match &trade_in_result {
                    TradeResult::TradeIn(trade_in) => trade_in.index_in,
                    _ => 0,
                };

                order.set_trade_id(trade_in_index);
                OperationResult::MarketInOrder(trade_in_result, order)
            }
            Operation::MarketOutOrder(mut order) => {
                let trade_in = trades_in.last().unwrap();

                let trade_type = match self.strategy_type().is_long_only() {
                    true => match order.order_type {
                        OrderType::BuyOrder(_, _) => TradeType::MarketInLong,
                        OrderType::SellOrder(_, _) => TradeType::MarketOutLong,
                        OrderType::TakeProfit(_, _) => TradeType::MarketOutLong,
                        OrderType::StopLoss(_) => TradeType::StopLoss,
                    },
                    false => match order.order_type {
                        OrderType::BuyOrder(_, _) => TradeType::MarketInShort,
                        OrderType::SellOrder(_, _) => TradeType::MarketOutShort,
                        OrderType::TakeProfit(_, _) => TradeType::MarketOutShort,
                        OrderType::StopLoss(_) => TradeType::StopLoss,
                    },
                };

                let trade_out_result = resolve_trade_out(index, instrument, trade_in, &trade_type);
                let trade_out_index = match &trade_out_result {
                    TradeResult::TradeOut(trade_out) => trade_out.index_out,
                    _ => 0,
                };

                order.set_trade_id(trade_out_index);
                //UPDATE ORDER INDEXOUT
                OperationResult::MarketOutOrder(trade_out_result, order)
            }
            _ => OperationResult::None,
        }
    }

    fn there_are_funds(&mut self, trades_out: &Vec<TradeOut>) -> bool {
        let profit: f64 = trades_out.iter().map(|trade| trade.profit_per).sum();
        if profit > -90. {
            true
        } else {
            false
        }
    }
}

dyn_clone::clone_trait_object!(Strategy);
