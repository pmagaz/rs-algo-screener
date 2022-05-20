use crate::strategies::strategy::Strategy;
use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::Instrument;
use std::env;
use std::ops::{Add, Div};
pub struct PortFolio {
    pub order_size: i32,
    pub stop_loss: f64,
    pub commission: f64,
    pub equity: f64,
    pub instruments: Vec<Instrument>,
    pub strategies: Vec<Box<dyn Strategy>>,
}

impl PortFolio {
    pub async fn backtest(&self) {
        let limit = env::var("BACkTEST_LIMIT_INSTRUMENTS")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let mut offset = 0;

        let endpoint = env::var("BACKEND_BACKTEST_INSTRUMENTS_ENDPOINT").unwrap();

        for _key in 0..500 / limit {
            let url = [
                &endpoint,
                "?offset=",
                &offset.to_string(),
                "&limit=",
                &limit.to_string(),
            ]
            .concat();

            let instruments_to_test: Vec<Instrument> =
                request(&url, &String::from("all"), HttpMethod::Get)
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            offset += limit;

            let instrument_result_endpoint = env::var("BACKEND_BACKTEST_ENDPOINT").unwrap().clone();
            for strategy in &self.strategies {
                let mut avg_sessions = vec![];
                let mut avg_trades = vec![];
                let mut avg_wining_trades = vec![];
                let mut avg_win_per_trade = vec![];
                let mut avg_losing_trades = vec![];
                let mut avg_stop_losses = vec![];
                let mut avg_gross_profit = vec![];
                let mut avg_commissions = vec![];
                let mut avg_net_profit = vec![];
                let mut avg_net_profit_per = vec![];
                let mut avg_profitable_trades = vec![];
                let mut avg_profit_factor = vec![];
                let mut avg_max_runup = vec![];
                let mut avg_max_drawdown = vec![];
                let mut avg_buy_hold = vec![];
                let mut avg_annual_return = vec![];

                for instrument in &instruments_to_test {
                    let backtest_result =
                        strategy.test(&instrument, self.equity, self.commission, self.stop_loss);

                    match backtest_result {
                        BackTestResult::BackTestInstrumentResult(result) => {
                            let _send_instrument_result: BackTestInstrumentResult =
                                request(&instrument_result_endpoint, &result, HttpMethod::Put)
                                    .await
                                    .unwrap()
                                    .json()
                                    .await
                                    .unwrap();

                            avg_sessions.push(result.sessions);
                            avg_trades.push(result.trades);
                            avg_wining_trades.push(result.wining_trades);
                            avg_win_per_trade.push(result.net_profit_per);
                            avg_losing_trades.push(result.losing_trades);
                            avg_stop_losses.push(result.stop_losses);
                            avg_gross_profit.push(result.gross_profit);
                            avg_commissions.push(result.commissions);
                            avg_net_profit.push(result.net_profit);
                            avg_net_profit_per.push(result.net_profit_per);
                            avg_profitable_trades.push(result.profitable_trades);
                            avg_profit_factor.push(result.profit_factor);
                            avg_max_runup.push(result.max_runup);
                            avg_max_drawdown.push(result.max_drawdown);
                            avg_buy_hold.push(result.buy_hold);
                            avg_annual_return.push(result.annual_return);
                        }
                        _ => (),
                    };
                }

                let strategy_result = BackTestStrategyResult {
                    strategy: strategy.name().to_owned(),
                    avg_sessions: average_usize(avg_sessions),
                    avg_trades: average_usize(avg_trades),
                    avg_wining_trades: average_usize(avg_wining_trades),
                    avg_win_per_trade: average_f64(avg_win_per_trade),
                    avg_losing_trades: average_usize(avg_losing_trades),
                    avg_stop_losses: average_usize(avg_stop_losses),
                    avg_gross_profit: average_f64(avg_gross_profit),
                    avg_commissions: average_f64(avg_commissions),
                    avg_net_profit: average_f64(avg_net_profit),
                    avg_net_profit_per: average_f64(avg_net_profit_per),
                    avg_profitable_trades: average_f64(avg_profitable_trades),
                    avg_profit_factor: average_f64(avg_profit_factor),
                    avg_max_runup: average_f64(avg_max_runup),
                    avg_max_drawdown: average_f64(avg_max_drawdown),
                    avg_buy_hold: average_f64(avg_buy_hold),
                    avg_annual_return: average_f64(avg_annual_return),
                };

                let strategy_result_endpoint = env::var("BACKEND_BACKTEST_STRATEGIES_ENDPOINT")
                    .unwrap()
                    .clone();

                let _send_strategy_results: BackTestStrategyResult =
                    request(&strategy_result_endpoint, &strategy_result, HttpMethod::Put)
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
            }
        }
    }
}