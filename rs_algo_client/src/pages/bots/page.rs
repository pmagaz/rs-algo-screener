use super::api;
use super::components::bot_list::BotList;
use crate::components::chart::Chart;
use crate::components::loading::Loading;

use rs_algo_shared::helpers::date::{DateTime, Local, Timelike};
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

                Interval::new(2_500, move || {
                    let date = Local::now();
                    let seconds = date.second();
                    if seconds >= 1 && seconds <= 5 {
                        wasm_bindgen_futures::spawn_local({
                            let use_bots = use_bots.clone();
                            async move {
                                log::info!("[CLIENT] Polling call...");
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
        let use_chart_url = use_chart_url.clone();
        Callback::from(move |chart_url: String| {
            log::info!("[CLIENT] Selecting {}", &chart_url);
            let use_chart_url = use_chart_url.clone();
            use_chart_url.set(chart_url.clone());

            Interval::new(2_500, move || {
                let date = Local::now();
                let seconds = date.second();
                let chart_url = chart_url.clone();
                let use_chart_url = use_chart_url.clone();

                if seconds >= 1 && seconds <= 5 {
                    wasm_bindgen_futures::spawn_local({
                        async move {
                            let date = Local::now();
                            let url = [&chart_url, "?ts=", &date.timestamp().to_string()].concat();
                            use_chart_url.set(url.clone());
                            log::info!("[CLIENT] Polling call...");
                            // let bots = api::get_bots2().await.unwrap();
                            // use_bots.set(bots);
                        }
                    });
                }
            })
            .forget();

            open_modal();
        })
    };

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
                    <BotList on_bot_click={ on_bot_click.clone()}  bots={(*use_bots).clone()} />

            </div>
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct InstrumentsProps {
    use_bots: Vec<CompactBotData>,
}
