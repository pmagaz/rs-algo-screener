use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::instrument::Instrument;
use rs_algo_shared::models::market::*;

use crate::helpers::calc::*;
use round::round;

pub fn resolve_trade_in(
    index: usize,
    order_size: f64,
    instrument: &Instrument,
    entry_type: TradeType,
    stop_loss: &StopLoss,
) -> TradeResult {
    if entry_type == TradeType::EntryLong || entry_type == TradeType::EntryShort {
        let nex_day_index = index + 1;
        let next_day_candle = instrument.data.get(nex_day_index);
        let next_day_price = match next_day_candle {
            Some(candle) => candle.open,
            None => -100.,
        };
        let current_date = next_day_candle.unwrap().date;

        let quantity = round(order_size / next_day_price, 3);

        TradeResult::TradeIn(TradeIn {
            index_in: nex_day_index,
            price_in: next_day_price,
            quantity,
            stop_loss: create_stop_loss(&entry_type, instrument, nex_day_index, stop_loss),
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
    trade_in: TradeIn,
    exit_type: TradeType,
) -> TradeResult {
    let quantity = trade_in.quantity;
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
    let profit = calculate_profit(quantity, price_in, current_price);
    let profit_per = calculate_profit_per(price_in, current_price);
    let run_up = calculate_runup(data, price_in, index_in, nex_day_index);
    let run_up_per = calculate_runup_per(run_up, price_in);
    let draw_down = calculate_drawdown(data, price_in, index_in, nex_day_index);
    let draw_down_per = calculate_drawdown_per(draw_down, price_in);

    let stop_loss_activated = resolve_stop_loss(current_price, &trade_in);

    // log::info!("3333333 {:?}", trade_in);

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
    strategy_type: &StrategyType,
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

        let w_trades: Vec<&TradeOut> = trades_out.iter().filter(|x| x.profit > 0.).collect();
        let l_trades: Vec<&TradeOut> = trades_out.iter().filter(|x| x.profit <= 0.).collect();
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
        let gross_profit = gross_profits + gross_loses;
        let commissions = total_commissions(trades, commission);
        let net_profit = gross_profit - commissions;
        let first = trades_in.first().unwrap();

        let initial_order_amount = (first.price_in * first.quantity).ceil();
        let profit_factor = total_profit_factor(gross_profits, gross_loses);

        let net_profit_per = total_profit_per(equity, net_profit, &trades_in, &trades_out);
        //let net_profit_per = total_profit_per(equity, net_profit);
        let profitable_trades = total_profitable_trades(wining_trades, trades);
        let max_drawdown = total_drawdown(&trades_out, equity);

        let max_runup = total_runup(&trades_out, equity);

        let strategy_start_price = match instrument.data.first().map(|x| x.open) {
            Some(open) => open,
            _ => 0.,
        };

        let buy_hold =
            calculate_buy_hold(strategy_start_price, initial_order_amount, current_price);
        let annual_return = 100.;

        log::info!(
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
            strategy_type: strategy_type.to_owned(),
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
        log::info!(
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
            strategy_type: strategy_type.to_owned(),
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

pub fn init_stop_loss(stop_type: StopLossType, value: f64) -> StopLoss {
    StopLoss {
        price: 0.,
        value,
        stop_type,
        created_at: to_dbtime(Local::now()),
        updated_at: to_dbtime(Local::now()),
        valid_until: to_dbtime(Local::now() + Duration::days(1000)),
    }
}

pub fn create_stop_loss(
    entry_type: &TradeType,
    instrument: &Instrument,
    index: usize,
    stop_loss: &StopLoss,
) -> StopLoss {
    let current_price = &instrument.data.get(index).unwrap().open;
    let stop_loss_value = stop_loss.value;
    let stop_loss_price = stop_loss.price;

    let price = match stop_loss.stop_type {
        StopLossType::Atr => {
            let atr_value = instrument.indicators.atr.data_a.get(index).unwrap() * stop_loss_value;
            let price = match entry_type {
                TradeType::EntryLong => current_price - atr_value,
                TradeType::EntryShort => current_price + atr_value,
                _ => current_price - atr_value,
            };
            price
        }
        _ => {
            stop_loss_price
        },
    };


    // let price = match entry_type {
    //     TradeType::EntryLong => current_price - atr_value,
    //     TradeType::EntryShort => current_price + atr_value,
    //     _ => current_price - atr_value,
    // };

    StopLoss {
        price,
        value: stop_loss_value,
        stop_type: stop_loss.stop_type.to_owned(), 
        created_at: to_dbtime(Local::now()),
        updated_at: to_dbtime(Local::now()),
        valid_until: to_dbtime(Local::now() + Duration::days(1000)),
    }
}

pub fn update_stop_loss_values(
    stop_loss: &StopLoss,
    stop_type: StopLossType,
    price: f64,
) -> StopLoss {
    StopLoss {
        price,
        value: stop_loss.value,
        stop_type,
        created_at: stop_loss.created_at,
        updated_at: to_dbtime(Local::now()),
        valid_until: stop_loss.valid_until,
    }
}

pub fn resolve_stop_loss(current_price: f64, trade_in: &TradeIn) -> bool {
    
    let stop_loss_price = trade_in.stop_loss.price;

    match trade_in.trade_type {
        TradeType::EntryLong => current_price <= stop_loss_price,
        TradeType::EntryShort => current_price >= stop_loss_price,
        _ => current_price - current_price <= stop_loss_price,
    }
}
