use crate::strategies::strategy::Strategy;
use rs_algo_shared::error::RsAlgoError;
use rs_algo_shared::helpers::http::{request, HttpMethod};
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::Instrument;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortFolio<S: Strategy> {
    pub order_size: i32,
    pub stop_loss: f64,
    pub commission: f64,
    pub capital: f64,
    pub instruments: Vec<BackTestInstrument>,
    pub strategy: S,
}

impl<S: Strategy> PortFolio<S> {
    pub async fn test(&self, instruments: &Vec<Instrument>) {
        let endpoint = env::var("BACKEND_BACKTEST_ENDPOINT").unwrap().clone();

        for instrument in instruments {
            println!("[BackTest] {:?}", endpoint);
            let backtested_instrument =
                self.strategy
                    .test(instrument, self.commission, self.stop_loss);

            let backtest_result: BackTestResult =
                request(&endpoint, &backtested_instrument, HttpMethod::Put)
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            println!("111 {:?}", backtest_result);
        }
    }
}
