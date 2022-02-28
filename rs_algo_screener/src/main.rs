use crate::error::Result;
use error::RsAlgoErrorKind;

use broker::xtb::*;
use helpers::date;
use helpers::date::Local;
use instrument::Instrument;
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
- Calculate % of the pattern
- Add activated chart figures for channels and broadenings
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
    let from_date = env::var("FROM_DATE").unwrap().parse::<i64>().unwrap();
    let sleep_time = &env::var("SLEEP_TIME").unwrap().parse::<u64>().unwrap();

    let sleep = time::Duration::from_millis(*sleep_time);
    let from = (Local::now() - date::Duration::days(from_date)).timestamp();
    let time_frame = TimeFrame::D;

    let mut screener = Screener::<Xtb>::new().await?;
    screener.login(username, password).await?;
    let symbols = screener.get_symbols().await.unwrap().symbols;

    for s in symbols {
        screener
            .get_instrument_data(
                &s.symbol,
                time_frame.value(),
                from,
                |instrument: Instrument| async move {
                    println!("[INSTRUMENT]  {:?}", &instrument.symbol());

                    let endpoint = env::var("BACKEND_INSTRUMENTS_ENDPOINT").unwrap().clone();

                    let res = request::<Instrument>(&endpoint, &instrument)
                        .await
                        .map_err(|_e| RsAlgoErrorKind::RequestError)?;

                    println!(
                        "[Response] stats {:?} from {:?} at {:?}",
                        res.status(),
                        &instrument.symbol(),
                        Local::now()
                    );

                    Ok(())
                },
            )
            .await?;
        thread::sleep(sleep);
    }

    println!("[Finished] at {:?}", Local::now());

    Ok(())
}
