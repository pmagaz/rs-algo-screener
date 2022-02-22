use crate::error::Result;
use error::RsAlgoErrorKind;

use broker::xtb::*;
use broker::Symbol;
use helpers::date;
use helpers::date::Local;
use indicators::{Indicator, IndicatorReq, IndicatorType};
use instrument::{Instrument, InstrumentRes};
use screener::Screener;

mod backend;
mod broker;
mod candle;
mod error;
mod helpers;
mod indicators;
mod instrument;
mod patterns;
mod screener;

use dotenv::dotenv;
use helpers::http::request;
use std::env;
use std::{thread, time};
/*
TODO LIST
- Add activated chart figures for channels and broadenings
- Fix horizontal levels
- Calculate divergences on indicators
- Calculate pattern size
- Review candles formulas
- Add degrees to higher_highs increment/decrement
*/

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let username = &env::var("BROKER_USERNAME").unwrap();
    let password = &env::var("BROKER_PASSWORD").unwrap();
    let from_date = env::var("FROM_DATE").unwrap().parse::<i64>().unwrap();
    let sleep_time = &env::var("SLEEP_TIME").unwrap().parse::<u64>().unwrap();

    let sleep = time::Duration::from_millis(*sleep_time);
    let from = (Local::now() - date::Duration::days(from_date)).timestamp();
    let time_frame = TimeFrame::D;

    let mut screener = Screener::<Xtb>::new().await?;
    screener.login(username, password).await?;
    let symbols = screener.get_symbols().await.unwrap().symbols;

    for s in symbols {
        screener
            .get_instrument_data(
                &s.symbol,
                time_frame.value(),
                from,
                |inst: Instrument| async move {
                    let endpoint = env::var("BACKEND_INSTRUMENTS_ENDPOINT").unwrap().clone();

                    let current_price = inst.current_price();
                    let new_data = InstrumentRes {
                        symbol: inst.symbol().to_owned(),
                        updated: "".to_string(),
                        candle: inst.current_candle().candle_type().clone(),
                        current_price: inst.current_price(),
                        patterns: inst.patterns().clone(),
                        indicators: vec![
                            IndicatorReq::new()
                                .indicator_type(IndicatorType::MacD)
                                .status(inst.indicators().macd().get_status(current_price))
                                .data_a(inst.indicators().macd().get_data_a().clone())
                                .data_b(inst.indicators().macd().get_data_b().clone())
                                .build()
                                .unwrap(),
                            IndicatorReq::new()
                                .indicator_type(IndicatorType::Stoch)
                                .status(inst.indicators().stoch().get_status(current_price))
                                .data_a(inst.indicators().stoch().get_data_a().clone())
                                .data_b(inst.indicators().stoch().get_data_b().clone())
                                .build()
                                .unwrap(),
                            IndicatorReq::new()
                                .indicator_type(IndicatorType::Rsi)
                                .status(inst.indicators().rsi().get_status(current_price))
                                .data_a(inst.indicators().rsi().get_data_a().clone())
                                .data_b(inst.indicators().rsi().get_data_b().clone())
                                .build()
                                .unwrap(),
                            IndicatorReq::new()
                                .indicator_type(IndicatorType::Ema_a)
                                .status(inst.indicators().ema_a().get_status(current_price))
                                .data_a(inst.indicators().ema_a().get_data_a().clone())
                                .data_b(inst.indicators().ema_a().get_data_b().clone())
                                .build()
                                .unwrap(),
                            IndicatorReq::new()
                                .indicator_type(IndicatorType::Ema_b)
                                .status(inst.indicators().ema_b().get_status(current_price))
                                .data_a(inst.indicators().ema_b().get_data_a().clone())
                                .data_b(inst.indicators().ema_b().get_data_b().clone())
                                .build()
                                .unwrap(),
                            IndicatorReq::new()
                                .indicator_type(IndicatorType::Ema_c)
                                .status(inst.indicators().ema_c().get_status(current_price))
                                .data_a(inst.indicators().ema_c().get_data_a().clone())
                                .data_b(inst.indicators().ema_c().get_data_b().clone())
                                .build()
                                .unwrap(),
                            IndicatorReq::new()
                                .indicator_type(IndicatorType::Ema_d)
                                .status(inst.indicators().ema_d().get_status(current_price))
                                .data_a(inst.indicators().ema_d().get_data_a().clone())
                                .data_b(inst.indicators().ema_d().get_data_b().clone())
                                .build()
                                .unwrap(),
                            IndicatorReq::new()
                                .indicator_type(IndicatorType::Ema_e)
                                .status(inst.indicators().ema_e().get_status(current_price))
                                .data_a(inst.indicators().ema_e().get_data_a().clone())
                                .data_b(inst.indicators().ema_e().get_data_b().clone())
                                .build()
                                .unwrap(),
                        ],
                    };

                    let res = request::<InstrumentRes>(&endpoint, new_data)
                        .await
                        .map_err(|_e| RsAlgoErrorKind::RequestError)?;

                    println!("[Response] status {:?}", res.status());

                    Ok(())
                },
            )
            .await?;
        thread::sleep(sleep);
    }
    Ok(())
}
