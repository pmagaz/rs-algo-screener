use crate::error::Result;
use broker::xtb::*;
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
TODO
- Fix relative local minima
- Fix stoch
- Fix candle calculation
- Add gap identification
- Add EMAS to indicators
- Fix horizontal levels
- Add degrees to higher_highs increment/decrement
- Calculate divergences on indicators
- Add activated chart figures
*/

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let username = &env::var("BROKER_USERNAME").unwrap();
    let password = &env::var("BROKER_PASSWORD").unwrap();
    let sleep_time = &env::var("SLEEP_TIME").unwrap().parse::<u64>().unwrap();

    let sleep = time::Duration::from_millis(*sleep_time);
    let from = (Local::now() - date::Duration::days(365 * 3)).timestamp();
    let time_frame = TimeFrame::D;

    let mut screener = Screener::<Xtb>::new().await?;
    screener.login(username, password).await?;
    let symbols = screener.get_symbols().await.unwrap();
    for s in symbols.symbols {
        screener
            .get_instrument_data(
                &s.symbol,
                time_frame.value(),
                from,
                |inst: Instrument| async move {
                    println!(
                        "[INSTRUMENT] {:?} {:?} {:?} {:?}",
                        inst.symbol(),
                        inst.current_candle().candle_type(),
                        inst.patterns().patterns[0].pattern_type,
                        (
                            inst.indicators().macd().get_status(),
                            inst.indicators().rsi().get_status(),
                            inst.indicators().stoch().get_status()
                        )
                    );
                    Ok(())
                },
            )
            .await?;
        thread::sleep(sleep);
    }
    Ok(())
}
