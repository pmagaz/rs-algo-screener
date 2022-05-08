use crate::helpers::calc::*;

use rs_algo_shared::helpers::date::DbDateTime;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::Instrument;

pub fn resolve_trade_in(
    index: usize,
    instrument: &Instrument,
    result: bool,
    stop_loss: f64,
) -> TradeResult {
    let nex_day_index = index + 1;
    let current_candle = instrument.data.get(nex_day_index);
    let current_price = match current_candle {
        Some(candle) => candle.open,
        None => -100.,
    };
    let current_date = current_candle.unwrap().date;

    if result {
        TradeResult::TradeIn(TradeIn {
            index_in: nex_day_index,
            price_in: current_price,
            stop_loss: stop_loss,
            date_in: to_dbtime(current_date),
            trade_type: TradeType::Entry(TradeDirection::Long),
        })
    } else {
        TradeResult::None
    }
}

pub fn resolve_trade_out(
    index: usize,
    instrument: &Instrument,
    trade_in: &TradeIn,
    exit_condition: bool,
    stop_loss: f64,
) -> TradeResult {
    let size = 1.;
    let data = &instrument.data;
    let nex_day_index = index + 1;
    let index_in = trade_in.index_in;
    let price_in = trade_in.price_in;
    let current_candle = data.get(nex_day_index);
    let current_price = match current_candle {
        Some(candle) => candle.open,
        None => -100.,
    };

    let date_in = instrument.data.get(index_in).unwrap().date;
    let date_out = current_candle.unwrap().date;
    let profit = calculate_profit(size, price_in, current_price);
    let profit_per = calculate_profit_per(price_in, current_price);
    let run_up = calculate_runup(data, price_in, index_in, nex_day_index);
    let run_up_per = calculate_runup_per(run_up, price_in);
    let draw_down = calculate_drawdown(data, price_in, index_in, nex_day_index);
    let draw_down_per = calculate_drawdown_per(draw_down, price_in);

    //FIXME CHECK STOP LOSS
    let trade_type = TradeType::Exit(TradeDirection::Long);

    if exit_condition {
        TradeResult::TradeOut(TradeOut {
            index_in: index_in,
            price_in: price_in,
            trade_type,
            date_in: to_dbtime(date_in),
            index_out: index,
            price_out: current_price,
            date_out: to_dbtime(date_out),
            profit,
            profit_per,
            run_up,
            run_up_per,
            draw_down,
            draw_down_per,
        })
    } else {
        TradeResult::None
    }
}

pub fn resolve_backtest(
    instrument: &Instrument,
    trades_in: Vec<TradeIn>,
    trades_out: Vec<TradeOut>,
    name: &str,
    equity: f64,
    commission: f64,
) -> BackTestResult {
    let size = 1.;
    let data = &instrument.data;
    if trades_out.len() > 0 {
        let date_start = trades_out[0].date_in;
        let date_end = trades_out.last().unwrap().date_out;
        let sessions: usize = trades_out.iter().fold(0, |mut acc, x| {
            acc += x.index_out - x.index_in;
            acc
        });
        let current_candle = data.last().unwrap();
        let current_price = current_candle.close;
        let w_trades: Vec<&TradeOut> = trades_out.iter().filter(|x| x.profit >= 0.).collect();
        let l_trades: Vec<&TradeOut> = trades_out.iter().filter(|x| x.profit < 0.).collect();
        let wining_trades = w_trades.len();
        let losing_trades = l_trades.len();
        let trades = wining_trades + losing_trades;
        let gross_profits = total_gross(&w_trades);
        let gross_loses = total_gross(&l_trades);
        let gross_profit = gross_profits - gross_loses;
        let commissions = total_commissions(trades, commission);
        let net_profit = gross_profit - commissions;
        let net_profit_per = net_profit / 100.;
        let profitable_trades = total_profitable_trades(wining_trades, trades);
        let profit_factor = total_profit_factor(gross_profits, gross_loses);
        let max_drawdown = total_drawdown(&trades_out, equity);
        let max_runup = total_runup(&trades_out, equity);
        let buy_hold = calculate_buy_hold(&trades_out, equity, current_price);
        let annual_return = 100.;
        println!(
            "[BACKTEST] {:} backtested for {:?}",
            instrument.symbol, sessions
        );
        BackTestResult::BackTestInstrumentResult(BackTestInstrumentResult {
            instrument: BackTestInstrument {
                symbol: instrument.symbol.to_owned(),
                trades_in,
                trades_out,
            },
            strategy: name.to_owned(),
            date_start,
            date_end,
            sessions,
            trades,
            wining_trades,
            losing_trades,
            gross_profit,
            commissions,
            net_profit,
            net_profit_per,
            profitable_trades,
            profit_factor,
            max_runup,
            max_drawdown,
            buy_hold,
            annual_return,
        })
    } else {
        BackTestResult::None
    }
}
