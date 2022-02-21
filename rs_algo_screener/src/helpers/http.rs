use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub async fn request<T>(url: &str, data: T) -> Result<Response, Error>
where
    for<'de> T: Serialize + Deserialize<'de> + Debug,
{
    Client::builder()
        .build()?
        .post(url)
        .json(&data)
        .send()
        .await
}
