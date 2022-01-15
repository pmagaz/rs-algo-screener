use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub async fn request<T>(url: &str, data: T) -> Result<Response, Error>
where
  for<'de> T: Serialize + Deserialize<'de> + Debug,
  //for<'de> R: Serialize + Deserialize<'de>,
{
  println!("[Request] to {:?} with {:?} ", url, &data);

  Client::builder()
    .build()?
    .get(url)
    //.basic_auth(gh_user.clone(), Some(gh_pass.clone()))
    //.form(&data)
    //.json(&data)
    .send()
    .await
}
