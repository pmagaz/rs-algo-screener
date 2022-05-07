use std::time::Instant;

use rs_algo_shared::error::{Result, RsAlgoError};
use rs_algo_shared::helpers::date::Local;
use rs_algo_shared::models::instrument::Instrument;

use dotenv::dotenv;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use std::env;

mod helpers;
mod portfolio;
mod strategies;
mod trade;

use portfolio::PortFolio;
use strategies::strategy::Strategy;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let start = Instant::now();

    let endpoint = env::var("BACKEND_BACKTEST_INSTRUMENTS_ENDPOINT")
        .unwrap()
        .clone();

    let instruments: Vec<Instrument> = request(&endpoint, &String::from("all"), HttpMethod::Get)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let portfolio = PortFolio {
        order_size: 1,
        commission: 0.015,
        capital: 10000.,
        instruments: vec![],
        strategy: strategies::ema::Ema::new().unwrap(),
    };

    portfolio.test(&instruments).await;

    println!("[Finished] at {:?}  in {:?}", Local::now(), start.elapsed());

    Ok(())
}
