use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::stop_loss::{update_stop_loss_values, StopLoss, StopLossType};
use rs_algo_shared::models::strategy::StrategyType;
use rs_algo_shared::models::trade::*;
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
    ) -> bool;
    fn exit_long(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool;
    fn entry_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool;
    fn exit_short(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool;
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
                if open_positions {
                    let trade_in = trades_in.last().unwrap().to_owned();
                    let trade_out_result =
                        self.market_out_fn(index, instrument, upper_tf_instrument, trade_in);
                    match trade_out_result {
                        TradeResult::TradeOut(trade_out) => {
                            trades_out.push(trade_out);
                            open_positions = false;
                        }
                        _ => (),
                    };
                }

                if !open_positions && self.there_are_funds(&trades_out) {
                    let trade_in_result = self.market_in_fn(
                        index,
                        instrument,
                        upper_tf_instrument,
                        order_size,
                        spread,
                    );

                    match trade_in_result {
                        TradeResult::TradeIn(trade_in) => {
                            trades_in.push(trade_in);
                            open_positions = true;
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
    ) -> TradeResult {
        let entry_type: TradeType;

        if self.entry_long(index, instrument, upper_tf_instrument) {
            entry_type = TradeType::EntryLong
        } else if self.entry_short(index, instrument, upper_tf_instrument) {
            entry_type = TradeType::EntryShort
        } else {
            entry_type = TradeType::None
        }

        let stop_loss = self.stop_loss();

        resolve_trade_in(index, order_size, instrument, entry_type, spread, stop_loss)
    }

    fn market_out_fn(
        &mut self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        mut trade_in: TradeIn,
    ) -> TradeResult {
        let exit_type: TradeType;

        let stop_loss = self.stop_loss();

        if stop_loss.stop_type != StopLossType::Atr
            && stop_loss.stop_type != StopLossType::Percentage
        {
            trade_in.stop_loss = update_stop_loss_values(
                &trade_in.stop_loss,
                stop_loss.stop_type.to_owned(),
                stop_loss.price,
            );
        }

        if self.exit_long(index, instrument, upper_tf_instrument) {
            exit_type = TradeType::ExitLong
        } else if self.exit_short(index, instrument, upper_tf_instrument) {
            exit_type = TradeType::ExitShort
        } else {
            exit_type = TradeType::None
        }
        let stop_loss = true;

        resolve_trade_out(index, instrument, trade_in, exit_type)
    }
    fn stop_loss(&self) -> &StopLoss;
    fn update_stop_loss(&mut self, stop_type: StopLossType, price: f64) -> &StopLoss;
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
    fn stop_loss_exit(&mut self, stop_type: StopLossType, price: f64) -> bool {
        let stop_loss = self.stop_loss();
        update_stop_loss_values(stop_loss, stop_type, price);
        true
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
