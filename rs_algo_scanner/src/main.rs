use crate::error::Result;
use error::RsAlgoErrorKind;
use rs_algo_shared::broker::xtb::*;
use rs_algo_shared::broker::*;
use rs_algo_shared::helpers::comp::symbol_in_list;

use rs_algo_shared::helpers::date::*;
use rs_algo_shared::helpers::http::request;
use rs_algo_shared::helpers::symbols::{crypto, forex, sp500};
use rs_algo_shared::models::mode::ExecutionMode;
use rs_algo_shared::models::time_frame::*;
use rs_algo_shared::models::{market::*, mode};
use rs_algo_shared::scanner::instrument::Instrument;
use screener::Screener;
use std::time::Instant;

mod backend;
mod error;
mod helpers;
mod prices;
mod screener;

use dotenv::dotenv;
use rs_algo_shared::helpers::http::HttpMethod;

use std::env;
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let start = Instant::now();
    let env = env::var("ENV").unwrap();
    let username = &env::var("BROKER_USERNAME").unwrap();
    let password = &env::var("BROKER_PASSWORD").unwrap();
    let sleep_time = &env::var("SLEEP_TIME").unwrap().parse::<u64>().unwrap();
    let time_frame_str = &env::var("TIME_FRAME").unwrap();
    let execution_mode = mode::from_str(&env::var("EXECUTION_MODE").unwrap());

    let sleep = time::Duration::from_millis(*sleep_time);
    let time_frame = TimeFrame::new(time_frame_str);

    let num_bars = match execution_mode {
        mode::ExecutionMode::Scanner => env::var("NUM_BARS").unwrap().parse::<i64>().unwrap(),
        _ => env::var("NUM_TEST_BARS").unwrap().parse::<i64>().unwrap(),
    };

    let time_frame_from = TimeFrame::get_starting_bar(num_bars, &time_frame, &execution_mode); // stupid

    log::info!("Preparing Scanner from {}", time_frame_from);
    let mut screener = Screener::<Xtb>::new().await?;
    screener.login(username, password).await?;
    let mut symbols = screener.get_symbols().await.unwrap().symbols;

    let backtest_mode = match execution_mode {
        ExecutionMode::ScannerBackTest => true,
        _ => false,
    };

    if env == "development" {
        symbols = vec![Symbol {
            symbol: "BITCOIN".to_owned(),
            category: "".to_owned(),
            description: "".to_owned(),
            currency: "".to_owned(),
        }]
    };

    let mut market: Market = Market::Stock;
    let mut sp500_symbols = vec![];
    let mut forex_symbols = vec![];
    let mut crypto_symbols = vec![];
    let mut is_sp500: bool = false;
    let mut is_forex: bool = false;
    let mut is_crypto: bool = false;

    if backtest_mode {
        sp500_symbols = sp500::get_symbols();
        forex_symbols = forex::get_symbols();
        crypto_symbols = crypto::get_symbols();
    }

    for s in symbols {
        let now = Instant::now();

        if backtest_mode {
            if symbol_in_list(&s.symbol, &sp500_symbols) {
                is_sp500 = true;
                is_forex = false;
                is_crypto = false;
                market = Market::Stock;
            } else if symbol_in_list(&s.symbol, &forex_symbols) {
                is_forex = true;
                is_sp500 = false;
                is_crypto = false;
                market = Market::Forex;
            } else if symbol_in_list(&s.symbol, &crypto_symbols) {
                is_crypto = true;
                is_sp500 = false;
                is_forex = false;
                market = Market::Crypto;
            } else {
                is_sp500 = false;
                is_forex = false;
                is_crypto = false;
            }
        }

        if !backtest_mode || (backtest_mode && (is_forex || is_crypto)) {
            log::info!("processing {} ...", &s.symbol);

            screener
                .get_instrument_data(
                    &s.symbol,
                    &market,
                    &time_frame,
                    time_frame_from.timestamp(),
                    |instrument: Instrument| async move {
                        let endpoint = env::var("BACKEND_INSTRUMENTS_ENDPOINT").unwrap().clone();
                        let time_frame = &env::var("TIME_FRAME").unwrap();

                        log::info!(
                            "{} scanned {} from {} to {} in {:?}",
                            &instrument.symbol(),
                            &time_frame,
                            &instrument.data().first().unwrap().date(),
                            &instrument.date(),
                            now.elapsed(),
                        );

                        let url = match backtest_mode {
                            true => [
                                endpoint.as_ref(),
                                "?mode=backtest",
                                "&time_frame=",
                                time_frame,
                            ]
                            .concat(),
                            false => [endpoint.as_ref(), "?mode=daily", "&time_frame=", time_frame]
                                .concat(),
                        };

                        let now = Instant::now();
                        let res = request(&url, &instrument, HttpMethod::Put)
                            .await
                            .map_err(|_e| RsAlgoErrorKind::RequestError)?;

                        log::info!(
                            "[BACKEND RESPONSE] {:?} status {:?} at {:?} in {:?}",
                            &instrument.symbol(),
                            res.status(),
                            Local::now(),
                            now.elapsed()
                        );

                        Ok(())
                    },
                )
                .await?;

            thread::sleep(sleep);
        }
    }
    log::info!("[Finished] at {:?}  in {:?}", Local::now(), start.elapsed());

    Ok(())
}
