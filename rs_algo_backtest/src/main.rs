mod helpers;
mod portfolio;
mod strategies;

use portfolio::PortFolio;
use rs_algo_shared::helpers::date::Local;
use rs_algo_shared::models::strategy::StrategyType;
use strategies::strategy::Strategy;

use dotenv::dotenv;
use std::env;
use std::time::Instant;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let start = Instant::now();
    let env = env::var("ENV").unwrap();
    let trade_size = env::var("ORDER_SIZE").unwrap().parse::<f64>().unwrap();
    let stop_loss = env::var("ATR_STOP_LOSS").unwrap().parse::<f64>().unwrap();
    let commission = env::var("COMISSION").unwrap().parse::<f64>().unwrap();
    let equity = env::var("EQUITY").unwrap().parse::<f64>().unwrap();

    let portfolio = PortFolio {
        trade_size,
        commission,
        equity,
        instruments: vec![],
        strategies: vec![
            /* Scalping */
            Box::new(
                strategies::ema_scalping2::EmaScalping2::new(
                    Some("M5"),
                    Some("H1"),
                    Some(StrategyType::LongShortMTF),
                )
                .unwrap(),
            ),
            Box::new(
                strategies::ema_scalping2::EmaScalping2::new(
                    Some("M15"),
                    Some("H4"),
                    Some(StrategyType::LongShortMTF),
                )
                .unwrap(),
            ),
            Box::new(
                strategies::ema_scalping2::EmaScalping2::new(
                    Some("M30"),
                    Some("H4"),
                    Some(StrategyType::LongShortMTF),
                )
                .unwrap(),
            ),
            Box::new(
                strategies::ema_scalping::EmaScalping::new(
                    Some("M5"),
                    Some("H1"),
                    Some(StrategyType::LongShortMTF),
                )
                .unwrap(),
            ),
            Box::new(
                strategies::ema_scalping::EmaScalping::new(
                    Some("M15"),
                    Some("H4"),
                    Some(StrategyType::LongShortMTF),
                )
                .unwrap(),
            ),
            Box::new(
                strategies::ema_scalping::EmaScalping::new(
                    Some("M30"),
                    Some("H4"),
                    Some(StrategyType::LongShortMTF),
                )
                .unwrap(),
            ),
            // Box::new(
            //     strategies::bollinger_bands_reversals::BollingerBandsReversals::new(
            //         Some("M5"),
            //         Some("H1"),
            //         Some(StrategyType::OnlyLongMTF),
            //     )
            //     .unwrap(),
            // ),
            // Box::new(
            //     strategies::bollinger_bands_reversals::BollingerBandsReversals::new(
            //         Some("M15"),
            //         Some("H4"),
            //         Some(StrategyType::OnlyLongMTF),
            //     )
            //     .unwrap(),
            // ),
            // Box::new(
            //     strategies::bollinger_bands_reversals::BollingerBandsReversals::new(
            //         Some("M30"),
            //         Some("H4"),
            //         Some(StrategyType::OnlyLongMTF),
            //     )
            //     .unwrap(),
            // ),
            // Box::new(
            //     strategies::bollinger_bands_reversals::BollingerBandsReversals::new(
            //         Some("M15"),
            //         Some("H4"),
            //         Some(StrategyType::LongShortMTF),
            //     )
            //     .unwrap(),
            // ),
            // Box::new(
            //     strategies::bollinger_bands_reversals::BollingerBandsReversals::new(
            //         Some("M30"),
            //         Some("H4"),
            //         Some(StrategyType::LongShortMTF),
            //     )
            //     .unwrap(),
            // ),
            /* MultiTimeFrame */
        ],
    };

    let backtest_market = env::var("BACKTEST_MARKET").unwrap();
    if env == "development" {
        let backtest_markets = vec!["Forex", "Crypto", "Stock"];
        for market in backtest_markets.iter() {
            portfolio.backtest(market.to_string()).await;
        }
    } else {
        portfolio.backtest(backtest_market).await;
    }

    log::info!("[Finished] at {:?}  in {:?}", Local::now(), start.elapsed());
}
