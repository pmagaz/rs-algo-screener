use crate::trade::*;
use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::models::backtest_instrument::*;

pub fn calculate_profit(size: f64, price_in: f64, price_out: f64) -> f64 {
    size * (price_out - price_in)
}

pub fn calculate_profit_per(price_in: f64, price_out: f64) -> f64 {
    percentage_change(price_in, price_out)
}

pub fn calculate_cum_profit_per(size: f64, price_in: f64, price_out: f64) -> f64 {
    (size * ((price_out - price_in) / price_in)) * 100.
}

pub fn calculate_drawdown_per(size: f64, price_in: f64, low: f64) -> f64 {
    (size * (low - price_in) / (price_in)) * 100.
}

pub fn calculate_runup(size: f64, price_in: f64, high: f64) -> f64 {
    size * (high - price_in)
}

pub fn calculate_drawdown(size: f64, price_in: f64, low: f64) -> f64 {
    size * (low - price_in)
}

pub fn calculate_runup_per(size: f64, price_in: f64, high: f64) -> f64 {
    (size * (high - price_in) / (price_in)) * 100.
}

pub fn total_profit(trades_out: &Vec<TradeOut>) -> f64 {
    let profit_sum: f64 = trades_out.iter().map(|trade| trade.profit).sum();
    (profit_sum * trades_out.len() as f64) / 100.
}

pub fn total_drawdown(trades_out: &Vec<TradeOut>) -> f64 {
    trades_out.iter().map(|trade| trade.draw_down).sum()
}

pub fn total_runup(trades_out: &Vec<TradeOut>) -> f64 {
    trades_out.iter().map(|trade| trade.run_up).sum()
}

pub fn total_profitable_trades(trades_out: &Vec<TradeOut>) -> f64 {
    let total_trades = trades_out.len() as f64;
    let profitable_trades: Vec<&TradeOut> = trades_out
        .iter()
        .filter(|trade| trade.profit > 0.)
        .collect();
    let num_profitable = profitable_trades.len() as f64;
    (num_profitable / total_trades) * 100.
}

pub fn total_profit_factor(trades_out: &Vec<TradeOut>) -> f64 {
    let profitable: Vec<f64> = trades_out
        .iter()
        .filter(|trade| trade.profit >= 0.)
        .map(|trade| trade.profit)
        .collect();

    let non_profitable: Vec<f64> = trades_out
        .iter()
        .filter(|trade| trade.profit < 0.)
        .map(|trade| trade.profit)
        .collect();

    let num_profitable: f64 = profitable.iter().sum();
    let num_no_profitable: f64 = non_profitable.iter().sum();
    if num_no_profitable == 0. {
        num_profitable.abs()
    } else {
        num_profitable.abs() / num_no_profitable.abs()
    }
}
