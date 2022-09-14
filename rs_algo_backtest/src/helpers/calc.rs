use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::candle::Candle;
use rs_algo_shared::models::instrument::*;
use rs_algo_shared::models::pattern::*;
use rs_algo_shared::models::time_frame::*;
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

pub fn avg_per_trade(trades_out: &Vec<&TradeOut>) -> f64 {
    if trades_out.is_empty() {
        0.01
    } else {
        let profit_per_trade: f64 = trades_out.iter().map(|trade| trade.profit_per).sum();
        profit_per_trade / trades_out.len() as f64
    }
}

pub fn total_drawdown(trades_out: &Vec<TradeOut>, equity: f64) -> f64 {
    let mut max_acc_equity = equity;
    let mut max_equity_index: usize = 0;
    let max_equity = trades_out
        .iter()
        .enumerate()
        .map(|(idx, x)| {
            max_acc_equity += x.profit;
            max_equity_index = idx;
            max_acc_equity
        })
        .fold(0. / 0., f64::max);

    let mut min_acc_equity = max_equity;

    let min_equity = trades_out
        .iter()
        .enumerate()
        .filter(|(idx, x)| idx >= &max_equity_index)
        .map(|(_idx, x)| {
            min_acc_equity += x.profit;
            min_acc_equity
        })
        .fold(0. / 0., f64::min);

    ((min_equity - max_equity) / max_equity * 100.).abs()
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

// pub fn calculate_annual_return(
//     trades_out: &Vec<TradeOut>,
//     profit_factor: f64,
//     profitable_trades: usize,
//     max_drawdown: f64,
// ) -> f64 {
//     let bought_at = trades_out.first().unwrap().price_in;
//     let size = equity / bought_at;
//     let sold_at = size * current_price;
//     let profit = sold_at - (equity);
//     (profit / equity) * 100.
// }

pub fn calculate_buy_hold(bought_at: f64, initial_equity: f64, current_price: f64) -> f64 {
    let size = initial_equity / bought_at;
    let sold_at = size * current_price;
    percentage_change(initial_equity, sold_at)
}

pub fn total_commissions(num_trades: usize, commission: f64) -> f64 {
    num_trades as f64 * commission
}

pub fn total_profitable_trades(winning_trades: usize, total_trades: usize) -> f64 {
    ((winning_trades as f64 / total_trades as f64) * 100.).abs()
}

pub fn total_profit_per(
    equity: f64,
    net_profit: f64,
    _trades_in: &Vec<TradeIn>,
    _trades_out: &Vec<TradeOut>,
) -> f64 {
    let initial_value = equity;
    let end_value = initial_value + net_profit;
    percentage_change(initial_value, end_value)
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

pub fn get_proportion(_base_time_frame: &TimeFrame, _time_frame: &TimeFrame) -> usize {
    // match time_frame {
    //     TimeFrameType::D => 5,
    // }
    1
}

pub fn get_current_pattern(index: usize, patterns: &Vec<Pattern>) -> PatternType {
    let last_pattern = patterns.iter().filter(|pat| pat.index < index).last();
    match last_pattern {
        Some(pattern) => pattern.pattern_type.clone(),
        None => PatternType::None,
    }
}

pub fn get_upper_timeframe_data<F>(
    index: usize,
    instrument: &Instrument,
    upper_tf_instrument: &HigherTMInstrument,
    mut callback: F,
) -> bool
where
    F: Send + FnMut((usize, usize, &Instrument)) -> bool,
{
    let base_date = &instrument.data.get(index).unwrap().date;
    let upper_tf_data = match upper_tf_instrument {
        HigherTMInstrument::HigherTMInstrument(upper_instrument) => {
            let upper_indexes: Vec<usize> = upper_instrument
                .data
                .iter()
                .enumerate()
                .filter(|(_id, x)| &x.date <= base_date)
                .map(|(id, _x)| id)
                .collect();

            let upper_tf_indx = match upper_indexes.last() {
                Some(val) => *val,
                _ => 0,
            };

            let prev_upper_tf_indx = get_prev_index(upper_tf_indx);

            (upper_tf_indx, prev_upper_tf_indx, upper_instrument)
        }
        _ => (0, 0, instrument),
    };
    callback(upper_tf_data)
}
