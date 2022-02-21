use crate::error::Result;
use error::RsAlgoErrorKind;

use broker::xtb::*;
use broker::Symbol;
use helpers::date;
use helpers::date::Local;
use indicators::Indicator;
use instrument::{Instrument, InstrumentRes};
use screener::Screener;

mod backend;
mod broker;
mod candle;
mod error;
mod helpers;
mod indicators;
mod instrument;
mod patterns;
mod screener;

use dotenv::dotenv;
use helpers::http::request;
use std::env;
use std::{thread, time};
/*
TODO LIST
- Add activated chart figures for channels and broadenings
- Fix horizontal levels
- Calculate divergences on indicators
- Calculate pattern size
- Review candles formulas
- Add degrees to higher_highs increment/decrement
*/

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let username = &env::var("BROKER_USERNAME").unwrap();
    let password = &env::var("BROKER_PASSWORD").unwrap();

    let sleep_time = &env::var("SLEEP_TIME").unwrap().parse::<u64>().unwrap();

    let sleep = time::Duration::from_millis(*sleep_time);
    let from = (Local::now() - date::Duration::days(365 * 2)).timestamp();
    let time_frame = TimeFrame::D;

    let mut screener = Screener::<Xtb>::new().await?;
    screener.login(username, password).await?;
    let symbols = screener.get_symbols().await.unwrap().symbols;

    // let symbols = [
    //     Symbol {
    //         symbol: "FXA.US".to_owned(),
    //         category: "".to_owned(),
    //         description: "".to_owned(),
    //         currency: "".to_owned(),
    //     },
    //     Symbol {
    //         symbol: "BMRN.US_9".to_owned(),
    //         category: "".to_owned(),
    //         description: "".to_owned(),
    //         currency: "".to_owned(),
    //     },
    //     Symbol {
    //         symbol: "SIRI.US_9".to_owned(),
    //         category: "".to_owned(),
    //         description: "".to_owned(),
    //         currency: "".to_owned(),
    //     },
    //     Symbol {
    //         symbol: "GOOGL.US_9".to_owned(),
    //         category: "".to_owned(),
    //         description: "".to_owned(),
    //         currency: "".to_owned(),
    //     },
    // ];

    for s in symbols {
        //println!("[Response122222] {:?} ", s.symbol);

        screener
            .get_instrument_data(
                &s.symbol,
                time_frame.value(),
                from,
                |inst: Instrument| async move {
                    let endpoint = env::var("BACKEND_INSTRUMENTS_ENDPOINT").unwrap().clone();

                    let current_price = inst.current_price();
                    // println!(
                    //     "[INSTRUMENT] {:?} [CANDLE] {:?} [PATTERNS] {:?} [INDICATORS] {:?}",
                    //     inst.symbol(),
                    //     inst.current_candle().candle_type(),
                    //     [
                    //         inst.patterns().extrema_patterns[0].pattern_type.clone(),
                    //         inst.patterns().local_patterns[0].pattern_type.clone(),
                    //     ],
                    //     [
                    //         inst.indicators().macd().get_status(current_price),
                    //         inst.indicators().rsi().get_status(current_price),
                    //         inst.indicators().stoch().get_status(current_price),
                    //         inst.indicators().ema_a().get_status(current_price),
                    //         inst.indicators().ema_b().get_status(current_price),
                    //         inst.indicators().ema_c().get_status(current_price)
                    //     ]
                    // );

                    let new_data = InstrumentRes {
                        symbol: inst.symbol().to_owned(),
                        created: Local::now().to_string(),
                        updated: "".to_string(),
                        candle: inst.current_candle().candle_type().clone(),
                        current_price: inst.current_price(),
                        patterns: inst.patterns().clone(),
                        indicators: vec![
                            inst.indicators().macd().get_status(current_price),
                            inst.indicators().rsi().get_status(current_price),
                            inst.indicators().stoch().get_status(current_price),
                            inst.indicators().ema_a().get_status(current_price),
                            inst.indicators().ema_b().get_status(current_price),
                            inst.indicators().ema_c().get_status(current_price),
                        ],
                    };

                    let res = request::<InstrumentRes>(&endpoint, new_data)
                        .await
                        .map_err(|_e| RsAlgoErrorKind::RequestError)?;

                    println!("[Response] status {:?}", res.status());

                    Ok(())
                },
            )
            .await?;
        thread::sleep(sleep);
    }
    Ok(())
}
