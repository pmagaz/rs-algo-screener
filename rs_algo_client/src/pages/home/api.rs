use reqwest::Client;
//use rs_algo_shared::error::{Result, RsAlgoError};
use rs_algo_shared::error::Result;
use rs_algo_shared::models::*;
use std::env;

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
