use crate::error::{Result, RsAlgoError};
use broker::xtb::Xtb;
use broker::Broker;
use helpers::date;
use helpers::date::Local;
use instrument::Instrument;
use screener::Screener;

use dotenv::dotenv;
use futures::future;
use std::env;
use std::future::Future;
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
- Add degrees to higher_highs increment/decrement
- Calculate divergences on indicators
- Add activated chart figures
*/

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let username = &env::var("BROKER_USERNAME").unwrap();
    let password = &env::var("BROKER_PASSWORD").unwrap();
    let from = (Local::now() - date::Duration::days(365 * 3)).timestamp();

    let mut screener = Screener::new().await?;
    screener.login(username, password).await?;
    let handler = |data: Instrument| async move {
        println!("[INSTRUMENT] {:?}", data.symbol());
        Ok(())
    };
    let symbols = screener.get_symbols().await.unwrap();

    let symbols = vec!["AAPL.US_4", "TTD.US_4", "SQ.US_4"];
    for symbol in symbols {
        screener
            .get_instrument_data(symbol, 1440, from, handler)
            .await?;
    }
    Ok(())
}
