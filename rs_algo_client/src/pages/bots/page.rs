use super::api;
use super::components::bot_list::BotList;
use crate::components::chart::Chart;
use crate::components::loading::Loading;

use rs_algo_shared::helpers::date::{Local, Timelike};
use rs_algo_shared::models::bot::CompactBotData;

use gloo::timers::callback::Interval;
use wasm_bindgen::prelude::*;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Properties};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = get_query_value)]
    fn get_query_value() -> String;
    #[wasm_bindgen(js_name = get_base_url)]
    fn get_base_url() -> String;
    fn open_modal();

}

#[function_component(Bots)]
pub fn bots() -> Html {
    let base_url = get_base_url();
    let bots_url = [base_url.replace("bots/", "api/bots").as_str()].concat();
    let use_bots = use_state(|| vec![]);
    let use_bot = use_state(|| String::from(""));
    let use_loading = use_state(|| true);
    let use_chart_url = use_state(|| String::from(""));
    let polling_seconds_list = [0, 5, 30];
    let polling_seconds_chart = [0, 5, 10, 20, 30, 40, 50];
    let interval_task = use_state(|| None);

    {
        let use_bots = use_bots.clone();
        let use_loading = use_loading.clone();
        let bots_url = bots_url.clone();

        use_effect_with_deps(
            move |_| {
                log::info!("[CLIENT] API call...");
                let use_loading = use_loading.clone();
                wasm_bindgen_futures::spawn_local({
                    let use_bots = use_bots.clone();
                    async move {
                        let bots_url = bots_url.clone();
                        let use_bots = use_bots.clone();
                        let query = get_query_value();
                        let strategies = api::get_bots(&bots_url, query).await.unwrap();
                        use_bots.set(strategies);
                        use_loading.set(false);
                    }
                });

                Interval::new(5000, move || {
                    let date = Local::now();
                    let seconds = date.second();
                    if seconds >= 1 && seconds <= 7 {
                        // if polling_seconds_list.contains(&seconds)
                        //     || polling_seconds_list
                        //         .iter()
                        //         .any(|&n| seconds >= n + 1 && seconds <= n + 5)
                        // {
                        log::info!("[CLIENT] Polling API... {}", seconds);

                        wasm_bindgen_futures::spawn_local({
                            let use_bots = use_bots.clone();
                            async move {
                                let bots = api::get_bots2().await.unwrap();
                                use_bots.set(bots);
                            }
                        });
                    }
                })
                .forget();

                || ()
            },
            (),
        );
    }

    let on_bot_click = {
        let interval_task = interval_task.clone();
        let use_chart_url = use_chart_url.clone();
        Callback::from(move |chart_url: String| {
            log::info!("[CLIENT] Selecting {}", &chart_url);
            let use_chart_url = use_chart_url.clone();
            use_chart_url.set(chart_url.clone());

            let new_task = Interval::new(5000, move || {
                let date = Local::now();
                let seconds = date.second();
                let chart_url = chart_url.clone();
                let use_chart_url = use_chart_url.clone();
                //if seconds >= 1 && seconds <= 7 {
                //if polling_seconds_chart.contains(&seconds) {
                log::info!("[CLIENT] Polling chart... {}", seconds);

                wasm_bindgen_futures::spawn_local({
                    async move {
                        let date = Local::now();
                        let url = [&chart_url, "?ts=", &date.timestamp().to_string()].concat();
                        use_chart_url.set(url.clone());
                        // let bots = api::get_bots2().await.unwrap();
                        // use_bots.set(bots);
                    }
                });
                // }
            });
            //.forget();
            interval_task.set(Some(new_task));

            open_modal();
        })
    };

    let prod_bots: Vec<CompactBotData> = use_bots
        .iter()
        .filter(|x| x.env.is_prod())
        .map(|x| x.clone())
        .collect();

    let dev_bots: Vec<CompactBotData> = use_bots
        .iter()
        .filter(|x| x.env.is_dev())
        .map(|x| x.clone())
        .collect();

    let backtest_bots: Vec<CompactBotData> = use_bots
        .iter()
        .filter(|x| x.env.is_backtest())
        .map(|x| x.clone())
        .collect();

    html! {
        <div class="tile is-ancestor is-vertical ">
            <div class="section is-child hero">
                <div class="hero-body container pb-0">
                        //<label class="label">{ "Query" }</label>
                     <h1 class="navbar-item is-size-2">{ "Bots" }</h1>
                        //<textarea id="query_box" class="textarea is-link is-invisible" placeholder="Textarea" cols="60" rows="0" value={ {format!("{}", *use_query)}}></textarea>
                        // <button id="leches" class="button" onclick={on_query_send}>{ "Search" }</button>
                        //<div width="400" height="400"></div>
                        <Loading loading={ *use_loading } />
                </div>
            </div>
            <Chart url={(*use_chart_url).clone()}/>
           <div class="container">
                <div class="notification is-fluid ">
                    <h3 class="navbar-item is-size-2">{ "Prod" }</h3>
                    <BotList on_bot_click={ on_bot_click.clone()}  bots={(prod_bots).clone()} />
                    <h3 class="navbar-item is-size-2">{ "Dev" }</h3>
                    <BotList on_bot_click={ on_bot_click.clone()}  bots={(dev_bots).clone()} />
                    <h3 class="navbar-item is-size-2">{ "Backtest" }</h3>
                    <BotList on_bot_click={ on_bot_click.clone()}  bots={(backtest_bots).clone()} />

            </div>
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct InstrumentsProps {
    use_bots: Vec<CompactBotData>,
}
