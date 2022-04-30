use std::time::Instant;

use rs_algo_shared::error::{Result, RsAlgoError};
use rs_algo_shared::helpers::date;
use rs_algo_shared::helpers::date::Local;
use rs_algo_shared::models::Instrument;

use dotenv::dotenv;
use rs_algo_shared::helpers::http::{request, HttpMethod};

use std::env;
use std::{thread, time};

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

    println!("[RES] {:?}", instruments);
    println!("[Finished] at {:?}  in {:?}", Local::now(), start.elapsed());

    Ok(())
}
