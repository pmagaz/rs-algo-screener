use broker::xtb::Xtb;
use broker::Broker;
use error::Result;
use helpers::date;
use helpers::date::Local;
use screener::Screener;

use dotenv::dotenv;
use std::env;

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

    let mut screener = Screener::<Xtb>::new().await?;
    screener.login(username, password).await?;
    screener.load_data("AAPL.US_4", 1440, from).await?;
    screener.load_data("NFLX.US_4", 1440, from).await?;
    screener.start().await?;
    Ok(())
}
