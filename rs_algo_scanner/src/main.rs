use crate::broker::*;
use crate::error::Result;
use chrono::Datelike;
use error::RsAlgoErrorKind;
use instrument::Instrument;
use rs_algo_shared::broker;
use rs_algo_shared::broker::xtb::*;
use rs_algo_shared::helpers::comp::symbol_in_list;
use rs_algo_shared::helpers::date;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::helpers::http::request;
use rs_algo_shared::helpers::symbols::{crypto, forex, sp500};
use rs_algo_shared::models::market::*;
use rs_algo_shared::models::time_frame::*;
use screener::Screener;
use std::time::Instant;

mod backend;
mod candle;
mod error;
mod helpers;
mod indicators;
mod instrument;
mod patterns;
mod prices;
mod screener;

use dotenv::dotenv;
use rs_algo_shared::helpers::http::HttpMethod;

use std::env;
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let start = Instant::now();
    let username = &env::var("BROKER_USERNAME").unwrap();
    let password = &env::var("BROKER_PASSWORD").unwrap();
    let num_test_bars = env::var("NUM_TEST_BARS").unwrap().parse::<i64>().unwrap();
    let sleep_time = &env::var("SLEEP_TIME").unwrap().parse::<u64>().unwrap();
    let time_frame = &env::var("BASE_TIME_FRAME").unwrap();
    //let upper_time_frame = &env::var("UPPER_BASE_TIME_FRAME").unwrap();

    let sleep = time::Duration::from_millis(*sleep_time);
    let time_frame = TimeFrame::new(time_frame);
    //let upper_time_frame = TimeFrame::new(upper_time_frame);

    let base_timeframe_from = match time_frame {
        TimeFrameType::W => {
            let num_weeks = date::Duration::days(num_test_bars).num_weeks();
            Local::now() - date::Duration::weeks(num_weeks + 1)
        }
        TimeFrameType::D => (Local::now() - date::Duration::days(num_test_bars)),
        TimeFrameType::H1 => (Local::now() - date::Duration::hours(num_test_bars)),
        TimeFrameType::M30 => (Local::now() - date::Duration::minutes(num_test_bars)),
        _ => (Local::now() - date::Duration::days(num_test_bars)),
    };

    // let upper_timeframe_from = match upper_time_frame {
    //     TimeFrameType::W => {
    //         let num_weeks = date::Duration::days(num_test_bars).num_weeks();
    //         Local::now() - date::Duration::weeks(num_weeks + 1)
    //     }
    //     _ => (Local::now() - date::Duration::days(num_test_bars)),
    // };

    //let num_weeks = date::Duration::days(from_leches).num_weeks();

    //println!("444444 {:?}", &base_timeframe_from.clone());

    let from = (Local::now() - date::Duration::days(num_test_bars)).timestamp();

    let mut screener = Screener::<Xtb>::new().await?;
    screener.login(username, password).await?;
    let mut symbols = screener.get_symbols().await.unwrap().symbols;

    let env = env::var("ENV").unwrap();

    let filter = env::var("SYMBOLS_FILTER_LIST").unwrap();

    let backtest_mode = env::var("SCANNER_BACKTEST_MODE")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    // symbols = vec![
    //     Symbol {
    //         symbol: "FANG.US_4".to_owned(),
    //         category: "".to_owned(),
    //         description: "".to_owned(),
    //         currency: "".to_owned(),
    //     },
    //     Symbol {
    //         symbol: "TGNA.US_9".to_owned(),
    //         category: "".to_owned(),
    //         description: "".to_owned(),
    //         currency: "".to_owned(),
    //     },
    // ];

    if env == "development" {
        symbols = vec![
            Symbol {
                symbol: "DDOG.US".to_owned(),
                category: "".to_owned(),
                description: "".to_owned(),
                currency: "".to_owned(),
            },
            // Symbol {
            //     symbol: "TGNA.US_9".to_owned(),
            //     category: "".to_owned(),
            //     description: "".to_owned(),
            //     currency: "".to_owned(),
            // },
            // Symbol {
            //     symbol: "ETHEREUM".to_owned(),
            //     category: "".to_owned(),
            //     description: "".to_owned(),
            //     currency: "".to_owned(),
            // },
            // Symbol {
            //     symbol: "CRM.US_4".to_owned(),
            //     category: "".to_owned(),
            //     description: "".to_owned(),
            //     currency: "".to_owned(),
            // },
        ]
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

        if !backtest_mode || (backtest_mode && (is_sp500 || is_forex || is_crypto)) {
            println!("[SCANNER] processing {} ...", &s.symbol);

            screener
                .get_instrument_data(
                    &s.symbol,
                    &market,
                    &time_frame,
                    base_timeframe_from.timestamp(),
                    |instrument: Instrument| async move {
                        let endpoint = env::var("BACKEND_INSTRUMENTS_ENDPOINT").unwrap().clone();
                        let time_frame = &env::var("BASE_TIME_FRAME").unwrap();

                        println!(
                            "[SCANNER] {} scanned {} from {} to {} in {:?}",
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
                                &time_frame,
                            ]
                            .concat(),
                            false => [
                                endpoint.as_ref(),
                                "?mode=daily",
                                "&time_frame=",
                                &time_frame,
                            ]
                            .concat(),
                        };

                        let now = Instant::now();
                        // let res = request(&url, &instrument, HttpMethod::Put)
                        //     .await
                        //     .map_err(|_e| RsAlgoErrorKind::RequestError)?;

                        // println!(
                        //     "[BACKEND RESPONSE] {:?} status {:?} at {:?} in {:?}",
                        //     &instrument.symbol(),
                        //     res.status(),
                        //     Local::now(),
                        //     now.elapsed()
                        // );

                        Ok(())
                    },
                )
                .await?;

            thread::sleep(sleep);
        }
    }
    println!("[Finished] at {:?}  in {:?}", Local::now(), start.elapsed());

    Ok(())
}
