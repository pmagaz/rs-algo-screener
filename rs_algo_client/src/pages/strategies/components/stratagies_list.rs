use round::round;
use rs_algo_shared::models::backtest_strategy::BackTestStrategyResult;
use rs_algo_shared::models::status::Status;
use wasm_bindgen::prelude::*;
use yew::{function_component, html, Callback, Html, Properties};

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
    pub strategies: Vec<BackTestStrategyResult>,
    pub on_strategy_click: Callback<String>,
}

#[function_component(StrategiesList)]
pub fn strategy_list(props: &Props) -> Html {
    let Props {
        strategies,
        on_strategy_click,
    } = props;
    let base_url = get_base_url();
    let url = [base_url.as_str(), "api/strategies/chart/"].concat();

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

    let strategy_list: Html = strategies
        .iter()
        .map(|strategy| {

            let on_strategy_click = {
                let on_strategy_click = on_strategy_click.clone();
                let strategy = strategy.strategy.clone();
                let url = [url.clone(),strategy].concat();
                Callback::from(move |_| on_strategy_click.emit(url.clone()))
            };

            let profit_factor = strategy.avg_profit_factor;
            let profit_factor_status = match profit_factor {
                _x if profit_factor <= 1.0 => Status::Bearish,
                _x if profit_factor > 1.0 && profit_factor < 1.8 => Status::Neutral,
                _x if profit_factor > 1.8 => Status::Bullish,
                _ => Status::Neutral,
            };


            let profitable_trades = strategy.avg_profitable_trades;
            let profitable_trades_status = match profitable_trades {
                _x if profitable_trades <= 30. => Status::Bearish,
                _x if profitable_trades > 30. && profitable_trades < 40. => Status::Neutral,
                _x if profitable_trades > 40. => Status::Bullish,
                _ => Status::Neutral,
            };

            let profit = strategy.avg_net_profit_per;
            let profit_status = match profit {
                _x if profit <= 10. => Status::Bearish,
                _x if profit > 10. && profitable_trades < 12. => Status::Neutral,
                _x if profit >= 15. => Status::Bullish,
                _ => Status::Neutral,
            };

            html! {
                <tr>
                    <td  onclick={ on_strategy_click }><a href={format!("javascript:void(0);")}>{format!("{}", strategy.strategy)}</a></td>
                    <td class={get_status_class(&profit_factor_status)}> { round(strategy.avg_profit_factor,2)}</td>
                    <td class={get_status_class(&profitable_trades_status)}> { format!("{}%", round(strategy.avg_profitable_trades,2))}</td>
                    <td class={get_status_class(&profit_status)}> { format!("{}%", round(strategy.avg_net_profit_per,2))}</td>
                    <td>{ format!("{}%", round(strategy.avg_max_drawdown,2))}</td>
                    <td>{ strategy.avg_trades}</td>
                    <td>{ strategy.avg_stop_losses}</td>
                    <td>{ round(strategy.avg_annual_return,2)}</td>
                    <td>{ round(strategy.avg_buy_hold,2)}</td>
                </tr>
            }
        })
        .collect();

    let table = html! {
        <table class="table is-bordered">
            <thead class="has-background-grey-lighter">
                <tr>
                <th><abbr>{ "Strategy" }</abbr></th>
                <th><abbr>{ "Avg. Profit Factor" }</abbr></th>
                <th><abbr>{ "Avg. Profitable trades" }</abbr></th>
                <th><abbr>{ "Avg. Profit" }</abbr></th>
                <th><abbr>{ "Avg. Max Drawdown" }</abbr></th>
                <th><abbr>{ "Avg. Num trades" }</abbr></th>
                <th><abbr>{ "Avg. Stop losses" }</abbr></th>
                <th><abbr>{ "Avg. Anual return" }</abbr></th>
                <th><abbr>{ "Buy & Hold" }</abbr></th>
                </tr>
            </thead>
            <tbody>
                { strategy_list }
            </tbody>
        </table>
    };

    table
}
