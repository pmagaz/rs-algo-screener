use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::calc::get_prev_index;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::order::*;
use rs_algo_shared::models::strategy::{is_long_only, StrategyType};
use rs_algo_shared::models::trade::*;
use rs_algo_shared::models::trade::{Operation, TradeIn, TradeOut};
use rs_algo_shared::models::{backtest_instrument::*, order};
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
        trade_size: f64,
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

        for (index, candle) in data.iter().enumerate() {
            if index < len - 1 && index >= 5 {
                let pending_orders = order::get_pending(&orders);
                let active_orders_result = self.resolve_pending_orders(
                    index,
                    instrument,
                    trade_size,
                    spread,
                    &pending_orders,
                    &trades_in,
                );
                match active_orders_result {
                    OperationResult::MarketInOrder(TradeResult::TradeIn(trade_in), order) => {
                        if !open_positions {
                            // let order_position = orders.iter().position(|x| {
                            //     x.status == OrderStatus::Pending && x.order_type == order.order_type
                            // });

                            // match order_position {
                            //     Some(x) => {
                            //         open_positions = true;
                            //         orders
                            //             .get_mut(x)
                            //             .unwrap()
                            //             .fulfill_order(index, candle.date());

                            //         log::info!("LEEEEECHES {} @ {:?}", index, orders.get_mut(x));
                            //         //UPDATE STOP LOSS AND SELL ORDER BASED ON PRICE_IN

                            //         log::info!(
                            //             "TRADE_IN CREATED {} @ {:?}",
                            //             index,
                            //             (trade_in.price_in, trade_in.ask)
                            //         );
                            //         trades_in.push(trade_in);
                            //     }
                            //     None => {}
                            // }

                            // let order_position = orders.iter().filter(|x| {
                            //     x.status == OrderStatus::Pending && x.order_type == order.order_type
                            // });

                            order::fulfill_trade_in_order(index, &trade_in, &order, &mut orders);
                            open_positions = true;
                            trades_in.push(trade_in);
                        }
                    }
                    OperationResult::MarketOutOrder(TradeResult::TradeOut(trade_out), order) => {
                        if open_positions {
                            // let order_position = orders.iter().position(|x| {
                            //     x.status == OrderStatus::Pending && x.order_type == order.order_type
                            // });
                            // //.position(|x| x.id == order.id);

                            // match order_position {
                            //     Some(x) => {
                            //         open_positions = false;
                            //         orders
                            //             .get_mut(x)
                            //             .unwrap()
                            //             .fulfill_order(index, candle.date());
                            //         log::info!("CANCELING MARKETOUT");
                            //         orders = order::cancel_pending_trade_orders(&trade_out, orders);
                            //         trades_out.push(trade_out);
                            //     }
                            //     None => {}
                            // }
                            order::fulfill_order(
                                index,
                                &fom_dbtime(trade_out.date_out),
                                &order,
                                &mut orders,
                            );
                            open_positions = false;
                            trades_out.push(trade_out);
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
                    let exit_result = self.resolve_exit_operation(
                        index,
                        instrument,
                        upper_tf_instrument,
                        &trade_in,
                        &exit_type,
                        spread,
                    );

                    match exit_result {
                        OperationResult::MarketOut(TradeResult::TradeOut(trade_out)) => {
                            open_positions = false;
                            orders = order::cancel_pending_trade_orders(&trade_out, orders);
                            trades_out.push(trade_out.clone());
                        }
                        OperationResult::PendingOrder(new_orders) => {
                            orders = order::add_pending(orders, new_orders);
                        }
                        _ => (),
                    };
                }

                if !open_positions && self.there_are_funds(&trades_out) {
                    let entry_operation_result = self.resolve_entry_operation(
                        index,
                        instrument,
                        upper_tf_instrument,
                        &orders,
                        trade_size,
                        spread,
                    );

                    match entry_operation_result {
                        OperationResult::MarketIn(TradeResult::TradeIn(trade_in), new_orders) => {
                            open_positions = true;
                            trades_in.push(trade_in);
                            log::warn!("eeeeeeeeeeee");

                            match new_orders {
                                Some(new_ords) => orders = order::add_pending(orders, new_ords),
                                None => (),
                            }
                        }
                        OperationResult::PendingOrder(new_orders) => {
                            orders = order::add_pending(orders, new_orders);
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
    fn resolve_entry_operation(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        orders: &Vec<Order>,
        trade_size: f64,
        spread: f64,
    ) -> OperationResult {
        let trade_type = match self.strategy_type().is_long_only() {
            true => TradeType::MarketInLong,
            false => TradeType::MarketInShort,
        };

        let data = &instrument.data;
        let prev_index = get_prev_index(index);
        let current_candle = data.get(index).unwrap();
        // let stop_loss_pending_orders: Vec<&Order> = orders
        //     .iter()
        //     .rev()
        //     .take(5)
        //     .filter(|x| x.status == OrderStatus::Pending && !x.order_type.is_stop())
        //     .collect();

        let mut long_entry: bool = false;
        let mut short_entry: bool = false;

        //IMPROVE
        let entry_long = match self.strategy_type() {
            StrategyType::OnlyLong
            | StrategyType::LongShort
            | StrategyType::OnlyLongMultiTF
            | StrategyType::LongShortMultiTF => {
                match self.entry_long(index, instrument, upper_tf_instrument, spread) {
                    Operation::MarketIn(order_types) => {
                        long_entry = true;
                        short_entry = false;

                        // let short_pending_orders: Vec<&Order> = orders
                        //     .iter()
                        //     .rev()
                        //     .take(5)
                        //     .filter(|x| x.status == OrderStatus::Pending && !x.order_type.is_long())
                        //     .collect();

                        let trade_in_result =
                            resolve_trade_in(index, trade_size, instrument, &trade_type, spread);

                        let mut new_orders = match order_types {
                            Some(orders) => {
                                log::info!("eeeeeeee 11111111 {:?}", orders);
                                Some(prepare_orders(
                                    index,
                                    instrument,
                                    &trade_type,
                                    &orders,
                                    spread,
                                ))
                            }
                            None => None,
                        };

                        OperationResult::MarketIn(trade_in_result, new_orders)
                    }
                    Operation::Order(order_types) => {
                        long_entry = true;
                        short_entry = false;
                        log::info!("eeeeeeee 999999999");
                        let mut new_orders =
                            prepare_orders(index, instrument, &trade_type, &order_types, spread);
                        // let leches: Vec<Order> = orders
                        //     .iter()
                        //     .rev()
                        //     .take(5)
                        //     .filter(|x| x.status == OrderStatus::Pending)
                        //     .map(|x| x.clone())
                        //     .collect();

                        // log::info!("111111111 {}", leches.len());

                        // orders = orders
                        //     .iter_mut()
                        //     .map(|x| {
                        //         if x.status == OrderStatus::Pending
                        //             && match x.order_type {
                        //                 OrderType::BuyOrderShort(_, _, _)
                        //                 | OrderType::SellOrderShort(_, _, _)
                        //                 | OrderType::StopLoss(_, _) => true,
                        //                 _ => false,
                        //             }
                        //         {
                        //             x.cancel_order(to_dbtime(current_candle.date()));
                        //         }
                        //         x.clone()
                        //     })
                        //     .collect();

                        OperationResult::PendingOrder(new_orders)
                    }
                    _ => OperationResult::None,
                }
            }
            _ => OperationResult::None,
        };

        let entry_short = match self.strategy_type() {
            StrategyType::OnlyShort
            | StrategyType::LongShort
            | StrategyType::OnlyShortMultiTF
            | StrategyType::LongShortMultiTF => {
                match self.entry_short(index, instrument, upper_tf_instrument, spread) {
                    Operation::MarketIn(order_types) => {
                        short_entry = true;
                        long_entry = false;

                        // let mut long_pending_orders: Vec<Order> = orders
                        //     .iter()
                        //     .rev()
                        //     .take(5)
                        //     .filter(|x| x.status == OrderStatus::Pending && x.order_type.is_long())
                        //     .map(|x| x.clone())
                        //     .collect();

                        let trade_in_result =
                            resolve_trade_in(index, trade_size, instrument, &trade_type, spread);

                        let mut new_orders = match order_types {
                            Some(order_types) => {
                                log::info!("eeeeeeee 3333333 {:?}", order_types);
                                Some(prepare_orders(
                                    index,
                                    instrument,
                                    &trade_type,
                                    &order_types,
                                    spread,
                                ))
                            }
                            None => None,
                        };

                        // CONTINUE HERE CANCEL PENDING ON CHANGE LONG/SHORT

                        // orders = match &trade_in_result {
                        //     TradeResult::TradeIn(trade_in) => {
                        //         Some(order::cancel_pending_trade_orders_in(&trade_in, long_pending_orders))
                        //     }
                        //     _ => None,
                        // };
                        //order::cancel_pending_trade_orders_in(&trade_in, long_pending_orders);
                        OperationResult::MarketIn(trade_in_result, new_orders)
                    }
                    Operation::Order(order_types) => {
                        short_entry = true;
                        long_entry = false;
                        log::info!("eeeeeeee 444444444 {:?}", order_types);
                        let mut new_orders =
                            prepare_orders(index, instrument, &trade_type, &order_types, spread);

                        // orders = orders
                        //     .iter_mut()
                        //     .map(|x| {
                        //         if x.status == OrderStatus::Pending
                        //             && match x.order_type {
                        //                 OrderType::BuyOrderLong(_, _, _)
                        //                 | OrderType::SellOrderLong(_, _, _)
                        //                 | OrderType::StopLoss(_, _) => true,
                        //                 _ => false,
                        //             }
                        //         {
                        //             x.cancel_order(to_dbtime(current_candle.date()));
                        //         }
                        //         x.clone()
                        //     })
                        //     .collect();

                        OperationResult::PendingOrder(new_orders)
                    }
                    _ => OperationResult::None,
                }
            }
            _ => OperationResult::None,
        };

        if long_entry && !short_entry {
            entry_long
        } else if !long_entry && short_entry {
            entry_short
        } else {
            OperationResult::None
        }
    }

    fn resolve_exit_operation(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        trade_in: &TradeIn,
        exit_type: &TradeType,
        spread: f64,
    ) -> OperationResult {
        let mut long_exit: bool = false;
        let mut short_exit: bool = false;

        let exit_long = match self.strategy_type() {
            StrategyType::OnlyLong
            | StrategyType::LongShort
            | StrategyType::OnlyLongMultiTF
            | StrategyType::LongShortMultiTF => {
                match self.exit_long(index, instrument, upper_tf_instrument, spread) {
                    Operation::MarketOut(_) => {
                        log::info!("eeeeeeee 888888888");
                        long_exit = true;
                        short_exit = false;
                        let trade_out_result =
                            resolve_trade_out(index, instrument, trade_in, exit_type, spread);

                        OperationResult::MarketOut(trade_out_result)
                    }
                    Operation::Order(order_types) => {
                        long_exit = true;
                        short_exit = false;
                        log::info!("eeeeeeee 555555555 {:?}", order_types);
                        let orders =
                            prepare_orders(index, instrument, &exit_type, &order_types, spread);
                        OperationResult::PendingOrder(orders)
                    }
                    _ => OperationResult::None,
                }
            }
            _ => OperationResult::None,
        };

        let exit_short = match self.strategy_type() {
            StrategyType::OnlyShort
            | StrategyType::LongShort
            | StrategyType::OnlyShortMultiTF
            | StrategyType::LongShortMultiTF => {
                match self.exit_short(index, instrument, upper_tf_instrument, spread) {
                    Operation::MarketOut(_) => {
                        short_exit = true;
                        long_exit = false;
                        let trade_out_result =
                            resolve_trade_out(index, instrument, trade_in, exit_type, spread);

                        OperationResult::MarketOut(trade_out_result)
                    }
                    Operation::Order(order_types) => {
                        short_exit = true;
                        long_exit = false;
                        log::info!("eeeeeeee 2222222 {:?}", order_types);
                        let orders =
                            prepare_orders(index, instrument, &exit_type, &order_types, spread);
                        OperationResult::PendingOrder(orders)
                    }
                    _ => OperationResult::None,
                }
            }
            _ => OperationResult::None,
        };

        if long_exit && !short_exit {
            exit_long
        } else if !long_exit && short_exit {
            exit_short
        } else {
            OperationResult::None
        }
    }

    fn resolve_pending_orders(
        &mut self,
        index: usize,
        instrument: &Instrument,
        trade_size: f64,
        spread: f64,
        pending_orders: &Vec<Order>,
        trades_in: &Vec<TradeIn>,
    ) -> OperationResult {
        let trade_type = match self.strategy_type().is_long_only() {
            true => TradeType::OrderInLong,
            false => TradeType::OrderInShort,
        };

        match resolve_active_orders(index, instrument, self.strategy_type(), pending_orders) {
            Operation::MarketInOrder(mut order) => {
                let trade_in_result =
                    resolve_trade_in(index, trade_size, instrument, &trade_type, spread);

                let trade_id = match &trade_in_result {
                    TradeResult::TradeIn(trade_in) => trade_in.id,
                    _ => 0,
                };

                order.set_trade_id(trade_id);
                OperationResult::MarketInOrder(trade_in_result, order)
            }
            Operation::MarketOutOrder(mut order) => {
                let trade_type =
                    match self.strategy_type().is_long_only() {
                        true => match order.order_type {
                            OrderType::BuyOrderLong(_, _, _)
                            | OrderType::BuyOrderShort(_, _, _) => TradeType::MarketInLong,
                            OrderType::SellOrderLong(_, _, _)
                            | OrderType::SellOrderShort(_, _, _)
                            | OrderType::TakeProfitLong(_, _, _)
                            | OrderType::TakeProfitShort(_, _, _) => TradeType::MarketOutLong,
                            OrderType::StopLoss(_, _) => TradeType::StopLoss,
                        },
                        false => match order.order_type {
                            OrderType::BuyOrderLong(_, _, _)
                            | OrderType::BuyOrderShort(_, _, _) => TradeType::MarketInShort,
                            OrderType::SellOrderLong(_, _, _)
                            | OrderType::SellOrderShort(_, _, _)
                            | OrderType::TakeProfitLong(_, _, _)
                            | OrderType::TakeProfitShort(_, _, _) => TradeType::MarketOutShort,
                            OrderType::StopLoss(_, _) => TradeType::StopLoss,
                        },
                    };

                let trade_out_result = match trades_in.last() {
                    Some(trade_in) => {
                        log::info!("RESOLVED {} @@@ {:?}", index, trade_type);
                        resolve_trade_out(index, instrument, trade_in, &trade_type, spread)
                    }
                    None => TradeResult::None,
                };

                let trade_id = match &trade_out_result {
                    TradeResult::TradeOut(trade_out) => trade_out.id,
                    _ => 0,
                };

                order.set_trade_id(trade_id);
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
