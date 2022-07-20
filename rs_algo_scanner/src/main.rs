use crate::broker::*;
use crate::error::Result;
use error::RsAlgoErrorKind;
use instrument::Instrument;
use rs_algo_shared::broker;
use rs_algo_shared::broker::xtb::*;
use rs_algo_shared::helpers::comp::symbol_in_list;
use rs_algo_shared::helpers::date;
use rs_algo_shared::helpers::date::Local;
use rs_algo_shared::helpers::http::request;
use rs_algo_shared::models::time_frame::TimeFrame;
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
    let from_date = env::var("FROM_DATE").unwrap().parse::<i64>().unwrap();
    let sleep_time = &env::var("SLEEP_TIME").unwrap().parse::<u64>().unwrap();
    let time_frame = &env::var("TIME_FRAME").unwrap();

    let sleep = time::Duration::from_millis(*sleep_time);
    let from = (Local::now() - date::Duration::days(from_date)).timestamp();

    let time_frame = TimeFrame::new(time_frame);

    let mut screener = Screener::<Xtb>::new().await?;
    screener.login(username, password).await?;
    let mut symbols = screener.get_symbols().await.unwrap().symbols;

    let env = env::var("ENV").unwrap();

    let filter = env::var("SYMBOLS_FILTER_LIST").unwrap();

    let backtest_mode = env::var("SCANNER_BACKTEST_MODE")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    let backtest_market = env::var("SCANNER_BACKTEST_MARKET").unwrap().clone();
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
                symbol: "BITCOIN".to_owned(),
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

    for s in symbols {
        let now = Instant::now();

        if !backtest_mode
            || (backtest_mode
                && (symbol_in_list(&s.symbol, &sp500::get_symbols())
                    || symbol_in_list(&s.symbol, &forex::get_symbols())
                    || symbol_in_list(&s.symbol, &crypto::get_symbols())))
        {
            println!("[SCANNER] processing {} ...", &s.symbol);

            screener
                .get_instrument_data(
                    &s.symbol,
                    time_frame.clone(),
                    from,
                    |instrument: Instrument| async move {
                        println!(
                            "[SCANNER] {} processed {} to {} in {:?}",
                            &instrument.symbol(),
                            &instrument.data().first().unwrap().date(),
                            &instrument.date(),
                            now.elapsed(),
                        );

                        let endpoint = env::var("BACKEND_INSTRUMENTS_ENDPOINT").unwrap().clone();

                        let url = match backtest_mode {
                            true => [endpoint, "?mode=backtest".to_string()].concat(),
                            false => [endpoint, "?mode=daily".to_string()].concat(),
                        };

                        let now = Instant::now();
                        let res = request(&url, &instrument, HttpMethod::Put)
                            .await
                            .map_err(|_e| RsAlgoErrorKind::RequestError)?;

                        println!(
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
    println!("[Finished] at {:?}  in {:?}", Local::now(), start.elapsed());

    Ok(())
}
