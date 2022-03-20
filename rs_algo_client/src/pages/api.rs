use dotenv::dotenv;
use reqwest::{Client, Error, Response};
//use rs_algo_shared::error::{Result, RsAlgoError};
//use rs_algo_shared::models::*;
use rs_algo_shared::error::{Result, RsAlgoError};
use rs_algo_shared::helpers::http::{request, HttpMethod};
use serde::{Deserialize, Serialize};
use std::env;
use yew::prelude::*;

pub async fn get_instruments(data: String) -> Result<Response>
where
    //for<'de> T: Serialize + Deserialize<'de>,
    //for<'de> R: Serialize + Deserialize<'de>,
{
    let url = "http://localhost:8000/api/instruments";
    log::info!("[CLIENT] Request with {}", data.to_owned());

    let res = Client::builder()
        .build()
        .unwrap()
        .post(url)
        .body(data)
        .send()
        .await
        .unwrap();
    Ok(res)
}
