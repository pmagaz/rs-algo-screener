use crate::strategies::strategy::Strategy;
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
    pub equity: f64,
    pub instruments: Vec<Instrument>,
    pub strategy: S,
}

impl<S: Strategy> PortFolio<S> {
    pub async fn test(&self, instruments: &Vec<Instrument>) {
        let endpoint = env::var("BACKEND_BACKTEST_ENDPOINT").unwrap().clone();

        for instrument in instruments {
            println!("333333333 {:?}", instrument.data.first().unwrap().date);
            let backtested_instrument =
                self.strategy
                    .test(instrument, self.equity, self.commission, self.stop_loss);

            match backtested_instrument {
                BackTestResult::BackTestInstrumentResult(backtested_instrument) => {
                    let leches: BackTestInstrumentResult =
                        request(&endpoint, &backtested_instrument, HttpMethod::Put)
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    //println!("111 {:?}", leches);
                }
                _ => (),
            };
        }
    }
}
