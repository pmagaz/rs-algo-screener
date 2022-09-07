use crate::trade::*;

use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::*;
use std::env;

#[async_trait(?Send)]
pub trait Strategy {
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn name(&self) -> &str;
    fn strategy_type(&self) -> &StrategyType;
    fn entry_long(
        &self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool;
    fn exit_long(
        &self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool;
    fn entry_short(
        &self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
    ) -> bool;
    fn exit_short(
        &self,
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

            let url = [&endpoint, "/", symbol, "/", &uppertimeframe].concat();

            println!(
                "[BACKTEST UPPER TIMEFRAME] {} instrument for {}",
                &uppertimeframe, &symbol
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
        &self,
        instrument: &Instrument,
        order_size: f64,
        equity: f64,
        commission: f64,
        stop_loss: f64,
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

        println!(
            "[BACKTEST] Starting {} backtest for {} from {}",
            self.name(),
            &instrument.symbol,
            start_date
        );

        let uppertimeframe = env::var("UPPER_TIME_FRAME").unwrap();

        let upper_tf_instrument = &self
            .get_upper_tf_instrument(&instrument.symbol, &uppertimeframe)
            .await;

        for (index, _candle) in data.iter().enumerate() {
            if index < len - 1 && index >= 5 {
                if open_positions {
                    let trade_in = trades_in.last().unwrap();
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

                if !open_positions {
                    let trade_in_result = self.market_in_fn(
                        index,
                        instrument,
                        upper_tf_instrument,
                        order_size,
                        stop_loss,
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
        &self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        order_size: f64,
        stop_loss: f64,
    ) -> TradeResult {
        let entry_type: TradeType;

        if self.entry_long(index, instrument, upper_tf_instrument) {
            entry_type = TradeType::EntryLong
        } else if self.entry_short(index, instrument, upper_tf_instrument) {
            entry_type = TradeType::EntryShort
        } else {
            entry_type = TradeType::None
        }

        resolve_trade_in(index, order_size, instrument, entry_type, stop_loss)
    }

    fn market_out_fn(
        &self,
        index: usize,
        instrument: &Instrument,
        upper_tf_instrument: &HigherTMInstrument,
        trade_in: &TradeIn,
    ) -> TradeResult {
        let exit_type: TradeType;

        if self.exit_long(index, instrument, upper_tf_instrument) {
            exit_type = TradeType::ExitLong
        } else if self.exit_short(index, instrument, upper_tf_instrument) {
            exit_type = TradeType::ExitShort
        } else {
            exit_type = TradeType::None
        }
        let stop_loss = true;

        resolve_trade_out(index, instrument, trade_in, exit_type, stop_loss)
    }
}
