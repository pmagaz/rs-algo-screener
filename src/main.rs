use crate::error::{Result, RsAlgoError};
use broker::xtb::Xtb;
use broker::Broker;
use helpers::date;
use helpers::date::Local;
use screener::Screener;

use dotenv::dotenv;
use futures::future;
use std::env;
use std::future::Future;

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

    //let mut screener = Screener::<Xtb>::new().await?;
    let mut screener = Screener::new().await?;
    screener.login(username, password).await?;
    //let leches = screener.to_owned();
    //&screener.load_data("AAPL.US_4", 1440, from).await?;
    let handler = |x: Screener| async {
        //&screener.load_data("AAPL.US_4", 1440, from).await?;
        //println!("1111 {}", x);
        Ok(())
        // future::ok::<(), RsAlgoError>(())
    };
    screener.start(handler).await?;
    println!("1112222222111");
    Ok(())
}
