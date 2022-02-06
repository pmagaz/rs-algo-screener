use crate::backend::Backend;
use crate::broker::{Broker, Response, VEC_DOHLC};
use crate::error::{Result, RsAlgoError};
use crate::helpers::websocket::MessageType;
use crate::instrument::Instrument;

use futures::future;

pub struct Screener<BK> {
    broker: BK,
    // instrument: Instrument,
    backend: Backend,
}

impl<BK> Screener<BK>
where
    BK: Broker,
{
    pub async fn login(&mut self, username: &str, password: &str) -> Result<()> {
        Ok(self.broker.login(username, password).await?)
    }

    pub async fn load_data(
        &mut self,
        symbol: &str,
        time_frame: usize,
        start_date: i64,
    ) -> Result<()> {
        self.broker
            .get_prices(symbol, time_frame, start_date)
            .await?;
        Ok(())
    }

    pub async fn new() -> Result<Self> {
        Ok(Self {
            broker: BK::new().await,
            //instrument: Instrument::new().symbol(symbol).build().unwrap(),
            backend: Backend::new(),
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        let handler = |res: Response<VEC_DOHLC>| {
            match &res.msg_type {
                MessageType::Login => {
                    println!("[Login] Ok");
                }
                MessageType::GetInstrumentPrice => {
                    println!("[Data Prices] Ok");
                    //res.data.reverse();
                    let mut instrument = Instrument::new().symbol(&res.symbol).build().unwrap();
                    instrument.set_data(res.data).unwrap();
                    println!("[Backend] rendering...");

                    self.backend.render(&instrument).unwrap();
                }
                MessageType::Other => {
                    println!("33333");
                }
            };

            future::ok::<(), RsAlgoError>(())
        };

        self.broker.listen(handler).await;
        Ok(())
    }
}
