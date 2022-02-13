use crate::error::Result;
use broker::xtb::*;
use broker::Symbol;
use helpers::date;
use helpers::date::Local;
use indicators::Indicator;
use instrument::Instrument;
use screener::Screener;

use dotenv::dotenv;
use std::env;
use std::{thread, time};

mod backend;
mod broker;
mod candle;
mod error;
mod helpers;
mod indicators;
mod instrument;
mod patterns;
mod screener;

/*
TODO LIST
- Add activated chart figures
- Fix horizontal levels
- Calculate divergences on indicators
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
    //         symbol: "TGNA.US_9".to_owned(),
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
        screener
            .get_instrument_data(
                &s.symbol,
                time_frame.value(),
                from,
                |inst: Instrument| async move {
                    let current_price = inst.current_price();
                    println!(
                        "[INSTRUMENT] {:?} [CANDLE] {:?} [PATTERNS] {:?} [INDICATORS] {:?}",
                        inst.symbol(),
                        inst.current_candle().candle_type(),
                        [
                            inst.patterns().extrema_patterns[0].pattern_type.clone(),
                            inst.patterns().local_patterns[0].pattern_type.clone(),
                        ],
                        [
                            inst.indicators().macd().get_status(current_price),
                            inst.indicators().rsi().get_status(current_price),
                            inst.indicators().stoch().get_status(current_price),
                            inst.indicators().ema200().get_status(current_price),
                            inst.indicators().ema100().get_status(current_price),
                            inst.indicators().ema50().get_status(current_price)
                        ]
                    );

                    Ok(())
                },
            )
            .await?;
        thread::sleep(sleep);
    }
    Ok(())
}
