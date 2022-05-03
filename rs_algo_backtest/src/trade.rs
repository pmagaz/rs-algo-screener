use crate::helpers::calc::*;

use rs_algo_shared::helpers::date::DbDateTime;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::instrument::Instrument;

pub fn resolve_trade_in(
    index: usize,
    instrument: &Instrument,
    result: bool,
    stop_loss: f64,
) -> TradeResult {
    let current_candle = instrument.data.get(index);
    let current_price = match current_candle {
        Some(candle) => candle.close,
        None => -100.,
    };
    let current_date = current_candle.unwrap().date;

    if result {
        TradeResult::TradeIn(TradeIn {
            index_in: index,
            price_in: current_price,
            stop_loss: stop_loss,
            date_in: DbDateTime::from_chrono(current_date),
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
    result: bool,
) -> TradeResult {
    let size = 1.;
    let current_candle = instrument.data.get(index);
    let current_price = match current_candle {
        Some(candle) => candle.close,
        None => -100.,
    };
    let current_date = current_candle.unwrap().date;
    let index_in = trade_in.index_in;
    let price_in = trade_in.price_in;

    let profit = calculate_profit(size, price_in, current_price);
    let profit_per = calculate_profit_per(price_in, current_price);
    let cum_profit = calculate_cum_profit_per(size, price_in, current_price);
    //let cum_profit_per = calculate_cum_profit(size, price_in, current_price); //FIXME
    let run_up = calculate_runup_per(size, price_in, current_candle.unwrap().high);
    //let run_up_per = calculate_runup(size, price_in, current_candle.unwrap().high); //FIXME
    let draw_down = calculate_drawdown_per(size, price_in, current_candle.unwrap().low);
    //let draw_down_per = calculate_drawdown(size, price_in, current_candle.unwrap().low);

    if result {
        TradeResult::TradeOut(TradeOut {
            index_in: index_in,
            price_in: price_in,
            date_in: DbDateTime::from_chrono(current_date),
            index_out: index,
            price_out: current_price,
            date_out: DbDateTime::from_chrono(current_date),
            profit,
            profit_per,
            cum_profit,
            //cum_profit_per,
            run_up,
            //run_up_per,
            draw_down,
            //draw_down_per,
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
) -> BackTestResult {
    let size = 1.;
    let last_candle = instrument.data.last().unwrap();
    let trades = trades_out.len();
    let net_profit = total_profit(&trades_out);
    let profitable_per = total_profitable_trades(&trades_out);
    let profit_factor = total_profit_factor(&trades_out);
    let max_drawdown = total_drawdown(&trades_out);
    let max_runup = total_runup(&trades_out);
    let buy_hold = calculate_profit(size, trades_in[0].price_in, last_candle.close);
    let annual_return = 100.;

    BackTestResult {
        instrument: BackTestInstrument {
            symbol: instrument.symbol.to_owned(),
            trades_in,
            trades_out,
        },
        trades,
        strategy: name.to_owned(),
        net_profit,
        profitable_per,
        profit_factor,
        max_runup,
        max_drawdown,
        buy_hold,
        annual_return,
    }
}
