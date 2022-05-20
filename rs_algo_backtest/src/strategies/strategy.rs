use crate::helpers::calc::*;
use async_trait::async_trait;
use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::Instrument;

#[async_trait(?Send)]
pub trait Strategy {
    fn new() -> Result<Self>
    where
        Self: Sized;

    fn test(
        &self,
        instrument: &Instrument,
        equity: f64,
        commission: f64,
        stop_loss: f64,
    ) -> BackTestResult {
        let mut trades_in: Vec<TradeIn> = vec![];
        let mut trades_out: Vec<TradeOut> = vec![];
        let mut open_positions = false;
        let data = &instrument.data;
        let len = data.len();
        let start_date = data.first().map(|x| x.date).unwrap();

        println!(
            "[BACKTEST] Starting {:?} backtest for {:?}  from {:}?",
            self.name(),
            &instrument.symbol,
            start_date
        );
        for (index, _candle) in data.iter().enumerate() {
            if index < len - 1 {
                if !open_positions {
                    let trade_in_result = self.market_in_fn(index, instrument, stop_loss);
                    match trade_in_result {
                        TradeResult::TradeIn(trade_in) => {
                            trades_in.push(trade_in);
                            open_positions = true;
                        }
                        _ => (),
                    };
                }

                if open_positions {
                    let trade_in = trades_in.last().unwrap();
                    let trade_out_result = self.market_out_fn(index, instrument, trade_in);
                    match trade_out_result {
                        TradeResult::TradeOut(trade_out) => {
                            trades_out.push(trade_out);
                            open_positions = false;
                        }
                        _ => (),
                    };
                }
            }
        }

        self.backtest_result(instrument, trades_in, trades_out, equity, commission)
    }
    fn name(&self) -> &str;
    fn market_in_fn(&self, index: usize, instrument: &Instrument, stop_loss: f64) -> TradeResult;
    fn market_out_fn(
        &self,
        index: usize,
        instrument: &Instrument,
        trade_in: &TradeIn,
    ) -> TradeResult;
    fn backtest_result(
        &self,
        instrument: &Instrument,
        trades_in: Vec<TradeIn>,
        trades_out: Vec<TradeOut>,
        equity: f64,
        commision: f64,
    ) -> BackTestResult;
}