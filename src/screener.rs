use crate::backend::Backend;
use crate::broker::{Broker, Response, VEC_DOHLC};
use crate::error::Result;
use crate::instrument::Instrument;

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
        time_frame: usize,
        start_date: i64,
        mut callback: F,
    ) -> Result<()>
    where
        F: Send + FnMut(Instrument) -> T,
        T: Future<Output = Result<()>> + Send + 'static,
    {
        let res = self
            .broker
            .get_instrument_data(symbol, time_frame, start_date)
            .await?;

        let mut instrument = Instrument::new().symbol(&res.symbol).build().unwrap();
        instrument.set_data(res.data).unwrap();
        //self.backend.render(&instrument).unwrap();
        tokio::spawn(callback(instrument));
        Ok(())
    }

    //     pub async fn listen(&mut self) -> Result<()>
    // // where
    //     //     F: Send + FnMut(bool) -> T,
    //     //     T: Future<Output = Result<()>> + Send + 'static,
    //     {
    //         let handler = |res: Response<VEC_DOHLC>| {
    //             match &res.msg_type {
    //                 MessageType::Login => {
    //                     println!("[Login] Ok");
    //                 }
    //                 MessageType::GetInstrumentPrice => {
    //                     println!("[Data Prices] Ok {}", &res.symbol);
    //                     //res.data.reverse();
    //                     let mut instrument = Instrument::new().symbol(&res.symbol).build().unwrap();
    //                     instrument.set_data(res.data).unwrap();
    //                     println!("[Backend] rendering...");

    //                     self.backend.render(&instrument).unwrap();
    //                 }
    //                 MessageType::Other => {
    //                     println!("33333");
    //                 }
    //             };
    //             //CONTINUE HERE. TO return a handler or something
    //             //tokio::spawn(callback(&mut self.broker.operator));
    //             //  callback(true);
    //             //tokio::spawn(async move { callback(true) });
    //             future::ok::<(), RsAlgoError>(())
    //         };

    //         self.broker.listen(handler).await;
    //         Ok(())
    //     }
}
