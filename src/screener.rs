use crate::backend::Backend;
use crate::broker::xtb::Operator;
use crate::broker::xtb::Xtb;
use crate::broker::{Broker, Response, VEC_DOHLC};
use crate::error::{Result, RsAlgoError};
use crate::helpers::websocket::MessageType;
use crate::helpers::websocket::{Message, WebSocket};
use crate::instrument::Instrument;
use futures::future;
use std::future::Future;
#[derive(Debug)]
pub struct Screener {
    broker: Xtb,
    backend: Backend,
}

impl Screener
//impl<BK> Screener<BK>
// where
//     BK: Broker,
{
    pub async fn login(&mut self, username: &str, password: &str) -> Result<()> {
        let result = self.broker.login(username, password).await?;
        let msg = self.broker.websocket.read().await.unwrap();
        Ok(result)
    }

    pub async fn load_data<F, T>(
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
        self.broker
            .get_prices(symbol, time_frame, start_date)
            .await?;
        let msg = self.broker.websocket.read().await.unwrap();
        let txt_msg = match msg {
            Message::Text(txt) => txt,
            _ => panic!(),
        };
        let res = self
            .broker
            .handle_response::<VEC_DOHLC>(&txt_msg)
            .await
            .unwrap();

        let mut instrument = Instrument::new().symbol(&res.symbol).build().unwrap();
        instrument.set_data(res.data).unwrap();
        tokio::spawn(callback(instrument));

        //callback(instrument);
        println!("22222222 {:?}", res.symbol.clone());
        Ok(())
    }

    pub async fn new() -> Result<Self> {
        Ok(Self {
            broker: Xtb::new().await,
            backend: Backend::new(),
        })
    }

    pub async fn start<F, T>(&mut self, mut callback: F) -> Result<()>
    where
        F: Send + FnMut(bool) -> T,
        T: Future<Output = Result<()>> + Send + 'static,
    {
        let handler = |res: Response<VEC_DOHLC>| {
            match &res.msg_type {
                MessageType::Login => {
                    println!("[Login] Ok");
                }
                MessageType::GetInstrumentPrice => {
                    println!("[Data Prices] Ok {}", &res.symbol);
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
            //CONTINUE HERE. TO return a handler or something
            //tokio::spawn(callback(&mut self.broker.operator));
            callback(true);
            //tokio::spawn(async move { callback(true) });
            future::ok::<(), RsAlgoError>(())
        };

        self.broker.listen(handler).await;
        Ok(())
    }
}
