use crate::helpers::status::*;

use rs_algo_shared::helpers::status::*;
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
    pub market: String,
}

#[function_component(StrategyDetail)]
pub fn strategy_detail(props: &Props) -> Html {
    let Props {
        market,
        backtested_instruments,
        on_symbol_click,
    } = props;
    let base_url = get_base_url();

    let backtest_instruments: Html = backtested_instruments
        .iter()
        .map(|backtest_instrument| {

            let on_instrument_select = {
                let on_symbol_click = on_symbol_click.clone();
                let symbol = backtest_instrument.instrument.symbol.clone();
                let strategy = backtest_instrument.strategy.clone();
                let strategy_type = backtest_instrument.strategy_type.clone();

                let replace = ["strategy/", market, "/", &strategy, "/", &strategy_type.to_string()].concat();

                let url = [
                    base_url.replace(replace.as_str(), "api/backtest/strategies/chart"),
                    market.to_string(),
                    "/".to_owned(),
                    strategy.to_string(),
                    "/".to_owned(),
                    strategy_type.to_string(),
                    "?symbol=".to_string(),
                    symbol
                ]
                .concat();
                Callback::from(move |_| on_symbol_click.emit(url.clone()))
            };

            let profit_factor_status = get_profit_factor_status(backtest_instrument.profit_factor);
            let profitable_trades_status = get_profitable_trades_status(backtest_instrument.profitable_trades);
            let profit_status = get_profit_status(backtest_instrument.net_profit_per, backtest_instrument.buy_hold);
            let max_drawdown_status = get_max_drawdown_status(backtest_instrument.max_drawdown);
            let avg_won_status = get_won_per_trade_status(backtest_instrument.won_per_trade_per); 
            let avg_lost_status = get_lost_per_trade_status(backtest_instrument.lost_per_trade_per);

            html! {
                <tr>
                    <td  onclick={ on_instrument_select }><a href={format!("javascript:void(0);")}>{backtest_instrument.instrument.symbol.clone()}</a></td>
                    <td class={get_status_class(&profit_status)}> { format!("{}%", round(backtest_instrument.net_profit_per,2))}</td>
                    <td class={get_status_class(&profit_factor_status)}> { round(backtest_instrument.profit_factor,2) }</td>
                    <td class={get_status_class(&profitable_trades_status)}> { format!("{}%", round(backtest_instrument.profitable_trades,2))}</td>
                    <td class={get_status_class(&max_drawdown_status)}>{ format!("{}%", round(backtest_instrument.max_drawdown,2))}</td>
                    <td>{ backtest_instrument.trades}</td>
                    <td class={get_status_class(&avg_won_status)}>{ format!("{}%", round(backtest_instrument.won_per_trade_per,2))}</td>
                    <td class={get_status_class(&avg_lost_status)}>{ format!("{}%", round(backtest_instrument.lost_per_trade_per,2))}</td>
                    <td>{ format!("{} / {}", backtest_instrument.wining_trades, backtest_instrument.losing_trades)} </td>
                    <td>{ backtest_instrument.stop_losses}</td>
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
                <th><abbr>{ "Net Profit" }</abbr></th>
                <th><abbr>{ "Profit F." }</abbr></th>
                <th><abbr>{ "Win Rate" }</abbr></th>
                <th><abbr>{ "Drawdown" }</abbr></th>
                <th><abbr>{ "Trades" }</abbr></th>
                <th><abbr>{ "Avg Won" }</abbr></th>
                <th><abbr>{ "Avg Lost" }</abbr></th>
                <th><abbr>{ "Won / Lost" }</abbr></th>
                <th><abbr>{ "Stops " }</abbr></th>
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
