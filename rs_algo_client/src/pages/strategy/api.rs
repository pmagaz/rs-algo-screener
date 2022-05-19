use reqwest::Client;
//use rs_algo_shared::error::{Result, RsAlgoError};
use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::api::ApiResponse;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::*;

pub async fn get_backtest_strategy_instruments(
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
