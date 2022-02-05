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

const TICKER: &str = "ADSK.US_4";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let username = &env::var("BROKER_USERNAME").unwrap();
    let password = &env::var("BROKER_PASSWORD").unwrap();
    let from = (Local::now() - date::Duration::days(365 * 2)).timestamp();

    //TODO configure with builder (credentials, optional render)
    let mut screener = Screener::<Xtb>::new(TICKER).await?;
    screener.login(username, password).await?;
    screener.load_data(TICKER, 1440, from).await?;
    screener.start().await?;
    Ok(())
}
