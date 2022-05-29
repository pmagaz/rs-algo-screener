use reqwest::Client;
//use rs_algo_shared::error::{Result, RsAlgoError};
use rs_algo_shared::error::Result;
use rs_algo_shared::error::*;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::api::ApiResponse;
use rs_algo_shared::models::instrument::*;
use rs_algo_shared::models::watch_instrument::*;

pub async fn get_instruments(url: &str, data: String) -> Result<Vec<CompactInstrument>>
where
{
    log::info!("[CLIENT] Request with {}", data.to_owned());

    let res = Client::builder()
        .build()
        .unwrap()
        .post(url)
        .body(data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok(res)
}

pub async fn get_watch_instruments(url: &str) -> Result<Vec<CompactInstrument>>
where
{
    log::info!("[CLIENT] Request get watch instruments");

    let res = Client::builder()
        .build()
        .unwrap()
        .get(url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    Ok(res)
}

pub async fn upsert_watch_instrument(url: &str, data: WatchInstrument) -> Result<ApiResponse>
where
{
    log::info!("[CLIENT] Request with {}", data);

    let res: ApiResponse = request::<WatchInstrument>(&url, &data, HttpMethod::Put)
        .await
        .unwrap()
        .json()
        .await
        .map_err(|_e| RsAlgoError::RequestError)?;

    Ok(res)
}

pub async fn get_portfolio_instruments(url: &str) -> Result<Vec<CompactInstrument>>
where
{
    log::info!("[CLIENT] Request get portfolio instruments");

    let res = Client::builder()
        .build()
        .unwrap()
        .get(url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    Ok(res)
}

pub async fn upsert_portfolio_instrument(url: &str, data: WatchInstrument) -> Result<ApiResponse>
where
{
    log::info!("[CLIENT] Request with {}", data);

    let res: ApiResponse = request::<WatchInstrument>(&url, &data, HttpMethod::Put)
        .await
        .unwrap()
        .json()
        .await
        .map_err(|_e| RsAlgoError::RequestError)?;

    Ok(res)
}
