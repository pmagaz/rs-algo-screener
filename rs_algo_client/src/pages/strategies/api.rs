use rs_algo_shared::error::Result;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::BackTestStrategyResult;

use reqwest::Client;

pub async fn get_strategies(url: &str, data: String) -> Result<Vec<BackTestStrategyResult>>
where
{
    log::info!("[CLIENT] Request with {}", data.to_owned());

    let res = Client::builder()
        .build()
        .unwrap()
        .get(url)
        .body(data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok(res)
}

pub async fn get_instruments_strategies(
    url: &str,
    data: String,
) -> Result<Vec<BackTestInstrumentResult>>
where
{
    log::info!("[CLIENT] Request with {}", data.to_owned());

    let res = Client::builder()
        .build()
        .unwrap()
        .get(url)
        .body(data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok(res)
}
