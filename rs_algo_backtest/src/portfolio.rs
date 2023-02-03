use crate::strategies::strategy::Strategy;

use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::helpers::{date::*, uuid};
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::market::*;
use rs_algo_shared::models::pricing::*;
use rs_algo_shared::models::time_frame::TimeFrame;
use rs_algo_shared::scanner::instrument::Instrument;
use std::env;

#[derive(Clone)]
pub struct PortFolio {
    pub trade_size: f64,
    pub commission: f64,
    pub equity: f64,
    pub instruments: Vec<Instrument>,
    pub strategies: Vec<Box<dyn Strategy>>,
}

impl PortFolio {
    pub async fn backtest(&self, backtest_market: String) {
        let limit = env::var("BACkTEST_LIMIT_INSTRUMENTS")
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let market = match backtest_market.as_ref() {
            "Stock" => Market::Stock,
            "Forex" => Market::Forex,
            "Crypto" => Market::Crypto,
            _ => Market::Stock,
        };

        let endpoint = env::var("BACKEND_BACKTEST_INSTRUMENTS_ENDPOINT").unwrap();
        let instrument_result_endpoint = env::var("BACKEND_BACKTEST_ENDPOINT").unwrap().clone();
        let pricing_endpoint = env::var("BACKEND_BACKTEST_PRICING_ENDPOINT")
            .unwrap()
            .clone();

        //let time_frame = env::var("TIME_FRAME").unwrap().clone();
        // let htf_time_frame = env::var("HIGHER_TIME_FRAME")
        //     .unwrap()
        //     .parse::<String>()
        //     .unwrap();

        log::info!("[BACKTEST] Requesting Pricing");

        let prices: Vec<Pricing> =
            request(&pricing_endpoint, &String::from("all"), HttpMethod::Get)
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

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

            let time_frame = strategy.time_frame().to_string();
            let htf_time_frame = strategy.higher_time_frame();
            for _key in 0..500 / limit {
                let url = [
                    &endpoint,
                    "/markets/",
                    backtest_market.as_ref(),
                    "/",
                    &time_frame,
                    "?offset=",
                    &offset.to_string(),
                    "&limit=",
                    &limit.to_string(),
                ]
                .concat();

                log::info!(
                    "[BACKTEST] Requesting instruments from {} to {}",
                    url,
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
                    log::info!("[BACKTEST] Testing {}... ", instrument.symbol);

                    let pricing = match prices
                        .iter()
                        .position(|pricing| pricing.symbol() == instrument.symbol)
                    {
                        Some(idx) => prices.get(idx).unwrap().clone(),
                        None => {
                            panic!("[PANIC] Pricing not found for {:?}", instrument.symbol);
                        }
                    };

                    let backtest_result = dyn_clone::clone_box(strategy)
                        .test(
                            instrument,
                            &pricing,
                            self.trade_size,
                            self.equity,
                            self.commission,
                        )
                        .await;

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

                            log::info!(
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

            log::info!(
                "[BACKTEST] Calculating {} averages for {} instruments",
                &strategy.name(),
                avg_sessions.len()
            );

            // let htf_time_frame = match strategy.strategy_type().is_multi_timeframe() {
            //     true => &htf_time_frame,
            //     false => None,
            // };

            let htf_str = match htf_time_frame {
                Some(htf) => htf.to_string(),
                None => "".to_string(),
            };

            let seed = [
                &strategy.name().to_string(),
                &strategy.strategy_type().to_string(),
                &[time_frame.clone()].concat(),
                &market.to_string(),
            ];

            let uuid = uuid::generate(seed);

            let strategy_result = BackTestStrategyResult {
                uuid,
                strategy: strategy.name().to_owned(),
                strategy_type: strategy.strategy_type().to_owned(),
                time_frame: TimeFrame::new(&time_frame),
                higher_time_frame: htf_time_frame.to_owned(),
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
