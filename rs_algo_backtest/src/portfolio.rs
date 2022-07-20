use crate::strategies::strategy::Strategy;
use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::Instrument;
use rs_algo_shared::models::market::*;
use std::env;

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

        let backtest_market = env::var("BACKTEST_MARKET").unwrap();

        let market = match backtest_market.as_ref() {
            "stock" => Market::Stock,
            "forex" => Market::Forex,
            "crypto" => Market::Crypto,
            _ => Market::Stock,
        };

        let endpoint = env::var("BACKEND_BACKTEST_INSTRUMENTS_ENDPOINT").unwrap();

        let instrument_result_endpoint = env::var("BACKEND_BACKTEST_ENDPOINT").unwrap().clone();
        for strategy in &self.strategies {
            let mut avg_sessions = vec![];
            let mut avg_trades = vec![];
            let mut avg_wining_trades = vec![];
            let mut avg_won_per_trade = vec![];
            let mut avg_lost_per_trade = vec![];
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
            let mut offset = 0;

            for _key in 0..500 / limit {
                let url = [
                    &endpoint,
                    "/",
                    backtest_market.as_ref(),
                    "?offset=",
                    &offset.to_string(),
                    "&limit=",
                    &limit.to_string(),
                ]
                .concat();

                println!(
                    "[BACKTEST] Requesting instruments from {} to {}",
                    offset,
                    offset + limit
                );

                let instruments_to_test: Vec<Instrument> =
                    request(&url, &String::from("all"), HttpMethod::Get)
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                offset += limit;

                for instrument in &instruments_to_test {
                    println!("[BACKTEST] Testing {}... ", instrument.symbol);
                    let backtest_result =
                        strategy.test(instrument, self.equity, self.commission, self.stop_loss);

                    match backtest_result {
                        BackTestResult::BackTestInstrumentResult(mut result) => {
                            result.market = market.to_owned();
                            let _send_instrument_result: BackTestInstrumentResult =
                                request(&instrument_result_endpoint, &result, HttpMethod::Put)
                                    .await
                                    .unwrap()
                                    .json()
                                    .await
                                    .unwrap();

                            println!(
                                "[BACKTEST] Strategy {} tested for {} instruments",
                                &strategy.name(),
                                avg_sessions.len()
                            );

                            avg_sessions.push(result.sessions);
                            avg_trades.push(result.trades);
                            avg_wining_trades.push(result.wining_trades);
                            avg_won_per_trade.push(result.won_per_trade_per);
                            avg_lost_per_trade.push(result.lost_per_trade_per);
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
            }

            println!(
                "[BACKTEST] Calculating {} averages for {} instruments",
                &strategy.name(),
                avg_sessions.len()
            );

            let strategy_result = BackTestStrategyResult {
                strategy: strategy.name().to_owned(),
                strategy_type: strategy.strategy_type().to_owned(),
                market: market.to_owned(),
                date: to_dbtime(Local::now()),
                avg_sessions: average_usize(&avg_sessions),
                avg_trades: average_usize(&avg_trades),
                avg_wining_trades: average_usize(&avg_wining_trades),
                avg_won_per_trade: average_f64(&avg_won_per_trade),
                avg_lost_per_trade: average_f64(&avg_lost_per_trade),
                avg_losing_trades: average_usize(&avg_losing_trades),
                avg_stop_losses: average_usize(&avg_stop_losses),
                avg_gross_profit: average_f64(&avg_gross_profit),
                avg_commissions: average_f64(&avg_commissions),
                avg_net_profit: average_f64(&avg_net_profit),
                avg_net_profit_per: average_f64(&avg_net_profit_per),
                avg_profitable_trades: average_f64(&avg_profitable_trades),
                avg_profit_factor: average_f64(&avg_profit_factor),
                avg_max_runup: average_f64(&avg_max_runup),
                avg_max_drawdown: average_f64(&avg_max_drawdown),
                avg_buy_hold: average_f64(&avg_buy_hold),
                avg_annual_return: average_f64(&avg_annual_return),
            };

            let strategy_result_endpoint = env::var("BACKEND_BACKTEST_STRATEGIES_ENDPOINT")
                .unwrap()
                .clone();

            let _send_strategy_results =
                request(&strategy_result_endpoint, &strategy_result, HttpMethod::Put)
                    .await
                    .unwrap();
        }
    }
}
