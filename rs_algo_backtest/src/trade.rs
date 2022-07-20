use crate::helpers::calc::*;

use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::Instrument;

pub fn resolve_trade_in(
    index: usize,
    instrument: &Instrument,
    entry_type: TradeType,
    stop_loss: f64,
) -> TradeResult {
    if entry_type == TradeType::EntryLong || entry_type == TradeType::EntryShort {
        let nex_day_index = index + 1;
        let current_candle = instrument.data.get(nex_day_index);
        let current_price = match current_candle {
            Some(candle) => candle.open,
            None => -100.,
        };
        let current_date = current_candle.unwrap().date;

        TradeResult::TradeIn(TradeIn {
            index_in: nex_day_index,
            price_in: current_price,
            stop_loss: calculate_stoploss(instrument, nex_day_index, stop_loss),
            date_in: to_dbtime(current_date),
            trade_type: entry_type,
        })
    } else {
        TradeResult::None
    }
}

pub fn resolve_trade_out(
    index: usize,
    instrument: &Instrument,
    trade_in: &TradeIn,
    exit_type: TradeType,
    stop_loss: bool,
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

    let stop_loss_activated = match stop_loss {
        true => resolve_stoploss(current_price, trade_in),
        false => false,
    };

    if index > trade_in.index_in
        && (exit_type == TradeType::ExitLong
            || exit_type == TradeType::ExitShort
            || stop_loss_activated)
    {
        let trade_type = match stop_loss_activated {
            true => TradeType::StopLoss,
            false => exit_type,
        };

        TradeResult::TradeOut(TradeOut {
            index_in,
            price_in,
            trade_type,
            date_in: to_dbtime(date_in),
            index_out: nex_day_index,
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
    let _size = 1.;
    let data = &instrument.data;
    if !trades_out.is_empty() {
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
        let won_per_trade_per = avg_per_trade(&w_trades);
        let lost_per_trade_per = avg_per_trade(&l_trades);
        let stop_losses = trades_out
            .iter()
            .filter(|x| x.trade_type == TradeType::StopLoss)
            .count();
        let gross_profits = total_gross(&w_trades);
        let gross_loses = total_gross(&l_trades);
        let gross_profit = gross_profits - gross_loses;
        let commissions = total_commissions(trades, commission);
        let net_profit = gross_profit - commissions;
        let net_profit_per = total_profit_per(&trades_in, &trades_out);
        let profitable_trades = total_profitable_trades(wining_trades, trades);
        let profit_factor = total_profit_factor(gross_profits, gross_loses);
        let max_drawdown = total_drawdown(&trades_out, equity);
        let max_runup = total_runup(&trades_out, equity);
        let buy_hold = calculate_buy_hold(&trades_out, equity, current_price);
        let annual_return = 100.;

        println!(
            "[BACKTEST] {:} backtested for {:?} sessions",
            instrument.symbol, sessions
        );

        BackTestResult::BackTestInstrumentResult(BackTestInstrumentResult {
            instrument: BackTestInstrument {
                symbol: instrument.symbol.to_owned(),
                trades_in,
                trades_out,
            },
            strategy: name.to_owned(),
            market: Market::Stock,
            date_start,
            date_end,
            sessions,
            trades,
            wining_trades,
            losing_trades,
            won_per_trade_per,
            lost_per_trade_per,
            stop_losses,
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
        println!(
            "[BACKTEST] Error! backtesing {}",
            instrument.symbol.to_owned()
        );
        //BackTestResult::None
        let fake_date = to_dbtime(Local::now() - Duration::days(1000));
        BackTestResult::BackTestInstrumentResult(BackTestInstrumentResult {
            instrument: BackTestInstrument {
                symbol: instrument.symbol.to_owned(),
                trades_in: vec![],
                trades_out: vec![],
            },
            strategy: name.to_owned(),
            market: Market::Stock,
            date_start: fake_date,
            date_end: fake_date,
            sessions: 0,
            trades: 0,
            wining_trades: 0,
            losing_trades: 0,
            won_per_trade_per: 0.,
            lost_per_trade_per: 0.,
            stop_losses: 0,
            gross_profit: 0.,
            commissions: 0.,
            net_profit: 0.,
            net_profit_per: 0.,
            profitable_trades: 0.,
            profit_factor: 0.,
            max_runup: 0.,
            max_drawdown: 0.,
            buy_hold: 0.,
            annual_return: 0.,
        })
    }
}

pub fn calculate_stoploss(instrument: &Instrument, index: usize, stop_loss: f64) -> f64 {
    let current_price = &instrument.data.get(index).unwrap().open;
    let atr_value = instrument.indicators.atr.data_a.get(index).unwrap() * stop_loss;
    current_price - atr_value
}

pub fn resolve_stoploss(current_price: f64, trade_in: &TradeIn) -> bool {
    let stop_loss = trade_in.stop_loss;
    current_price <= stop_loss
    //false
}
