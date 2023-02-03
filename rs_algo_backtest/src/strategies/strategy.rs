use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::order::*;
use rs_algo_shared::models::pricing::Pricing;
use rs_algo_shared::models::strategy::StrategyType;
use rs_algo_shared::models::time_frame::TimeFrameType;
use rs_algo_shared::models::trade::*;
use rs_algo_shared::models::trade::{Position, TradeIn, TradeOut};
use rs_algo_shared::models::{backtest_instrument::*, order};
use rs_algo_shared::scanner::instrument::*;

use async_trait::async_trait;
use dyn_clone::DynClone;
use std::env;

#[async_trait(?Send)]
pub trait Strategy: DynClone {
    fn new(
        time_frame: Option<&str>,
        higher_time_frame: Option<&str>,
        strategy_type: Option<StrategyType>,
    ) -> Result<Self>
    where
        Self: Sized;
    fn name(&self) -> &str;
    fn strategy_type(&self) -> &StrategyType;
    fn time_frame(&self) -> &TimeFrameType;
    fn higher_time_frame(&self) -> &Option<TimeFrameType>;
    fn entry_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HTFInstrument,
        pricing: &Pricing,
    ) -> Position;
    fn exit_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HTFInstrument,
        trade_in: &TradeIn,
        pricing: &Pricing,
    ) -> Position;
    fn entry_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HTFInstrument,
        pricing: &Pricing,
    ) -> Position;
    fn exit_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HTFInstrument,
        pricing: &Pricing,
    ) -> Position;
    fn backtest_result(
        &self,
        instrument: &Instrument,
        trades_in: Vec<TradeIn>,
        trades_out: Vec<TradeOut>,
        orders: Vec<Order>,
        equity: f64,
        commision: f64,
    ) -> BackTestResult;
    async fn get_htf_instrument(&self, symbol: &str, uppertimeframe: &str) -> HTFInstrument {
        let uppertime_frame = match self.strategy_type() {
            StrategyType::OnlyLongMTF => true,
            StrategyType::LongShortMTF => true,
            StrategyType::OnlyShortMTF => true,
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

            HTFInstrument::HTFInstrument(instrument)
        } else {
            HTFInstrument::None
        }
    }
    async fn test(
        &mut self,
        instrument: &Instrument,
        pricing: &Pricing,
        trade_size: f64,
        equity: f64,
        commission: f64,
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

        let overwrite_orders = env::var("OVERWRITE_ORDERS")
            .unwrap()
            .parse::<bool>()
            .unwrap();

        log::info!(
            "[BACKTEST] Starting {} backtest for {} from {} using {} spread",
            self.name(),
            &instrument.symbol,
            start_date,
            pricing.spread()
        );

        let higher_time_frame = match self.higher_time_frame() {
            Some(htf) => htf.to_string(),
            None => "".to_string(),
        };

        let htf_instrument = &self
            .get_htf_instrument(&instrument.symbol, &higher_time_frame)
            .await;

        for (index, _candle) in data.iter().enumerate() {
            if index < len - 1 && index >= 10 {
                let pending_orders = order::get_pending(&orders);
                let active_orders_result = self.resolve_pending_orders(
                    index,
                    instrument,
                    pricing,
                    trade_size,
                    &pending_orders,
                    &trades_in,
                );
                match active_orders_result {
                    PositionResult::MarketInOrder(TradeResult::TradeIn(trade_in), order) => {
                        if !open_positions {
                            //UPDATING
                            // order::fulfill_order_and_update_pricing(
                            //     index,
                            //     &trade_in,
                            //     pricing,
                            //     &order,
                            //     &mut orders,
                            // );

                            order::fulfill_trade_order(index, &trade_in, &order, &mut orders);
                            open_positions = true;
                            trades_in.push(trade_in);
                        }
                    }
                    PositionResult::MarketOutOrder(TradeResult::TradeOut(trade_out), order) => {
                        if open_positions {
                            order::fulfill_trade_order(index, &trade_out, &order, &mut orders);
                            open_positions = false;
                            orders = order::cancel_trade_pending_orders(&trade_out, orders);
                            trades_out.push(trade_out);
                        }
                    }
                    _ => (),
                };

                if open_positions {
                    let trade_in = trades_in.last().unwrap().to_owned();
                    let exit_result = self.resolve_exit_position(
                        index,
                        instrument,
                        htf_instrument,
                        pricing,
                        &trade_in,
                    );

                    match exit_result {
                        PositionResult::MarketOut(TradeResult::TradeOut(trade_out)) => {
                            open_positions = false;
                            orders = order::cancel_trade_pending_orders(&trade_out, orders);
                            trades_out.push(trade_out.clone());
                        }
                        PositionResult::PendingOrder(new_orders) => {
                            orders = order::add_pending(orders, new_orders);
                        }
                        _ => (),
                    };
                }

                if !open_positions && self.there_are_funds(&trades_out) {
                    let entry_position_result = self.resolve_entry_position(
                        index,
                        instrument,
                        htf_instrument,
                        pricing,
                        &orders,
                        trade_size,
                    );

                    match entry_position_result {
                        PositionResult::MarketIn(TradeResult::TradeIn(trade_in), new_orders) => {
                            open_positions = true;
                            trades_in.push(trade_in);

                            match new_orders {
                                Some(new_ords) => orders = order::add_pending(orders, new_ords),
                                None => (),
                            }
                        }
                        PositionResult::PendingOrder(new_orders) => {
                            match overwrite_orders {
                                true => {
                                    //log::info!("OVERWRITING ORDERS {:?}", orders.len());
                                    orders =
                                        order::cancel_all_pending_orders(index, instrument, orders);
                                }
                                false => (),
                            }

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

    fn resolve_entry_position(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HTFInstrument,
        pricing: &Pricing,
        orders: &Vec<Order>,
        trade_size: f64,
    ) -> PositionResult {
        let mut long_entry: bool = false;
        let mut short_entry: bool = false;
        let pending_orders = order::get_pending(orders);
        let overwrite_orders = env::var("OVERWRITE_ORDERS")
            .unwrap()
            .parse::<bool>()
            .unwrap();

        let entry_long = match self.strategy_type() {
            StrategyType::OnlyLong
            | StrategyType::LongShort
            | StrategyType::OnlyLongMTF
            | StrategyType::LongShortMTF => {
                match self.entry_long(index, instrument, htf_instrument, pricing) {
                    Position::MarketIn(order_types) => {
                        let trade_type = TradeType::MarketInLong;
                        let trade_in_result = resolve_trade_in(
                            index,
                            trade_size,
                            instrument,
                            pricing,
                            &trade_type,
                            None,
                        );

                        let prepared_orders = match order_types {
                            Some(orders) => {
                                long_entry = true;
                                short_entry = false;
                                Some(prepare_orders(
                                    index,
                                    instrument,
                                    pricing,
                                    &trade_type,
                                    &orders,
                                ))
                            }
                            None => None,
                        };

                        let new_orders = match overwrite_orders {
                            true => prepared_orders,
                            false => match pending_orders.len().cmp(&0) {
                                std::cmp::Ordering::Equal => prepared_orders,
                                _ => None,
                            },
                        };

                        PositionResult::MarketIn(trade_in_result, new_orders)
                    }
                    Position::Order(order_types) => {
                        let trade_type = TradeType::OrderInLong;
                        // let new_orders = match pending_orders.len().cmp(&0) {
                        //     std::cmp::Ordering::Equal => {
                        //         long_entry = true;
                        //         short_entry = false;
                        //         prepare_orders(
                        //             index,
                        //             instrument,
                        //             pricing,
                        //             &trade_type,
                        //             &order_types,
                        //         )
                        //     }
                        //     _ => vec![],
                        // };

                        let prepared_orders =
                            prepare_orders(index, instrument, pricing, &trade_type, &order_types);

                        let new_orders = match overwrite_orders {
                            true => prepared_orders,
                            false => match pending_orders.len().cmp(&0) {
                                std::cmp::Ordering::Equal => prepared_orders,
                                _ => vec![],
                            },
                        };

                        if new_orders.len() > 0 {
                            long_entry = true;
                            short_entry = false;
                        }
                        //log::info!("111111111 {:?}", (orders.len(), new_orders.len()));

                        PositionResult::PendingOrder(new_orders)
                    }
                    _ => PositionResult::None,
                }
            }
            _ => PositionResult::None,
        };

        let entry_short = match self.strategy_type() {
            StrategyType::OnlyShort
            | StrategyType::LongShort
            | StrategyType::OnlyShortMTF
            | StrategyType::LongShortMTF => {
                match self.entry_short(index, instrument, htf_instrument, pricing) {
                    Position::MarketIn(order_types) => {
                        let trade_type = TradeType::MarketInShort;

                        let trade_in_result = resolve_trade_in(
                            index,
                            trade_size,
                            instrument,
                            pricing,
                            &trade_type,
                            None,
                        );

                        let prepared_orders = match order_types {
                            Some(orders) => {
                                long_entry = true;
                                short_entry = false;
                                Some(prepare_orders(
                                    index,
                                    instrument,
                                    pricing,
                                    &trade_type,
                                    &orders,
                                ))
                            }
                            None => None,
                        };

                        let new_orders = match overwrite_orders {
                            true => prepared_orders,
                            false => match pending_orders.len().cmp(&0) {
                                std::cmp::Ordering::Equal => prepared_orders,
                                _ => None,
                            },
                        };

                        PositionResult::MarketIn(trade_in_result, new_orders)
                    }
                    Position::Order(order_types) => {
                        let trade_type = TradeType::OrderInShort;
                        // let new_orders = match pending_orders.len().cmp(&0) {
                        //     std::cmp::Ordering::Equal => {
                        //         short_entry = true;
                        //         long_entry = false;
                        //         prepare_orders(
                        //             index,
                        //             instrument,
                        //             pricing,
                        //             &trade_type,
                        //             &order_types,
                        //         )
                        //     }
                        //     _ => vec![],
                        // };

                        let prepared_orders =
                            prepare_orders(index, instrument, pricing, &trade_type, &order_types);

                        let new_orders = match overwrite_orders {
                            true => prepared_orders,
                            false => match pending_orders.len().cmp(&0) {
                                std::cmp::Ordering::Equal => prepared_orders,
                                _ => vec![],
                            },
                        };

                        if new_orders.len() > 0 {
                            short_entry = true;
                            long_entry = false;
                        }

                        //log::info!("222222222 {:?}", (orders.len(), new_orders.len()));

                        PositionResult::PendingOrder(new_orders)
                    }
                    _ => PositionResult::None,
                }
            }
            _ => PositionResult::None,
        };

        if long_entry && !short_entry {
            entry_long
        } else if !long_entry && short_entry {
            entry_short
        } else {
            PositionResult::None
        }
    }

    fn resolve_exit_position(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HTFInstrument,
        pricing: &Pricing,
        trade_in: &TradeIn,
        //exit_type: &TradeType,
    ) -> PositionResult {
        let mut long_exit: bool = false;
        let mut short_exit: bool = false;

        let exit_long = match self.strategy_type() {
            StrategyType::OnlyLong
            | StrategyType::LongShort
            | StrategyType::OnlyLongMTF
            | StrategyType::LongShortMTF => {
                match self.exit_long(index, instrument, htf_instrument, trade_in, pricing) {
                    Position::MarketOut(_) => {
                        let trade_type = TradeType::MarketOutLong;
                        long_exit = true;
                        short_exit = false;
                        let trade_out_result = resolve_trade_out(
                            index,
                            instrument,
                            pricing,
                            trade_in,
                            &trade_type,
                            None,
                        );

                        PositionResult::MarketOut(trade_out_result)
                    }
                    Position::Order(order_types) => {
                        let trade_type = TradeType::OrderOutLong;
                        long_exit = true;
                        short_exit = false;
                        let orders =
                            prepare_orders(index, instrument, pricing, &trade_type, &order_types);
                        PositionResult::PendingOrder(orders)
                    }
                    _ => PositionResult::None,
                }
            }
            _ => PositionResult::None,
        };

        let exit_short = match self.strategy_type() {
            StrategyType::OnlyShort
            | StrategyType::LongShort
            | StrategyType::OnlyShortMTF
            | StrategyType::LongShortMTF => {
                match self.exit_short(index, instrument, htf_instrument, pricing) {
                    Position::MarketOut(_) => {
                        let trade_type = TradeType::MarketOutShort;
                        short_exit = true;
                        long_exit = false;
                        let trade_out_result = resolve_trade_out(
                            index,
                            instrument,
                            pricing,
                            trade_in,
                            &trade_type,
                            None,
                        );

                        PositionResult::MarketOut(trade_out_result)
                    }
                    Position::Order(order_types) => {
                        let trade_type = TradeType::OrderOutShort;
                        short_exit = true;
                        long_exit = false;
                        let orders =
                            prepare_orders(index, instrument, pricing, &trade_type, &order_types);
                        PositionResult::PendingOrder(orders)
                    }
                    _ => PositionResult::None,
                }
            }
            _ => PositionResult::None,
        };

        if long_exit && !short_exit {
            exit_long
        } else if !long_exit && short_exit {
            exit_short
        } else {
            PositionResult::None
        }
    }

    fn resolve_pending_orders(
        &mut self,
        index: usize,
        instrument: &Instrument,
        pricing: &Pricing,
        trade_size: f64,
        pending_orders: &Vec<Order>,
        trades_in: &Vec<TradeIn>,
    ) -> PositionResult {
        match resolve_active_orders(index, instrument, pending_orders, pricing) {
            Position::MarketInOrder(mut order) => {
                let trade_type = match order.order_type.is_long() {
                    true => TradeType::OrderInLong,
                    false => TradeType::OrderInShort,
                };
                let trade_in_result = resolve_trade_in(
                    index,
                    trade_size,
                    instrument,
                    pricing,
                    &trade_type,
                    Some(&order),
                );

                let trade_id = match &trade_in_result {
                    TradeResult::TradeIn(trade_in) => trade_in.id,
                    _ => 0,
                };

                order.set_trade_id(trade_id);
                PositionResult::MarketInOrder(trade_in_result, order)
            }
            Position::MarketOutOrder(order) => {
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
                    Some(trade_in) => resolve_trade_out(
                        index,
                        instrument,
                        pricing,
                        trade_in,
                        &trade_type,
                        Some(&order),
                    ),
                    None => TradeResult::None,
                };

                PositionResult::MarketOutOrder(trade_out_result, order)
            }
            _ => PositionResult::None,
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
