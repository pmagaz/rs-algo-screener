use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::candle::Candle;
use std::cmp::Ordering;

pub fn calculate_profit(size: f64, price_in: f64, price_out: f64) -> f64 {
    size * (price_out - price_in)
}

pub fn calculate_profit_per(price_in: f64, price_out: f64) -> f64 {
    ((price_out - price_in) / price_in) * 100.
}

pub fn calculate_cum_profit(size: f64, price_in: f64, price_out: f64) -> f64 {
    size * ((price_out - price_in) / price_in)
}

pub fn calculate_cum_profit_per(size: f64, price_in: f64, price_out: f64) -> f64 {
    (size * ((price_out - price_in) / price_in)) * 100.
}

pub fn calculate_runup(
    data: &Vec<Candle>,
    price_in: f64,
    index_in: usize,
    index_out: usize,
) -> f64 {
    let max_price = data
        .iter()
        .enumerate()
        .filter(|(index, _x)| index >= &index_in && index <= &index_out)
        //   .map(|(_i, x)| x.high)
        .max_by(|a, b| a.1.high.partial_cmp(&b.1.high).unwrap())
        .map(|(_i, x)| x.high)
        .unwrap();
    (max_price - price_in).abs() * 100.
}

pub fn calculate_drawdown(
    data: &Vec<Candle>,
    price_in: f64,
    index_in: usize,
    index_out: usize,
) -> f64 {
    let min_price = data
        .iter()
        .enumerate()
        .filter(|(index, _x)| index >= &index_in && index <= &index_out)
        .map(|(_i, x)| x.low)
        .fold(0. / 0., f64::min);
    (price_in - min_price).abs()
}

pub fn calculate_drawdown_per(draw_down: f64, price_in: f64) -> f64 {
    (draw_down / price_in).abs() * 100.
}

pub fn calculate_runup_per(run_up: f64, price_in: f64) -> f64 {
    (run_up / price_in).abs() * 100.
}

pub fn total_gross(trades_out: &Vec<&TradeOut>) -> f64 {
    trades_out.iter().map(|trade| trade.profit).sum()
}

pub fn total_drawdown(trades_out: &Vec<TradeOut>, equity: f64) -> f64 {
    let mut max_acc_equity = equity;
    let mut min_acc_equity = equity;
    let max_equity = trades_out
        .iter()
        .map(|x| {
            max_acc_equity += x.profit;
            max_acc_equity
        })
        .fold(0. / 0., f64::max);

    let min_equity = trades_out
        .iter()
        .map(|x| {
            let profit = x.profit;
            min_acc_equity -= x.profit;
            min_acc_equity
        })
        .fold(0. / 0., f64::min);
    ((min_equity - max_equity) / max_equity * 100.).abs() * 100.
}

pub fn total_runup(trades_out: &Vec<TradeOut>, equity: f64) -> f64 {
    let mut max_acc_equity = equity;
    let mut min_acc_equity = equity;
    let max_equity = trades_out
        .iter()
        .enumerate()
        .map(|(_i, x)| {
            max_acc_equity += x.profit;
            max_acc_equity
        })
        .fold(0. / 0., f64::max);

    let min_equity = trades_out
        .iter()
        .enumerate()
        .map(|(_i, x)| {
            min_acc_equity += x.profit;
            min_acc_equity
        })
        .fold(0. / 0., f64::min);

    ((max_equity - min_equity) / min_equity * 100.).abs() * 100.
}

pub fn calculate_buy_hold(trades_out: &Vec<TradeOut>, equity: f64, current_price: f64) -> f64 {
    let bought_at = trades_out.first().unwrap().price_in;
    let size = equity / bought_at;
    let sold_at = size * current_price;
    let profit = sold_at - (equity);
    (profit / equity) * 100.
}

pub fn total_commissions(num_trades: usize, commission: f64) -> f64 {
    num_trades as f64 * commission
}

pub fn total_profitable_trades(winning_trades: usize, total_trades: usize) -> f64 {
    ((winning_trades as f64 / total_trades as f64) * 100.).abs()
}

pub fn total_profit_per(trades_out: &Vec<TradeOut>, equity: f64) -> f64 {
    trades_out.iter().map(|trade| trade.profit_per).sum()
}
pub fn total_profit_factor(gross_profits: f64, gross_loses: f64) -> f64 {
    match gross_loses {
        0.0 => 0.,
        _ => (gross_profits / gross_loses).abs(),
    }
}

pub fn get_prev_index(index: usize) -> usize {
    match index.cmp(&0) {
        Ordering::Greater => index - 1,
        Ordering::Equal => 0,
        Ordering::Less => 0,
    }
}