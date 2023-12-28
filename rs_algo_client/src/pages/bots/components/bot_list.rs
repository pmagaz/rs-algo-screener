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

    let updated_timeout = Local::now() - Duration::seconds(70);
    let mut strategy_totals: HashMap<
        String,
        (f64, f64, f64, f64, f64, f64, f64, f64, usize, usize, usize),
    > = HashMap::new();
    let mut last_strategy = String::new();
    let mut bot_list: Vec<VNode> = vec![];
    let mut strategy_order: Vec<(String, f64)> = vec![];
    let num_bots = bots.len();
    let mut bot_num = 1;

    for bot in bots.iter() {
        let on_bot_select = {
            let on_bot_click = on_bot_click.clone();
            let id = bot._id.clone();
            let bot_url = [url.clone(), id.to_string()].concat();
            Callback::from(move |_| on_bot_click.emit(bot_url.clone()))
        };

        let stats = bot.strategy_stats.clone();
        let num_trades = bot.strategy_stats.trades;
        let profit_status = get_profit_per_status(stats.net_profit_per);

        let (profit_factor_status, profitable_trades_status, max_drawdown_status) =
            match num_trades.cmp(&0) {
                std::cmp::Ordering::Greater => (
                    match stats.profitable_trades {
                        x if x == 100. => Status::Neutral,
                        _ => get_profit_factor_status(stats.profit_factor),
                    },
                    get_profitable_trades_status(stats.profitable_trades),
                    get_max_drawdown_status(stats.max_drawdown),
                ),
                _ => (Status::Neutral, Status::Neutral, Status::Neutral),
            };

        let avg_won_lost_status = match stats.won_per_trade_per {
            _ if stats.won_per_trade_per > (stats.lost_per_trade_per * -1.) => Status::Bullish,
            _ if stats.won_per_trade_per < (stats.lost_per_trade_per * -1.) => Status::Bearish,
            _ => Status::Neutral,
        };

        let higher_time_frame = match &bot.higher_time_frame {
            Some(htf) => htf.to_string(),
            None => "".to_string(),
        };

        let last_updated = date::from_dbtime(&bot.last_update);

        let updated_status = match last_updated.cmp(&updated_timeout) {
            std::cmp::Ordering::Less => Status::Bearish,
            _ => Status::Default,
        };

        let strategy_entry = strategy_totals
            .entry(bot.strategy_name.clone())
            .or_insert((0.0, 0.0, 0.0, 0., 0.0, 0.0, 0.0, 0., 0, 0, 0));

        strategy_entry.0 += bot.strategy_stats.net_profit;
        strategy_entry.1 += bot.strategy_stats.net_profit_per;
        strategy_entry.2 += bot.strategy_stats.profit_factor;
        strategy_entry.3 += bot.strategy_stats.trades as f64;
        strategy_entry.4 += bot.strategy_stats.profitable_trades;
        strategy_entry.5 += bot.strategy_stats.max_drawdown;
        strategy_entry.6 += bot.strategy_stats.won_per_trade_per;
        strategy_entry.7 += bot.strategy_stats.lost_per_trade_per;
        strategy_entry.8 += bot.strategy_stats.wining_trades;
        strategy_entry.9 += bot.strategy_stats.losing_trades;
        strategy_entry.10 += bot.strategy_stats.stop_losses;

        let num_strategy_items = bots
            .iter()
            .filter(|item| item.strategy_name == last_strategy)
            .count() as f64;

        if last_strategy != bot.strategy_name {
            if last_strategy != bot.strategy_name {
                if !last_strategy.is_empty() {
                    let subtotal_totals = calculate_totals(
                        &strategy_totals[&last_strategy],
                        num_strategy_items as f64,
                    );

                    if !bot.env.is_backtest() {
                        let total_row = create_total_row(
                            subtotal_totals.0,
                            subtotal_totals.1,
                            subtotal_totals.2,
                            subtotal_totals.3,
                            subtotal_totals.4,
                            subtotal_totals.5,
                            subtotal_totals.6,
                            subtotal_totals.7,
                            subtotal_totals.8,
                            subtotal_totals.9,
                            subtotal_totals.10,
                        );
                        bot_list.push(total_row);
                    }
                }
                last_strategy = bot.strategy_name.clone();
            }
        }

        bot_list.push(html! {
                    <tr style="opacity: 0.5;" onclick={ on_bot_select }>
                    <td> { bot.symbol.clone() } </td>
                    <td> { bot.strategy_name.clone() } </td>
                    <td> { bot.strategy_type.clone() } </td>
                    <td>{ format!(" {} / {} ", bot.time_frame.clone(), higher_time_frame)}</td>
                    <td class={get_status_class(&profit_status)}>  { format!("{} €", round(bot.strategy_stats.net_profit,3)) } </td>
                    <td class={get_status_class(&profit_status)}> { format!("{}%", round(bot.strategy_stats.net_profit_per,3) ) }</td>
                    <td class={get_status_class(&profit_factor_status)}>  { round(bot.strategy_stats.profit_factor,3) } </td>
                    <td class={get_status_class(&profitable_trades_status)}> { format!("{}%", round(bot.strategy_stats.profitable_trades,3))}</td>
                    <td class={get_status_class(&max_drawdown_status)}>  { format!("{}%", round(bot.strategy_stats.max_drawdown,3) ) } </td>
                    <td class={get_status_class(&avg_won_lost_status)}>{ format!("{}%", round(bot.strategy_stats.won_per_trade_per,3))}</td>
                    <td class={get_status_class(&avg_won_lost_status)}>{ format!("{}%", round(bot.strategy_stats.lost_per_trade_per,3))}</td>
                    <td>{ num_trades }</td>
                    <td> {format!("{} / {} / {}", bot.strategy_stats.wining_trades, bot.strategy_stats.losing_trades, bot.strategy_stats.stop_losses )}</td>
                    <td class={get_status_class(&updated_status)}> {format!("{}", bot.last_update.to_chrono().format("%H:%M:%S"))}</td>
                </tr>
            });

        //if !bot.strategy_name.contains("Back") {
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
        //  }

        if num_bots == bot_num {
            let subtotal_totals =
                calculate_totals(&strategy_totals[&last_strategy], num_strategy_items as f64);

            let total_row = create_total_row(
                subtotal_totals.0,
                subtotal_totals.1,
                subtotal_totals.2,
                subtotal_totals.3,
                subtotal_totals.4,
                subtotal_totals.5,
                subtotal_totals.6,
                subtotal_totals.7,
                subtotal_totals.8,
                subtotal_totals.9,
                subtotal_totals.10,
            );
            bot_list.push(total_row);
        }

        bot_num += 1;
    }

    //strategy_order.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let num_bots = bots.len();

    let total_profit_factor = total_profit_factor / num_bots as f64;
    let total_profitable_trades = total_profitable_trades / num_bots as f64;
    let total_won_per_trade_per = total_won_per_trade_per / num_bots as f64;
    let total_lost_per_trade_per = total_lost_per_trade_per / num_bots as f64;
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
                { bot_list }
                <tr>
                    <td>{ "TOTAL" }</td>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td class={get_status_class(&total_profit_status)}>{ format!("{} €", round(total_profit,3)) }</td>
                    <td class={get_status_class(&total_profit_per_status)}>{ format!("{} %", round(total_profit_per,3)) }</td>
                    <td class={get_status_class(&total_profit_factor_status)}>{ round(total_profit_factor , 3)}</td>
                    <td class={get_status_class(&total_profitable_trades_status)}>{ format!("{} %", round(total_profitable_trades, 3)) }</td>
                    <td class={get_status_class(&total_max_drawdown_status)}>{ format!("{} %", round(total_max_drawdown, 3)) }</td>
                    <td>{ round(total_won_per_trade_per, 3)}</td>
                    <td>{ round(total_lost_per_trade_per, 3)}</td>
                    <td>{ total_trades }</td>
                    <td> {format!("{} / {} / {}", total_winning_trades, total_losing_trades, total_stop_losses )}</td>
                    <td></td>
                </tr>
            </tbody>
        </table>
    };

    table
}

fn calculate_totals(
    totals: &(f64, f64, f64, f64, f64, f64, f64, f64, usize, usize, usize),
    num_items: f64,
) -> (f64, f64, f64, f64, f64, f64, f64, f64, usize, usize, usize) {
    (
        totals.0,             // subtotal_profit
        totals.1,             // subtotal_profit_per
        totals.2 / num_items, // subtotal_profit_factor
        totals.3,             // subtotal_trades
        totals.4 / num_items, // subtotal_profitable_trades
        totals.5,             // subtotal_max_drawdown
        totals.6 / num_items, // subtotal_won_per_trade_per
        totals.7 / num_items, // subtotal_lost_per_trade_per
        totals.8,             // subtotal_winning_trades
        totals.9,             // subtotal_losing_trades
        totals.10,            // subtotal_stop_losses
    )
}

fn create_total_row(
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
            <td class={get_status_class(&profit_status)}>{ format!("{} €", round(subtotal_profit, 3)) }</td>
            <td class={get_status_class(&total_profit_per_status)}>{ format!("{}%", round(subtotal_profit_per, 3)) }</td>
            <td class={get_status_class(&profit_factor_status)}>{ round(subtotal_profit_factor, 3) }</td>
            <td class={get_status_class(&profitable_trades_status)}>{ format!("{} %", round(subtotal_profitable_trades, 3)) }</td>
            <td class={get_status_class(&max_drawdown_status)}>{ format!("{}%", round(subtotal_max_drawdown, 3)) }</td>
            <td>{ format!("{}%", round(subtotal_won_per_trade_per, 3)) }</td>
            <td>{ format!("{}%", round(subtotal_lost_per_trade_per, 3)) }</td>
            <td>{ subtotal_trades }</td>
            <td>{ format!("{} / {} / {}", subtotal_winning_trades, subtotal_losing_trades, subtotal_stop_losses) }</td>
            <td></td>
        </tr>
    }
}
