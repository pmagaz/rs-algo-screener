use rs_algo_shared::error::Result;
use rs_algo_shared::models::bot::CompactBotData;

use reqwest::Client;
use wasm_bindgen::prelude::*;

pub async fn get_bots(url: &str, data: String) -> Result<Vec<CompactBotData>>
where
{
    log::info!("[CLIENT] Request with {}", data.to_owned());

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = get_query_value)]
    fn get_query_value() -> String;
    #[wasm_bindgen(js_name = get_base_url)]
    fn get_base_url() -> String;
    fn open_modal();

}

pub async fn get_bots2() -> Result<Vec<CompactBotData>>
where
{
    log::info!("[CLIENT] Request with");
    let base_url = get_base_url();
    let bots_url = [base_url.replace("bots/", "").as_str(), "api/bots"].concat();
    let res = Client::builder()
        .build()
        .unwrap()
        .get(bots_url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok(res)
}
