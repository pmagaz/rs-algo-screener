use super::{Broker, Response, Symbol, VEC_DOHLC};
use crate::error::Result;
use crate::helpers::websocket::{Message, MessageType, WebSocket};

use rs_algo_shared::helpers::date::parse_time;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fmt::Debug;
use std::future::Future;

#[derive(Debug)]
pub struct Xtb {
    pub websocket: WebSocket,
    symbol: String,
    sessionId: String,
    time_frame: usize,
    from_date: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Command<T> {
    pub command: String,
    pub arguments: T,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Command2 {
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginParams {
    pub userId: String,
    pub password: String,
    pub appName: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub status: bool,
    pub streamSessionId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickerPriceParams {
    pub command: String,
    pub streamSessionId: String,
    pub symbol: String,
    pub minArrivalTime: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instrument {
    info: InstrumentCandles,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentCandles {
    period: usize,
    start: i64,
    symbol: String,
}

#[async_trait::async_trait]
impl Broker for Xtb {
    async fn new() -> Self {
        let url = &env::var("BROKER_URL").unwrap();

        Self {
            websocket: WebSocket::connect(url).await,
            sessionId: "".to_owned(),
            symbol: "".to_owned(),
            time_frame: 0,
            from_date: 0,
        }
    }

    async fn login(&mut self, username: &str, password: &str) -> Result<()> {
        self.send(&Command {
            command: String::from("login"),
            arguments: LoginParams {
                userId: String::from(username),
                password: String::from(password),
                appName: String::from("rs-algo-screener"),
            },
        })
        .await?;

        let res = self.get_response().await?;

        Ok(())
    }

    async fn get_symbols(&mut self) -> Result<Response<VEC_DOHLC>> {
        self.send(&Command2 {
            command: "getAllSymbols".to_owned(),
        })
        .await?;
        let res = self.get_response().await?;

        Ok(res)
    }

    async fn get_instrument_data(
        &mut self,
        symbol: &str,
        time_frame: usize,
        from_date: i64,
    ) -> Result<Response<VEC_DOHLC>> {
        self.symbol = symbol.to_owned();
        self.send(&Command {
            command: "getChartLastRequest".to_owned(),
            arguments: Instrument {
                info: InstrumentCandles {
                    symbol: symbol.to_owned(),
                    period: time_frame,
                    start: from_date * 1000,
                },
            },
        })
        .await?;

        let res = self.get_response().await?;

        Ok(res)
    }

    async fn listen<F, T>(&mut self, mut callback: F)
    where
        F: Send + FnMut(Response<VEC_DOHLC>) -> T,
        T: Future<Output = Result<()>> + Send + 'static,
    {
        loop {
            let msg = self.websocket.read().await.unwrap();
            let txt_msg = match msg {
                Message::Text(txt) => txt,
                _ => panic!(),
            };
            let response = self.handle_response::<VEC_DOHLC>(&txt_msg).await.unwrap();
            tokio::spawn(callback(response));
        }
    }
}

impl Xtb {
    async fn send<T>(&mut self, command: &T) -> Result<()>
    where
        for<'de> T: Serialize + Deserialize<'de> + Debug,
    {
        self.websocket
            .send(&serde_json::to_string(&command).unwrap())
            .await?;

        Ok(())
    }

    async fn get_response(&mut self) -> Result<Response<VEC_DOHLC>> {
        let msg = self.websocket.read().await.unwrap();
        let txt_msg = match msg {
            Message::Text(txt) => txt,
            _ => panic!(),
        };
        let res = self.handle_response::<VEC_DOHLC>(&txt_msg).await.unwrap();

        Ok(res)
    }

    pub async fn parse_message(&mut self, msg: &str) -> Result<Value> {
        let parsed: Value = serde_json::from_str(&msg).expect("Can't parse to JSON");
        Ok(parsed)
    }

    pub async fn handle_response<'a, T>(&mut self, msg: &str) -> Result<Response<VEC_DOHLC>> {
        let data = self.parse_message(&msg).await.unwrap();
        let response: Response<VEC_DOHLC> = match &data {
            // Login
            _x if matches!(&data["streamSessionId"], Value::String(_x)) => {
                self.sessionId = data["streamSessionId"].to_string();
                Response {
                    msg_type: MessageType::Login,
                    symbol: "".to_owned(),
                    data: vec![],
                    symbols: vec![],
                }
            }
            // GetSymbols
            _x if matches!(&data["returnData"], Value::Array(_x)) => Response::<VEC_DOHLC> {
                msg_type: MessageType::GetInstrumentPrice,
                symbol: self.symbol.to_owned(),
                data: vec![],
                symbols: self.parse_symbols_data(&data).await.unwrap(),
            },
            // GetInstrumentPrice
            _x if matches!(&data["returnData"]["digits"], Value::Number(_x)) => {
                Response::<VEC_DOHLC> {
                    msg_type: MessageType::GetInstrumentPrice,
                    symbol: self.symbol.to_owned(),
                    symbols: vec![],
                    data: self.parse_price_data(&data).await.unwrap(),
                }
            }
            _ => {
                println!("[Error] {:?}", msg);
                Response {
                    msg_type: MessageType::Other,
                    symbol: "".to_owned(),
                    data: vec![],
                    symbols: vec![],
                }
            }
        };

        Ok(response)
    }

    async fn parse_price_data(&mut self, data: &Value) -> Result<VEC_DOHLC> {
        let mut result: VEC_DOHLC = vec![];
        let digits = data["returnData"]["digits"].as_f64().unwrap();
        let x = 10.0_f64;
        let pow = x.powf(digits);
        for obj in data["returnData"]["rateInfos"].as_array().unwrap() {
            //FIXME!!
            let date = parse_time(obj["ctm"].as_i64().unwrap() / 1000);
            let open = obj["open"].as_f64().unwrap() / pow;
            let high = open + obj["high"].as_f64().unwrap() / pow;
            let low = open + obj["low"].as_f64().unwrap() / pow;
            let close = open + obj["close"].as_f64().unwrap() / pow;
            let volume = obj["vol"].as_f64().unwrap();
            result.push((date, open, high, low, close, volume));
        }

        Ok(result)
    }

    async fn parse_symbols_data(&mut self, data: &Value) -> Result<Vec<Symbol>> {
        let mut result: Vec<Symbol> = vec![];
        let symbols = data["returnData"].as_array().unwrap();
        for s in symbols {
            let symbol = match &s["symbol"] {
                Value::String(s) => s.to_string(),
                _ => panic!("Symbol parse error"),
            };
            let currency = match &s["currency"] {
                Value::String(s) => s.to_string(),
                _ => panic!("Currency parse error"),
            };
            let category = match &s["symbol"] {
                Value::String(s) => s.to_string(),
                _ => panic!("Category parse error"),
            };

            let description = match &s["description"] {
                Value::String(s) => s.to_string(),
                _ => panic!("Description parse error"),
            };

            if symbol.contains(".US") {
                result.push(Symbol {
                    symbol,
                    currency,
                    category,
                    description,
                });
            }
        }
        Ok(result)
    }
}
