use crate::broker::Symbol;
use crate::error::Result;
use error::RsAlgoErrorKind;
use std::time::Instant;

use broker::xtb::*;
use instrument::Instrument;
use rs_algo_shared::helpers::date;
use rs_algo_shared::helpers::date::Local;
use rs_algo_shared::models::time_frame::TimeFrame;
use screener::Screener;

mod backend;
mod broker;
mod candle;
mod error;
mod helpers;
mod indicators;
mod instrument;
mod patterns;
mod prices;
mod screener;

use dotenv::dotenv;
use rs_algo_shared::helpers::http::{request, HttpMethod};

use std::env;
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<()> {
    // fn slope(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64) {
    //     let y = y2 - y1;
    //     let x = x2 - x1;
    //     let slope = y / x;
    //     let y_intercept = y1 - slope * x1;
    //     let q = y2 - (slope * x2);

    //     let hypo = hypotenuse(x, y);

    //     let new_x1 = x2 + hypo;
    //     let new_y1 = (slope * new_x1) + q;

    //     println!("foooo {:?}", (new_x1, new_y1));
    //     return (slope, y_intercept);
    // }

    // fn hypotenuse(x: f64, y: f64) -> f64 {
    //     println!("x y {:?} {:?}", x, y);

    //     let num = x.powi(2) + y.powi(2);
    //     num.powf(0.5)
    // }

    //println!("slope {:?}", slope(0., 86.19, 1., 96.25));
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
    let symbols = screener.get_symbols().await.unwrap().symbols;

    // let symbols = [
    //     Symbol {
    //         symbol: "BITCOIN".to_owned(),
    //         category: "".to_owned(),
    //         description: "".to_owned(),
    //         currency: "".to_owned(),
    //     },
    //     // Symbol {
    //     //     symbol: "OKE.US_4".to_owned(),
    //     //     category: "".to_owned(),
    //     //     description: "".to_owned(),
    //     //     currency: "".to_owned(),
    //     // },
    //     // Symbol {
    //     //     symbol: "NET.US".to_owned(),
    //     //     category: "".to_owned(),
    //     //     description: "".to_owned(),
    //     //     currency: "".to_owned(),
    //     // },
    //     // Symbol {
    //     //     symbol: "TSLA.US_4".to_owned(),
    //     //     category: "".to_owned(),
    //     //     description: "".to_owned(),
    //     //     currency: "".to_owned(),
    //     // },
    //     // Symbol {
    //     //     symbol: "OIL".to_owned(),
    //     //     category: "".to_owned(),
    //     //     description: "".to_owned(),
    //     //     currency: "".to_owned(),
    //     // },
    // ];

    let filter = env::var("SYMBOLS_FILTER_LIST").unwrap();

    for s in symbols {
        let now = Instant::now();
        println!("[INSTRUMENT] {:?} processing...", &s.symbol);
        if s.symbol.contains(&filter) {
            screener
                .get_instrument_data(
                    &s.symbol,
                    time_frame.clone(),
                    from,
                    |instrument: Instrument| async move {
                        println!(
                            "[INSTRUMENT] processed in {:?} at {:?}",
                            now.elapsed(),
                            &instrument.date()
                        );

                        let endpoint = env::var("BACKEND_INSTRUMENTS_ENDPOINT").unwrap().clone();
                        let backtest_mode = env::var("SCANNER_BACKTEST_MODE")
                            .unwrap()
                            .parse::<bool>()
                            .unwrap();

                        let url = match backtest_mode {
                            true => [endpoint, "?mode=backtest".to_string()].concat(),
                            false => [endpoint, "?mode=daily".to_string()].concat(),
                        };

                        let now = Instant::now();
                        let res = request(&url, &instrument, HttpMethod::Put)
                            .await
                            .map_err(|_e| RsAlgoErrorKind::RequestError)?;

                        println!(
                            "[RESPONSE] {:?} status {:?} at {:?} in {:?}",
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
