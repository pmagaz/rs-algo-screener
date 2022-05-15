use reqwest::Client;
//use rs_algo_shared::error::{Result, RsAlgoError};
use rs_algo_shared::error::Result;
use rs_algo_shared::error::*;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::api::ApiResponse;
use rs_algo_shared::models::backtest_strategy::BackTestStrategyResult;
use rs_algo_shared::models::instrument::*;
use rs_algo_shared::models::watch_instrument::*;

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
