use dotenv::dotenv;
use reqwest::{Client, Error, Response};
use rs_algo_shared::error::{Result, RsAlgoError};
//use rs_algo_shared::models::*;
use serde::{Deserialize, Serialize};
use std::env;
use yew::prelude::*;

pub async fn get_instruments(data: &str) -> Result<Response>
where
    //for<'de> T: Serialize + Deserialize<'de>,
    //for<'de> R: Serialize + Deserialize<'de>,
{
    let url = env::var("BACKEND_INSTRUMENTS_ENDPOINT").unwrap().clone();
    let res = Client::builder()
        .build()
        .unwrap()
        .post(url)
        .body(data.to_owned())
        .send()
        .await
        .unwrap();
    // let res = request(&endpoint, &None, HttpMethod::Get)
    //     .await
    //     .map_err(|_e| RsAlgoError::NotFound)
    //     .unwrap();

    Ok(res)
}

// let query = r#"{"symbol":"patterns.local_patterns": { "$elemMatch" : { "active.active": false} }}"#;
//let json = serde_json::from_str(query).unwrap();
//let res = api::get_instruments(query);
