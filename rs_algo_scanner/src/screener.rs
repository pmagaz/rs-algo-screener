use crate::backend::Backend;
use crate::broker::{Broker, Response, VEC_DOHLC};
use crate::error::Result;
use crate::instrument::Instrument;

use rs_algo_shared::models::TimeFrameType;
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
        let result = self.broker.login(username, password).await?;
        Ok(result)
    }

    pub async fn get_symbols(&mut self) -> Result<Response<VEC_DOHLC>> {
        let symbols = self.broker.get_symbols().await?;
        Ok(symbols)
    }

    pub async fn get_instrument_data<F, T>(
        &mut self,
        symbol: &str,
        time_frame: TimeFrameType,
        start_date: i64,
        mut callback: F,
    ) -> Result<()>
    where
        F: Send + FnMut(Instrument) -> T,
        T: Future<Output = Result<()>> + Send + 'static,
    {
        let res = self
            .broker
            .get_instrument_data(symbol, time_frame.value(), start_date)
            .await?;

        let mut instrument = Instrument::new()
            .symbol(&res.symbol)
            .time_frame(time_frame)
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
