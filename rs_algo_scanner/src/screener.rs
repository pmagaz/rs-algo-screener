use crate::backend::Backend;
use crate::error::Result;

use rs_algo_shared::broker::{Broker, Response, VEC_DOHLC};
use rs_algo_shared::models::market::*;
use rs_algo_shared::models::time_frame::TimeFrameType;
use rs_algo_shared::scanner::instrument::Instrument;

use std::env;
use std::future::Future;

#[derive(Debug)]
pub struct Screener<BK> {
    broker: BK,
    pub backend: Backend,
}

impl<BK> Screener<BK>
where
    BK: Broker,
{
    pub async fn new() -> Result<Self> {
        Ok(Self {
            broker: BK::new().await,
            backend: Backend::new(),
        })
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<()> {
        let result = self.broker.login(username, password).await.unwrap();
        Ok(())
    }

    pub async fn get_symbols(&mut self) -> Result<Response<VEC_DOHLC>> {
        let symbols = self.broker.get_symbols().await.unwrap();
        Ok(symbols)
    }

    pub async fn get_instrument_data<F, T>(
        &mut self,
        symbol: &str,
        market: &Market,
        time_frame: &TimeFrameType,
        start_date: i64,
        mut callback: F,
    ) -> Result<()>
    where
        F: Send + FnMut(Instrument) -> T,
        T: Future<Output = Result<()>> + Send + 'static,
    {
        let res = self
            .broker
            .get_instrument_data(symbol, time_frame.to_number() as usize, start_date)
            .await
            .unwrap();

        let mut instrument = Instrument::new()
            .symbol(&symbol)
            .market(market.to_owned())
            .time_frame(time_frame.to_owned())
            .build()
            .unwrap();

        instrument.set_data(res.data).unwrap();

        let render_to_image = env::var("RENDER_TO_IMAGE")
            .unwrap()
            .parse::<bool>()
            .unwrap();

        if render_to_image {
            self.backend.render(&instrument).unwrap();
        }

        tokio::spawn(callback(instrument));

        Ok(())
    }
}
