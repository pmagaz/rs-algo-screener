use rs_algo_shared::helpers::calc::*;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::market::*;
use rs_algo_shared::models::order::Order;
use rs_algo_shared::models::strategy::StrategyType;
use rs_algo_shared::models::trade::TradeIn;
use rs_algo_shared::models::trade::TradeOut;
use rs_algo_shared::models::trade::TradeType;
use rs_algo_shared::scanner::instrument::Instrument;

pub fn resolve_backtest(
    instrument: &Instrument,
    strategy_type: &StrategyType,
    trades_in: Vec<TradeIn>,
    trades_out: Vec<TradeOut>,
    orders: Vec<Order>,
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
            instrument.symbol,
            sessions
        );

        BackTestResult::BackTestInstrumentResult(BackTestInstrumentResult {
            instrument: BackTestInstrument {
                symbol: instrument.symbol.to_owned(),
                trades_in,
                trades_out,
                orders,
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
        log::error!(
            "[BACKTEST] No TradesIn for {} backtesting found!",
            instrument.symbol.to_owned()
        );
        //BackTestResult::None
        let fake_date = to_dbtime(Local::now() - Duration::days(1000));
        BackTestResult::BackTestInstrumentResult(BackTestInstrumentResult {
            instrument: BackTestInstrument {
                symbol: instrument.symbol.to_owned(),
                trades_in: vec![],
                trades_out: vec![],
                orders: orders,
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
