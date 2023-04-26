
use rs_algo_shared::{models::{bot::CompactBotData, status::Status}, helpers::{status::*, date::{Local, Duration, self}}};

use wasm_bindgen::prelude::*;
use yew::{function_component, html, Callback, Html, Properties};
use round::round;

use crate::helpers::status::get_status_class;

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
    pub bots: Vec<CompactBotData>,
    pub on_bot_click: Callback<String>,
}

#[function_component(BotList)]
pub fn bot_list(props: &Props) -> Html {
    let Props { bots, on_bot_click } = props;
    let base_url = get_base_url();
    let url = [base_url.replace("bots/", "api/bots/chart/").as_str()].concat();

    let mut total_profit = 0.;
    let mut total_profit_per = 0.;
    let mut total_profit_factor = 0.;
    let mut total_profitable_trades = 0.;
    let mut total_max_drawdown = 0.;
    let mut total_won_per_trade_per = 0.;
    let mut total_lost_per_trade_per = 0.;
    let mut total_trades = 0;
    let mut total_winning_trades = 0;
    let mut total_losing_trades = 0;
    let mut total_stop_losses = 0;

    let  updated_timeout = Local::now() - Duration::seconds(70);

    let instrument_list: Html = bots
        .iter()
        .map(|bot| {
            let on_bot_select = {
                let on_bot_click = on_bot_click.clone();
                let id = bot._id.clone();
                let url = [url.clone(),id.to_string()].concat();
                Callback::from(move |_| on_bot_click.emit(url.clone()))
            };

            let stats = bot.strategy_stats.clone(); 
            let num_trades = bot.strategy_stats.trades;

            let profit_status = get_profit_per_status(stats.net_profit_per);

            let (profit_factor_status, profitable_trades_status, max_drawdown_status) = match num_trades.cmp(&0) {
                std::cmp::Ordering::Greater => (
                    match stats.profitable_trades {
                         x if x == 100. => Status::Neutral,
                         _  => get_profit_factor_status(stats.profit_factor)
                    },
                    get_profitable_trades_status(stats.profitable_trades),
                    get_max_drawdown_status(stats.max_drawdown)
                ),
                _ => (Status::Neutral, Status::Neutral,Status::Neutral),
            };

            let avg_won_lost_status = match stats.won_per_trade_per {
                _ if stats.won_per_trade_per > (stats.lost_per_trade_per * -1.) => Status::Bullish,
                _ if stats.won_per_trade_per < (stats.lost_per_trade_per * -1.) => Status::Bearish,
                _  => Status::Neutral
            };

            let higher_time_frame = match &bot.higher_time_frame{
                Some(htf) => htf.to_string(),
                None => "".to_string()
            };

            let last_updated = date::from_dbtime(&bot.last_update);

            let updated_status = match last_updated.cmp(&updated_timeout){
                    std::cmp::Ordering::Less => Status::Bearish,
                    _ => Status::Default,
            };

            total_profit += bot.strategy_stats.net_profit;
            total_profit_per += bot.strategy_stats.net_profit_per;
            total_profit_factor += bot.strategy_stats.profit_factor;
            total_profitable_trades += bot.strategy_stats.profitable_trades;
            total_max_drawdown += bot.strategy_stats.max_drawdown;
            total_won_per_trade_per += bot.strategy_stats.won_per_trade_per;
            total_lost_per_trade_per += bot.strategy_stats.lost_per_trade_per;
            total_trades += num_trades;
            total_winning_trades += bot.strategy_stats.wining_trades;
            total_losing_trades += bot.strategy_stats.losing_trades;
            total_stop_losses += bot.strategy_stats.stop_losses;

      

            html! {
                <tr  onclick={ on_bot_select }>
                    <td> { bot.symbol.clone() } </td>
                    <td> { bot.strategy_name.clone() } </td>
                    <td> { bot.strategy_type.clone() } </td>
                    <td>{ format!(" {} / {} ", bot.time_frame.clone(), higher_time_frame)}</td>
                    <td class={get_status_class(&profit_status)}>  { format!("{} €", round(bot.strategy_stats.net_profit,2)) } </td>
                    <td class={get_status_class(&profit_status)}> { format!("{}%", round(bot.strategy_stats.net_profit_per,2) ) }</td>
                    <td class={get_status_class(&profit_factor_status)}>  { round(bot.strategy_stats.profit_factor,2) } </td>
                    <td class={get_status_class(&profitable_trades_status)}> { format!("{}%", round(bot.strategy_stats.profitable_trades,2))}</td>
                    <td class={get_status_class(&max_drawdown_status)}>  { format!("{}%", round(bot.strategy_stats.max_drawdown,2) ) } </td>
                    <td class={get_status_class(&avg_won_lost_status)}>{ format!("{}%", round(bot.strategy_stats.won_per_trade_per,2))}</td>
                    <td class={get_status_class(&avg_won_lost_status)}>{ format!("{}%", round(bot.strategy_stats.lost_per_trade_per,2))}</td>
                    <td>{ num_trades }</td>
                    <td> {format!("{} / {} / {}", bot.strategy_stats.wining_trades, bot.strategy_stats.losing_trades, bot.strategy_stats.stop_losses )}</td>
                    <td class={get_status_class(&updated_status)}> {format!("{}", bot.last_update.to_chrono().format("%H:%M:%S"))}</td>
                </tr>
            }
        })
        .collect();

    let total_profit_status = get_profit_per_status(total_profit);
    let total_profit_per_status = get_profit_per_status(total_profit_per);
    let total_profit_factor_status = get_profit_factor_status(total_profit_factor);
    let total_profitable_trades_status = get_profitable_trades_status(total_profitable_trades);
    let total_max_drawdown_status = get_max_drawdown_status(total_max_drawdown);

    let table = html! {
        <table class="table is-bordered">
            <thead class="has-background-grey-lighter">
                <tr>
                <th>{ "Symbol" }</th>
                <th>{ "Strategy" }</th>
                <th>{ "Type" }</th>
                <th>{ "T.Frame" }</th>
                <th>{ "Profit" }</th>
                <th>{ "% Profit" }</th>
                <th>{ "P. Factor" }</th>
                <th>{ "WinRate" }</th>
                <th>{ "Drawdown" }</th>
                <th>{ "Avg Won" }</th>
                <th>{ "Avg Lost" }</th>
                <th>{ "Trades" }</th>
                <th>{ "Wo. / Lo. / St" }</th>
                <th>{ "Updated" }</th>
                </tr>
            </thead>
            <tbody>
                { instrument_list }
                <tr>
                <td>{ "TOTAL" }</td>
                <td></td>
                <td></td>
                <td></td>
                <td class={get_status_class(&total_profit_status)}>{ format!("{} €", round(total_profit,2)) }</td>
                <td class={get_status_class(&total_profit_per_status)}>{ format!("{} %", round(total_profit_per,2)) }</td>
                <td class={get_status_class(&total_profit_factor_status)}>{ round(total_profit_factor / bots.len() as f64, 2)}</td>
                <td class={get_status_class(&total_profitable_trades_status)}>{ format!("{} %", round(total_profitable_trades / bots.len() as f64, 2)) }</td>
                <td class={get_status_class(&total_max_drawdown_status)}>{ format!("{} %", round(total_max_drawdown, 2)) }</td>
                <td>{ round(total_won_per_trade_per / bots.len() as f64, 2)}</td>
                <td>{ round(total_lost_per_trade_per / bots.len() as f64, 2)}</td>
                <td>{ total_trades }</td>
                <td> {format!("{} / {} / {}", total_winning_trades, total_losing_trades, total_stop_losses )}</td>
                <td></td>
                </tr>
            </tbody>
        </table>
    };

    table
}
