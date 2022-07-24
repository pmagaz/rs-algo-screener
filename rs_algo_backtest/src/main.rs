use std::time::Instant;

use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::date::Local;

use dotenv::dotenv;

mod helpers;
mod portfolio;
mod strategies;
mod trade;

use portfolio::PortFolio;
use strategies::strategy::Strategy;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let start = Instant::now();

    let portfolio = PortFolio {
        order_size: 1,
        stop_loss: 2.,
        commission: 0.012,
        equity: 100000.,
        instruments: vec![],
        strategies: vec![
            //OnlyLong
            Box::new(strategies::ema_200::Ema::new().unwrap()),
            Box::new(strategies::ema_50::Ema::new().unwrap()),
            Box::new(strategies::ema_50200::Ema::new().unwrap()),
            Box::new(
                strategies::bollinger_bands_reversal_riding_rsi::BollingerBands::new().unwrap(),
            ),
            Box::new(strategies::bollinger_bands_reversal_riding::BollingerBands::new().unwrap()),
            Box::new(
                strategies::bollinger_bands_reversal_continuation::BollingerBands::new().unwrap(),
            ),
            Box::new(strategies::bollinger_bands_reversal::BollingerBands::new().unwrap()),
            Box::new(strategies::macd_over_zero::Macd::new().unwrap()),
            //LongShort
            Box::new(strategies::ema_200_ls::Ema::new().unwrap()),
            Box::new(strategies::ema_50200_ls::Ema::new().unwrap()),
            Box::new(
                strategies::bollinger_bands_reversal_riding_rsi_ls::BollingerBands::new().unwrap(),
            ),
            Box::new(strategies::bollinger_bands_reversal_ls::BollingerBands::new().unwrap()),
            Box::new(
                strategies::bollinger_bands_reversal_continuation_ls::BollingerBands::new()
                    .unwrap(),
            ),
            //Box::new(strategies::bollinger_bands_reversal_2::BollingerBands::new().unwrap()),

            // Box::new(strategies::bollinger_bands_reversal::BollingerBands::new().unwrap()),
            // Box::new(strategies::ema_200::Ema::new().unwrap()),
            // Box::new(strategies::ema_50200::Ema::new().unwrap()),
            // Box::new(strategies::ema_50::Ema::new().unwrap()),
            // Box::new(strategies::stoch::Stoch::new().unwrap()),
            // Box::new(strategies::macd::Macd::new().unwrap()),
            // Box::new(strategies::rsi::Rsi::new().unwrap()),
            // Box::new(strategies::macd_rsi::Macd::new().unwrap()),
            // Box::new(strategies::macd_over_zero::Macd::new().unwrap()),
        ],
    };

    portfolio.backtest().await;

    println!("[Finished] at {:?}  in {:?}", Local::now(), start.elapsed());

    Ok(())
}
