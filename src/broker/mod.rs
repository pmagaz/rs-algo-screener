pub mod xtb;
use crate::error::Result;
use crate::helpers::websocket::MessageType;
use crate::helpers::date::{DateTime, Local};

use serde_json::Value;
use std::future::Future;

pub type DOHLC = (DateTime<Local>, f64, f64, f64, f64, f64); 
pub type VEC_DOHLC = Vec<DOHLC>;

#[derive(Debug)]
pub struct Response<R> {
  pub msg_type: MessageType,
  //pub message: T,
  pub data: R 
}

#[async_trait::async_trait]
pub trait Broker {
  async fn new(ticker: &str) -> Self;
  async fn listen<F, T>(&mut self, mut callback: F)
  where
    F: Send + FnMut(Response<VEC_DOHLC>) -> T,
    T: Future<Output = Result<()>> + Send + 'static;
  async fn get_prices(&mut self, symbol: &str, period: usize, start: i64) -> Result<()>;
  async fn get_symbols(&mut self) -> Result<()>;
  async fn login(&mut self, username: &str, password: &str) -> Result<()>
  where
    Self: Sized;
}
