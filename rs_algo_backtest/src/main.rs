use std::time::Instant;

use rs_algo_shared::error::Result;
use rs_algo_shared::helpers::date::Local;

use dotenv::dotenv;
use std::env;

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
    let env = env::var("ENV").unwrap();
    let order_size = env::var("BACKTEST_ORDER_SIZE")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let stop_loss = env::var("BACKTEST_ATR_STOP_LOSS")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let commission = env::var("BACKTEST_COMISSION")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let equity = env::var("BACKTEST_EQUITY").unwrap().parse::<f64>().unwrap();

    let portfolio = PortFolio {
        order_size,
        stop_loss,
        commission,
        equity,
        instruments: vec![],
        strategies: vec![
            // MultiTimeFrame
            Box::new(strategies::macd_dual::MacdDual::new().unwrap()),
            Box::new(strategies::macd_weekly::MacdWeekly::new().unwrap()),
            Box::new(strategies::stoch_ls_mt_macd::Stoch::new().unwrap()),
            Box::new(strategies::ema_50200_ls_mt_macd::Ema::new().unwrap()),
            Box::new(
                strategies::bollinger_bands_reversals_mt_macd::MutiTimeFrameBollingerBands::new()
                    .unwrap(),
            ),
            Box::new(
                strategies::bollinger_bands_reversals2_mt_macd::MutiTimeFrameBollingerBands::new()
                    .unwrap(),
            ),
            Box::new(
                strategies::bollinger_bands_reversals_continuation_mt_macd::MutiTimeFrameBollingerBands::new()
                    .unwrap(),
            ),
            Box::new(
                strategies::bollinger_bands_reversals_continuation_ls_mt_macd::MutiTimeFrameBollingerBands::new()
                    .unwrap(),
            ),
            // // // OnlyLong
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
            Box::new(strategies::stoch::Stoch::new().unwrap()),
            // // LongShort
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
            Box::new(strategies::bollinger_bands_reversal_2::BollingerBands::new().unwrap()),
            Box::new(strategies::stoch_ls::Stoch::new().unwrap()),
            Box::new(strategies::bollinger_bands_reversal::BollingerBands::new().unwrap()),
            Box::new(strategies::ema_200::Ema::new().unwrap()),
            Box::new(strategies::ema_50200::Ema::new().unwrap()),
            Box::new(strategies::ema_50::Ema::new().unwrap()),
            Box::new(strategies::macd::Macd::new().unwrap()),
            Box::new(strategies::rsi::Rsi::new().unwrap()),
            Box::new(strategies::macd_rsi::Macd::new().unwrap()),
            Box::new(strategies::macd_over_zero::Macd::new().unwrap()),
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

    println!("[Finished] at {:?}  in {:?}", Local::now(), start.elapsed());

    Ok(())
}
