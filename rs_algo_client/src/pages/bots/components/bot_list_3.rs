use std::collections::HashMap;

use rs_algo_shared::{
    helpers::{
        date::{self, Duration, Local},
        status::*,
    },
    models::{bot::CompactBotData, status::Status},
};

use round::round;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use yew::{function_component, html, virtual_dom::VNode, Callback, Html, Properties};

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
    let url = base_url.replace("bots/", "api/bots/chart/");

    let updated_timeout = Local::now() - Duration::seconds(70);
    let mut strategy_totals: HashMap<
        String,
        (f64, f64, f64, f64, f64, f64, f64, f64, usize, usize, usize),
    > = HashMap::new();
    let mut last_strategy = String::new();
    let mut bot_list: Vec<VNode> = vec![];

    for bot in bots.iter() {
        let on_bot_select = {
            let on_bot_click = on_bot_click.clone();
            let bot_url = format!("{}{}", url, bot._id);
            Callback::from(move |_| on_bot_click.emit(bot_url.clone()))
        };

        let stats = &bot.strategy_stats;
        let num_trades = stats.trades;
        let profit_status = get_profit_per_status(stats.net_profit_per);

        let (profit_factor_status, profitable_trades_status, max_drawdown_status) =
            match num_trades.cmp(&0) {
                std::cmp::Ordering::Greater => (
                    get_profit_factor_status(stats.profit_factor),
                    get_profitable_trades_status(stats.profitable_trades),
                    get_max_drawdown_status(stats.max_drawdown),
                ),
                _ => (Status::Neutral, Status::Neutral, Status::Neutral),
            };

        let avg_won_lost_status = if stats.won_per_trade_per > (stats.lost_per_trade_per * -1.) {
            Status::Bullish
        } else if stats.won_per_trade_per < (stats.lost_per_trade_per * -1.) {
            Status::Bearish
        } else {
            Status::Neutral
        };

        let higher_time_frame = match &bot.higher_time_frame {
            Some(htf) => htf.to_string(),
            None => "".to_string(),
        };

        let last_updated = date::from_dbtime(&bot.last_update);
        let updated_status = if last_updated < updated_timeout {
            Status::Bearish
        } else {
            Status::Default
        };

        let strategy_entry = strategy_totals
            .entry(bot.strategy_name.clone())
            .or_default();

        // Updating strategy totals
        *strategy_entry = (
            strategy_entry.0 + stats.net_profit,
            strategy_entry.1 + stats.net_profit_per,
            strategy_entry.2 + stats.profit_factor,
            strategy_entry.3 + stats.trades as f64,
            strategy_entry.4 + stats.profitable_trades,
            strategy_entry.5 + stats.max_drawdown,
            strategy_entry.6 + stats.won_per_trade_per,
            strategy_entry.7 + stats.lost_per_trade_per,
            strategy_entry.8 + stats.wining_trades,
            strategy_entry.9 + stats.losing_trades,
            strategy_entry.10 + stats.stop_losses,
        );

        // Handling strategy change
        if last_strategy != bot.strategy_name {
            if !last_strategy.is_empty() {
                let num_strategy_items = bots
                    .iter()
                    .filter(|item| item.strategy_name == last_strategy)
                    .count() as f64;

                let subtotal_totals =
                    calculate_totals(&strategy_totals[&last_strategy], num_strategy_items);

                let total_row = create_total_row(subtotal_totals);
                bot_list.push(total_row);
            }
            last_strategy = bot.strategy_name.clone();
        }

        // Pushing bot data
        bot_list.push(create_bot_row(
            bot,
            &on_bot_select,
            &higher_time_frame,
            num_trades,
            &profit_status,
            &profit_factor_status,
            &profitable_trades_status,
            &max_drawdown_status,
            &avg_won_lost_status,
            &updated_status,
        ));
    }

    // Handling last strategy
    if !last_strategy.is_empty() {
        let num_strategy_items = bots
            .iter()
            .filter(|item| item.strategy_name == last_strategy)
            .count() as f64;

        let subtotal_totals =
            calculate_totals(&strategy_totals[&last_strategy], num_strategy_items);

        let total_row = create_total_row(subtotal_totals);
        bot_list.push(total_row);
    }

    // Creating final table
    html! {
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
                { bot_list }
            </tbody>
        </table>
    }
}

fn create_total_row(
    subtotal_totals: (f64, f64, f64, f64, f64, f64, f64, f64, usize, usize, usize),
) -> VNode {
    let profit_status = get_profit_per_status(subtotal_totals.1);
    let profit_factor_status = get_profit_factor_status(subtotal_totals.2);
    let profitable_trades_status = get_profitable_trades_status(subtotal_totals.4);
    let max_drawdown_status = get_max_drawdown_status(subtotal_totals.5);

    html! {
        <tr>
            <td>{ "TOTAL" }</td>
            <td></td>
            <td></td>
            <td></td>
            <td class={get_status_class(&profit_status)}>{ format!("{} €", round(subtotal_totals.0, 2)) }</td>
            <td class={get_status_class(&profit_status)}>{ format!("{}%", round(subtotal_totals.1, 2)) }</td>
            <td class={get_status_class(&profit_factor_status)}>{ round(subtotal_totals.2, 2) }</td>
            <td class={get_status_class(&profitable_trades_status)}>{ format!("{} %", round(subtotal_totals.4, 2)) }</td>
            <td class={get_status_class(&max_drawdown_status)}>{ format!("{}%", round(subtotal_totals.5, 2)) }</td>
            <td>{ format!("{}%", round(subtotal_totals.6, 2)) }</td>
            <td>{ format!("{}%", round(subtotal_totals.7, 2)) }</td>
            <td>{ subtotal_totals.3 }</td>
            <td>{ format!("{} / {} / {}", subtotal_totals.8, subtotal_totals.9, subtotal_totals.10) }</td>
            <td></td>
        </tr>
    }
}

fn calculate_totals(
    totals: &(f64, f64, f64, f64, f64, f64, f64, f64, usize, usize, usize),
    num_items: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64, usize, usize, usize) {
    (
        totals.0,             // subtotal_profit
        totals.1,             // subtotal_profit_per
        totals.2 / num_items, // average profit_factor
        totals.3,             // total trades
        totals.4 / num_items, // average profitable_trades
        totals.5,             // total max_drawdown
        totals.6 / num_items, // average won_per_trade_per
        totals.7 / num_items, // average lost_per_trade_per
        totals.8,             // total winning_trades
        totals.9,             // total losing_trades
        totals.10,            // total stop_losses
    )
}

fn create_bot_row(
    bot: &CompactBotData, // assuming Bot is the struct type for each bot
    on_bot_select: &Callback<MouseEvent>,
    higher_time_frame: &str,
    num_trades: usize,
    profit_status: &Status,
    profit_factor_status: &Status,
    profitable_trades_status: &Status,
    max_drawdown_status: &Status,
    avg_won_lost_status: &Status,
    updated_status: &Status,
) -> VNode {
    html! {
        <tr style="opacity: 0.5;" onclick={ on_bot_select }>
            <td> { bot.symbol.clone() } </td>
            <td> { bot.strategy_name.clone() } </td>
            <td> { bot.strategy_type.clone() } </td>
            <td>{ format!(" {} / {} ", bot.time_frame.clone(), higher_time_frame)}</td>
            <td class={get_status_class(profit_status)}>  { format!("{} €", round(bot.strategy_stats.net_profit, 2)) } </td>
            <td class={get_status_class(profit_status)}> { format!("{}%", round(bot.strategy_stats.net_profit_per, 2) ) }</td>
            <td class={get_status_class(profit_factor_status)}>  { round(bot.strategy_stats.profit_factor, 2) } </td>
            <td class={get_status_class(profitable_trades_status)}> { format!("{}%", round(bot.strategy_stats.profitable_trades, 2))}</td>
            <td class={get_status_class(max_drawdown_status)}>  { format!("{}%", round(bot.strategy_stats.max_drawdown, 2) ) } </td>
            <td class={get_status_class(avg_won_lost_status)}>{ format!("{}%", round(bot.strategy_stats.won_per_trade_per, 2))}</td>
            <td class={get_status_class(avg_won_lost_status)}>{ format!("{}%", round(bot.strategy_stats.lost_per_trade_per, 2))}</td>
            <td>{ num_trades }</td>
            <td> {format!("{} / {} / {}", bot.strategy_stats.wining_trades, bot.strategy_stats.losing_trades, bot.strategy_stats.stop_losses )}</td>
            <td class={get_status_class(updated_status)}> {format!("{}", bot.last_update.to_chrono().format("%H:%M:%S"))}</td>
        </tr>
    }
}

fn create_total_row2(
    subtotal_profit: f64,
    subtotal_profit_per: f64,
    subtotal_profit_factor: f64,
    subtotal_trades: f64,
    subtotal_profitable_trades: f64,
    subtotal_max_drawdown: f64,
    subtotal_won_per_trade_per: f64,
    subtotal_lost_per_trade_per: f64,
    subtotal_winning_trades: usize,
    subtotal_losing_trades: usize,
    subtotal_stop_losses: usize,
) -> VNode {
    let profit_status = get_profit_per_status(subtotal_profit);
    let total_profit_per_status = get_profit_per_status(subtotal_profit_per);
    let profit_factor_status = get_profit_factor_status(subtotal_profit_factor);
    let profitable_trades_status = get_profitable_trades_status(subtotal_profitable_trades);
    let max_drawdown_status = get_max_drawdown_status(subtotal_max_drawdown);

    html! {
        <tr>
            <td>{ "TOTAL" }</td>
            <td></td>
            <td></td>
            <td></td>
            <td class={get_status_class(&profit_status)}>{ format!("{} €", round(subtotal_profit, 2)) }</td>
            <td class={get_status_class(&total_profit_per_status)}>{ format!("{}%", round(subtotal_profit_per, 2)) }</td>
            <td class={get_status_class(&profit_factor_status)}>{ round(subtotal_profit_factor, 2) }</td>
            <td class={get_status_class(&profitable_trades_status)}>{ format!("{} %", round(subtotal_profitable_trades, 2)) }</td>
            <td class={get_status_class(&max_drawdown_status)}>{ format!("{}%", round(subtotal_max_drawdown, 2)) }</td>
            <td>{ format!("{}%", round(subtotal_won_per_trade_per, 2)) }</td>
            <td>{ format!("{}%", round(subtotal_lost_per_trade_per, 2)) }</td>
            <td>{ subtotal_trades }</td>
            <td>{ format!("{} / {} / {}", subtotal_winning_trades, subtotal_losing_trades, subtotal_stop_losses) }</td>
            <td></td>
        </tr>
    }
}
