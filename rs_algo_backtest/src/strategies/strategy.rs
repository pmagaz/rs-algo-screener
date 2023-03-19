use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::pricing::Pricing;
use rs_algo_shared::models::strategy::StrategyType;
use rs_algo_shared::models::time_frame::TimeFrameType;
use rs_algo_shared::models::trade::*;
use rs_algo_shared::models::trade::{Position, TradeIn, TradeOut};
use rs_algo_shared::models::{backtest_instrument::*, order};
use rs_algo_shared::models::{order::*, trade};
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
    fn trading_direction(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HTFInstrument,
    ) -> &TradeDirection;
    fn is_long_strategy(&self) -> bool {
        match self.strategy_type() {
            StrategyType::OnlyLong
            | StrategyType::LongShort
            | StrategyType::OnlyLongMTF
            | StrategyType::LongShortMTF => true,
            _ => false,
        }
    }
    fn is_short_strategy(&self) -> bool {
        match self.strategy_type() {
            StrategyType::OnlyShort
            | StrategyType::LongShort
            | StrategyType::OnlyShortMTF
            | StrategyType::LongShortMTF => true,
            _ => false,
        }
    }
    async fn get_htf_instrument(&self, symbol: &str, higher_time_frame: &str) -> HTFInstrument {
        let uppertime_frame = match self.strategy_type() {
            StrategyType::OnlyLongMTF => true,
            StrategyType::LongShortMTF => true,
            StrategyType::OnlyShortMTF => true,
            _ => false,
        };

        if uppertime_frame {
            let endpoint = env::var("BACKEND_BACKTEST_INSTRUMENTS_ENDPOINT").unwrap();

            let url = [&endpoint, "/", symbol, "/", higher_time_frame].concat();

            log::info!(
                "[BACKTEST UPPER TIMEFRAME] {} instrument for {}",
                &higher_time_frame,
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
        pricing: &mut Pricing,
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

        let _overwrite_orders = env::var("OVERWRITE_ORDERS")
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
                let current_candle = instrument.data().get(index).unwrap();
                let current_close = current_candle.close();
                let _current_date = current_candle.date();
                let pricing = pricing.calculate_spread(current_close);
                orders = order::cancel_pending_expired_orders(index, instrument, &mut orders);

                let pending_orders = order::get_pending(&orders);
                let active_orders_result = self.resolve_pending_orders(
                    index,
                    instrument,
                    pricing,
                    trade_size,
                    &pending_orders,
                    &trades_in,
                );

                let trading_direction = &self
                    .trading_direction(index, instrument, htf_instrument)
                    .clone();

                match active_orders_result {
                    PositionResult::MarketInOrder(TradeResult::TradeIn(trade_in), order) => {
                        if !open_positions {
                            order::fulfill_trade_order(index, &trade_in, &order, &mut orders);
                            order::extend_all_pending_orders(&mut orders);
                            trades_in.push(trade_in);
                            open_positions = true;
                        }
                    }
                    PositionResult::MarketOutOrder(TradeResult::TradeOut(trade_out), order) => {
                        if open_positions {
                            order::fulfill_trade_order(index, &trade_out, &order, &mut orders);
                            order::cancel_trade_pending_orders(&trade_out, &mut orders);
                            trades_out.push(trade_out);
                            open_positions = false;
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
                            order::cancel_trade_pending_orders(&trade_out, &mut orders);
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
                        trading_direction,
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
                            if !open_positions {
                                // match overwrite_orders {
                                //     true => {
                                //         order::cancel_pending_expired_orders(
                                //             index,
                                //             instrument,
                                //             &mut orders,
                                //         );
                                //     }
                                //     false => (),
                                // }

                                orders = order::add_pending(orders, new_orders);
                            }
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
        trading_direction: &TradeDirection,
    ) -> PositionResult {
        let pending_orders = order::get_pending(orders);

        let overwrite_orders = env::var("OVERWRITE_ORDERS")
            .unwrap()
            .parse::<bool>()
            .unwrap();

        match trading_direction.is_long() {
            true => match self.is_long_strategy() {
                true => match self.entry_long(index, instrument, htf_instrument, pricing) {
                    Position::MarketIn(order_types) => {
                        let trade_type = TradeType::MarketInLong;
                        let trade_in_result = trade::resolve_trade_in(
                            index,
                            trade_size,
                            instrument,
                            pricing,
                            &trade_type,
                            None,
                        );

                        let prepared_orders = order_types.map(|orders| {
                            order::prepare_orders(index, instrument, pricing, &trade_type, &orders)
                        });

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

                        let prepared_orders = order::prepare_orders(
                            index,
                            instrument,
                            pricing,
                            &trade_type,
                            &order_types,
                        );

                        let new_orders = match overwrite_orders {
                            true => prepared_orders,
                            false => match pending_orders.len().cmp(&0) {
                                std::cmp::Ordering::Equal => prepared_orders,
                                _ => vec![],
                            },
                        };

                        PositionResult::PendingOrder(new_orders)
                    }
                    _ => PositionResult::None,
                },
                false => PositionResult::None,

                _ => PositionResult::None,
            },
            false => match self.is_short_strategy() {
                true => match self.entry_short(index, instrument, htf_instrument, pricing) {
                    Position::MarketIn(order_types) => {
                        let trade_type = TradeType::MarketInShort;

                        let trade_in_result = trade::resolve_trade_in(
                            index,
                            trade_size,
                            instrument,
                            pricing,
                            &trade_type,
                            None,
                        );

                        let prepared_orders = order_types.map(|orders| {
                            order::prepare_orders(index, instrument, pricing, &trade_type, &orders)
                        });

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

                        let prepared_orders = order::prepare_orders(
                            index,
                            instrument,
                            pricing,
                            &trade_type,
                            &order_types,
                        );

                        let new_orders = match overwrite_orders {
                            true => prepared_orders,
                            false => match pending_orders.len().cmp(&0) {
                                std::cmp::Ordering::Equal => prepared_orders,
                                _ => vec![],
                            },
                        };

                        PositionResult::PendingOrder(new_orders)
                    }
                    _ => PositionResult::None,
                },
                false => PositionResult::None,
                _ => PositionResult::None,
            },
            false => PositionResult::None,
        }
    }

    fn resolve_exit_position(
        &mut self,
        index: usize,
        instrument: &Instrument,
        htf_instrument: &HTFInstrument,
        pricing: &Pricing,
        trade_in: &TradeIn,
    ) -> PositionResult {
        match self.is_long_strategy() {
            true => match self.exit_long(index, instrument, htf_instrument, trade_in, pricing) {
                Position::MarketOut(_) => {
                    let trade_type = TradeType::MarketOutLong;
                    let trade_out_result = trade::resolve_trade_out(
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
                    let orders = order::prepare_orders(
                        index,
                        instrument,
                        pricing,
                        &trade_type,
                        &order_types,
                    );
                    PositionResult::PendingOrder(orders)
                }
                _ => PositionResult::None,
            },
            false => match self.exit_short(index, instrument, htf_instrument, pricing) {
                Position::MarketOut(_) => {
                    let trade_type = TradeType::MarketOutShort;
                    let trade_out_result = trade::resolve_trade_out(
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
                    let orders = order::prepare_orders(
                        index,
                        instrument,
                        pricing,
                        &trade_type,
                        &order_types,
                    );
                    PositionResult::PendingOrder(orders)
                }
                _ => PositionResult::None,
            },
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
                let trade_type = order.to_trade_type();
                let trade_in_result = trade::resolve_trade_in(
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
                let trade_type = order.to_trade_type();
                let trade_out_result = match trades_in.last() {
                    Some(trade_in) => trade::resolve_trade_out(
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
        profit > -90.
    }
}

dyn_clone::clone_trait_object!(Strategy);
