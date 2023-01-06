use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::stop_loss::{StopLoss, StopLossType};
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
    ) -> Operation;
    fn exit_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> Operation;
    fn entry_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> Operation;
    fn exit_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> Operation;
    fn backtest_result(
        &self,
        instrument: &Instrument,
        trades_in: Vec<TradeIn>,
        trades_out: Vec<TradeOut>,
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
        let mut trades_in: Vec<TradeIn> = vec![];
        let mut trades_out: Vec<TradeOut> = vec![];
        let mut active_orders: Vec<Order> = vec![];

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
                let active_orders_result = self.active_orders_fn(
                    index,
                    instrument,
                    upper_tf_instrument,
                    &mut active_orders,
                );

                match active_orders_result {
                    OperationResult::MarketIn(TradeResult::TradeIn(trade_in), _) => {
                        open_positions = true;
                        trades_in.push(trade_in);
                    }
                    OperationResult::MarketOut(TradeResult::TradeOut(trade_out)) => {
                        trades_out.push(trade_out);
                        open_positions = false;
                    }
                    _ => (),
                };

                if open_positions {
                    let trade_in = trades_in.last().unwrap().to_owned();
                    let trade_out_result = self.market_out_fn(
                        index,
                        instrument,
                        upper_tf_instrument,
                        trade_in,
                        //&mut active_orders,
                    );
                    match trade_out_result {
                        OperationResult::MarketOut(TradeResult::TradeOut(trade_out)) => {
                            trades_out.push(trade_out);
                            open_positions = false;
                        }
                        OperationResult::Order(orders) => {
                            active_orders = [active_orders.clone(), orders].concat();
                        }
                        _ => (),
                    };
                }

                if !open_positions && self.there_are_funds(&trades_out) {
                    let operation_in_result = self.market_in_fn(
                        index,
                        instrument,
                        upper_tf_instrument,
                        order_size,
                        spread,
                        //&mut active_orders,
                    );

                    match operation_in_result {
                        OperationResult::MarketIn(TradeResult::TradeIn(trade_in), orders) => {
                            open_positions = true;
                            trades_in.push(trade_in);
                            match orders {
                                Some(orders) => {
                                    active_orders = [active_orders.clone(), orders].concat();
                                }
                                None => (),
                            }
                        }
                        OperationResult::Order(orders) => {
                            active_orders = [active_orders.clone(), orders].concat();
                        }
                        _ => (),
                    };
                }
            }
        }

        self.backtest_result(instrument, trades_in, trades_out, equity, commission)
    }
    fn market_in_fn(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        order_size: f64,
        spread: f64,
        //mut active_orders: &Vec<Order>,
    ) -> OperationResult {
        match is_long_only(self.strategy_type()) {
            true => match self.entry_long(index, instrument, upper_tf_instrument) {
                Operation::MarketIn(result, orders) => {
                    let trade_in = resolve_trade_in(
                        index,
                        order_size,
                        instrument,
                        TradeType::EntryLong,
                        spread,
                        //self.stop_loss(),
                    );
                    OperationResult::MarketIn(trade_in, orders)
                }
                _ => OperationResult::None,
            },
            false => match self.entry_short(index, instrument, upper_tf_instrument) {
                Operation::MarketIn(result, orders) => {
                    let trade_in = resolve_trade_in(
                        index,
                        order_size,
                        instrument,
                        TradeType::EntryShort,
                        spread,
                        //self.stop_loss(),
                    );
                    OperationResult::MarketIn(trade_in, orders)
                }
                _ => OperationResult::None,
            },
        }

        // let mut entry_type: TradeType = TradeType::None;

        // let trade_result =
        //     resolve_trade_in(index, order_size, instrument, entry_type, spread, stop_loss);
    }

    fn active_orders_fn(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        mut active_orders: &Vec<Order>,
    ) -> OperationResult {
        let leches = resolve_active_orders(index, instrument, upper_tf_instrument, active_orders);
        OperationResult::None
    }

    fn market_out_fn(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        mut trade_in: TradeIn,
        //mut active_orders: &Vec<Order>,
    ) -> OperationResult {
        // let mut exit_type: TradeType = TradeType::None;

        // let stop_loss = self.stop_loss();
        // if stop_loss.stop_type != StopLossType::Atr
        //     && stop_loss.stop_type != StopLossType::Percentage
        // {
        //     trade_in.stop_loss = update_stop_loss_values(
        //         &trade_in.stop_loss,
        //         stop_loss.stop_type.to_owned(),
        //         stop_loss.price,
        //     );
        // }

        // match self.exit_long(index, instrument, upper_tf_instrument) {
        //     Operation::MarketOut(result) => exit_type = TradeType::ExitLong,
        //     _ => (),
        // };

        // match self.exit_short(index, instrument, upper_tf_instrument) {
        //     Operation::MarketOut(result) => exit_type = TradeType::ExitShort,
        //     _ => (),
        // };

        match is_long_only(self.strategy_type()) {
            true => match self.entry_long(index, instrument, upper_tf_instrument) {
                Operation::MarketIn(result, orders) => {
                    let trade_out =
                        resolve_trade_out(index, instrument, trade_in, TradeType::ExitLong);
                    OperationResult::MarketOut(trade_out)
                }
                _ => OperationResult::None,
            },
            false => match self.entry_short(index, instrument, upper_tf_instrument) {
                Operation::MarketIn(result, orders) => {
                    let trade_out =
                        resolve_trade_out(index, instrument, trade_in, TradeType::ExitShort);
                    OperationResult::MarketOut(trade_out)
                }
                _ => OperationResult::None,
            },
        }

        // resolve_trade_out(index, instrument, trade_in, exit_type)
    }
    // fn stop_loss(&self) -> &StopLoss;
    // fn update_stop_loss(&mut self, stop_type: StopLossType, price: f64) -> &StopLoss;
    // fn stop_loss_exit(&mut self, exit_condition: bool, price: f64) -> bool {
    //     // match exit_condition {
    //     //     true => {
    //     //         self.update_stop_loss(price);
    //     //         false
    //     //     }
    //     //     false => {
    //     //         self.update_stop_loss(0.);
    //     //         false
    //     //     }
    //     // }
    //     false
    // }
    // fn stop_loss_exit(&mut self, stop_type: StopLossType, price: f64) -> bool {
    //     let stop_loss = self.stop_loss();
    //     update_stop_loss_values(stop_loss, stop_type, price);
    //     true
    // }

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
