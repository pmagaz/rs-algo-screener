use rs_algo_shared::models::backtest_instrument::*;
use rs_algo_shared::models::status::Status;
use wasm_bindgen::prelude::*;
use yew::{function_component, html, Callback, Html, Properties};

use round::round;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = get_query_value)]
    fn open_modal(modal: &str);
    #[wasm_bindgen(js_name = get_base_url)]
    fn get_base_url() -> String;
    fn close_modal(modal: &str);
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub backtested_instruments: Vec<BackTestInstrumentResult>,
    pub on_symbol_click: Callback<String>,
}

#[function_component(StrategyDetail)]
pub fn strategy_detail(props: &Props) -> Html {
    let Props {
        backtested_instruments,
        on_symbol_click,
    } = props;
    let base_url = get_base_url();

    fn get_status_class<'a>(status: &Status) -> &'a str {
        let class = match status {
            Status::Default => "",
            //Status::Neutral => "",
            Status::Bullish => "has-background-primary-light",
            Status::Bearish => "has-background-danger-light",
            Status::Neutral => "has-background-warning-light",
        };
        class
    }

    let backtest_instruments: Html = backtested_instruments
        .iter()
        .map(|backtest_instrument| {

            let on_instrument_select = {
                let on_symbol_click = on_symbol_click.clone();
                let symbol = backtest_instrument.instrument.symbol.clone();
                let strategy = backtest_instrument.strategy.clone();
                let url = [
                    base_url.replace(&["strategy/", &strategy].concat(), "").as_str(),
                    "api/backtest/strategies/chart/",
                    &strategy,
                    "?symbol=",
                    &symbol
                ]
                 .concat();
                Callback::from(move |_| on_symbol_click.emit(url.clone()))
            };


            let profit_factor = backtest_instrument.profit_factor;
            let profit_factor_status = match profit_factor {
                _x if profit_factor <= 2. => Status::Bearish,
                _x if profit_factor > 2.0 && profit_factor <= 2.5 => Status::Neutral,
                _x if profit_factor > 2.5 => Status::Bullish,
                _ => Status::Neutral,
            };


             let profitable_trades = backtest_instrument.profitable_trades;
            let profitable_trades_status = match profitable_trades {
                _x if profitable_trades <= 40. => Status::Bearish,
                _x if profitable_trades > 40. && profitable_trades < 50. => Status::Neutral,
                _x if profitable_trades > 50. => Status::Bullish,
                _ => Status::Neutral,
            };

            let max_drawdown = backtest_instrument.max_drawdown;
            let max_drawdown_status = match max_drawdown {
                _x if max_drawdown > 25. => Status::Bearish,
                _x if max_drawdown > 15. && max_drawdown < 25. => Status::Neutral,
                _x if max_drawdown <= 15. => Status::Bullish,
                _ => Status::Neutral,
            };

            let profit = backtest_instrument.net_profit_per;
            let profit_status = match profit {
                _x if profit <= 10. => Status::Bearish,
                _x if profit > 10. && profitable_trades < 12. => Status::Neutral,
                _x if profit >= 15. => Status::Bullish,
                _ => Status::Neutral,
            };

            let won_per_trade = backtest_instrument.won_per_trade_per; 
            let avg_won_status = match won_per_trade {
                _x if won_per_trade > 15. => Status::Bullish,
                _x if won_per_trade > 10. && won_per_trade < 15. => Status::Neutral,
                _x if won_per_trade <= 10. => Status::Bullish,
                _ => Status::Neutral,
            };


            let lost_per_trade = backtest_instrument.lost_per_trade_per;
            let avg_lost_status = match lost_per_trade {
                _x if lost_per_trade > -5. => Status::Bullish,
                _x if lost_per_trade < -5. && lost_per_trade > -10.  => Status::Neutral,
                _x if lost_per_trade <= -10. => Status::Bearish,
                _ => Status::Neutral,
            };

            html! {
                <tr>
                    <td  onclick={ on_instrument_select }><a href={format!("javascript:void(0);")}>{backtest_instrument.instrument.symbol.clone()}</a></td>
                    <td class={get_status_class(&profitable_trades_status)}> { format!("{}%", round(backtest_instrument.profitable_trades,2))}</td>
                    <td class={get_status_class(&profit_factor_status)}> { format!("{}%", round(backtest_instrument.profit_factor,2))}</td>
                    <td class={get_status_class(&max_drawdown_status)}>{ format!("{}%", round(backtest_instrument.max_drawdown,2))}</td>
                    <td>{ backtest_instrument.trades}</td>
                    <td>{ format!("{} / {}", backtest_instrument.wining_trades, backtest_instrument.losing_trades)} </td>
                    <td class={get_status_class(&avg_won_status)}>{ format!("{}%", round(backtest_instrument.won_per_trade_per,2))}</td>
                    <td class={get_status_class(&avg_lost_status)}>{ format!("{}%", round(backtest_instrument.lost_per_trade_per,2))}</td>
                    <td>{ backtest_instrument.stop_losses}</td>
                    <td>{ format!("{}%", round(backtest_instrument.net_profit_per,2))}</td>
                    <td>{ format!("{}%", round(backtest_instrument.buy_hold,2))}</td>
                </tr>
            }
        })
        .collect();

    let table = html! {
        <table class="table is-bordered">
            <thead class="has-background-grey-lighter">
                <tr>
                <th><abbr>{ "Instrument" }</abbr></th>
                <th><abbr>{ "Win Rate" }</abbr></th>
                <th><abbr>{ "Profit Factor" }</abbr></th>
                <th><abbr>{ "Drawdown" }</abbr></th>
                <th><abbr>{ "Trades" }</abbr></th>
                <th><abbr>{ "Won / Lost" }</abbr></th>
                <th><abbr>{ "Won p trade" }</abbr></th>
                <th><abbr>{ "Lost p trade" }</abbr></th>
                <th><abbr>{ "Stops " }</abbr></th>
                <th><abbr>{ "Net Profit" }</abbr></th>
                <th><abbr>{ "Buy & Hold" }</abbr></th>
                </tr>
            </thead>
            <tbody>
                { backtest_instruments }
            </tbody>
        </table>
    };

    table
}
