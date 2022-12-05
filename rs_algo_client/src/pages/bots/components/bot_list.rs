use crate::helpers::status::*;

use rs_algo_shared::helpers::date::{DateTime, Duration, Local, Utc};
use rs_algo_shared::helpers::status::*;
use rs_algo_shared::models::bot::CompactBotData;

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

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    PortfolioAdd,
    PortfolioDelete,
    WatchListAdd,
    WatchListDelete,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub bots: Vec<CompactBotData>,
    //pub on_symbol_click: Callback<String>,
}

#[function_component(BotList)]
pub fn bot_list(props: &Props) -> Html {
    let Props { bots } = props;
    let base_url = get_base_url();
    let url = [base_url.as_str(), "api/instruments/chart/"].concat();

    let instrument_list: Html = bots
        .iter()
        .map(|bot| {
            let on_bot_select = {
                //let on_symbol_click = on_symbol_click.clone();
                // let symbol = instrument.symbol.clone();
                // let url = [url.clone(),symbol].concat();
                // Callback::from(move |_| on_symbol_click.emit(url.clone()))
            };

            let stats = bot.strategy_stats.clone(); 

            let profit_factor_status = get_profit_factor_status(stats.profit_factor);
            let profitable_trades_status = get_profitable_trades_status(stats.profitable_trades);
            //let profit_status = get_profit_status(stats.net_profit_per, stats.avg_buy_hold);
            let max_drawdown_status = get_max_drawdown_status(stats.max_drawdown);
            // let avg_won_status = get_won_per_trade_status(stats.avg_won_per_trade); 
            // let avg_lost_status = get_lost_per_trade_status(stats.avg_lost_per_trade);

            html! {
                <tr>
                    <td> { bot.strategy_name.clone() } </td>
                    <td> { bot.strategy_type.clone() } </td>
                    <td> { bot.time_frame.clone() } </td>
                    <td> { round(bot.strategy_stats.net_profit,2) } </td>
                    <td> { round(bot.strategy_stats.net_profit_per,2) } </td>
                    <td class={get_status_class(&profit_factor_status)}>  { round(bot.strategy_stats.profit_factor,2) } </td>
                    <td> { round(bot.strategy_stats.profitable_trades, 2)} </td>
                    <td class={get_status_class(&max_drawdown_status)}>  { round(bot.strategy_stats.max_drawdown,2) } </td>
                    <td> { bot.strategy_stats.trades } </td>
                    <td> {format!("{} / {}", bot.strategy_stats.wining_trades, bot.strategy_stats.losing_trades)}</td>
                    <td> {format!("{}", bot.last_update.to_chrono().format("%H:%M:%S"))}</td>
                </tr>
            }
        })
        .collect();

    let table = html! {
        <table class="table is-bordered">
            <thead class="has-background-grey-lighter">
                <tr>
                <th>{ "Strategy" }</th>
                <th>{ "Type" }</th>
                <th>{ "TF" }</th>
                <th>{ "Profit" }</th>
                <th>{ "% Profit" }</th>
                <th>{ "Profit F." }</th>
                <th>{ "WinRate" }</th>
                <th>{ "DrawDown" }</th>
                <th>{ "Trades" }</th>
                <th>{ "Won/Lost" }</th>
                <th>{ "Updated" }</th>
                </tr>
            </thead>
            <tbody>
                { instrument_list }
            </tbody>
        </table>
    };

    table
}
