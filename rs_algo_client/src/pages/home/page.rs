use super::api;
use crate::pages::home::components::instruments_list::InstrumentsList;

use round::round;
use rs_algo_shared::models::{CompactInstrument, PatternType};
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element, HtmlElement, HtmlInputElement, MouseEvent, Window};
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html, Properties};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = get_query_value)]
    fn get_query_value() -> String;
}

#[function_component(Home)]
pub fn home() -> Html {
    let url = "http://192.168.1.10/api/instruments";
    let instruments = use_state(|| vec![]);
    {
        //let is_loading = props.is_loading.clone();

        let instruments = instruments.clone();
        use_effect_with_deps(
            move |_| {
                log::info!("[CLIENT] API call...");
                wasm_bindgen_futures::spawn_local(async move {
                    let query = get_query_value();
                    instruments.set(api::get_instruments(url, query).await.unwrap());
                });
                || ()
            },
            (),
        );
    }

    //let selected_instrument = use_state(|| None);
    let on_query_send = {
        {
            let instruments = instruments.clone();
            Callback::from(move |event: MouseEvent| {
                let instruments = instruments.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let query = get_query_value();
                    log::info!("[CLIENT] query {:?}", query);
                    instruments.set(api::get_instruments(url, query).await.unwrap());
                });
            })
        }
    };

    let default_query = "";

    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <h1 class="title is-1">{ "Instruments" }</h1>
                </div>
            </div>

            <div class="section is-child hero">
                <div class="hero-body container pb-0">
                <div class="field">
                    <label class="label">{ "query" }</label>
                    <div class="control">
                        <textarea id="query_box" class="textarea is-primary" placeholder="Textarea" cols="50" value={format!("{}", default_query)}></textarea>
                        <button id="leches" class="button" onclick={on_query_send}>{ "Search" }</button>
                    </div>
                    </div>
                </div>
            </div>
           <div class="container ">
                <div class="notification is-fluid">
            <table class="table">
                <thead>
                    <tr>
                    <th><abbr>{ "Symbol" }</abbr></th>
                    <th><abbr>{ "Price" }</abbr></th>
                    <th><abbr>{ "Candle" }</abbr></th>
                    <th><abbr>{ "Pattern" }</abbr></th>
                    <th><abbr>{ "Target" }</abbr></th>
                    <th><abbr>{ "Direction" }</abbr></th>
                    <th><abbr>{ "Activated" }</abbr></th>
                    <th><abbr>{ "MacD" }</abbr></th>
                    <th><abbr>{ "Stoch" }</abbr></th>
                    <th><abbr>{ "Rsi" }</abbr></th>
                    <th><abbr>{ "Updated" }</abbr></th>
                    </tr>
                </thead>
                 <tbody>
                          <InstrumentsList instruments={(*instruments).clone()} />
                </tbody>
            </table>
            </div>
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct InstrumentsProps {
    instruments: Vec<CompactInstrument>,
    //on_click: Callback<CompactInstrument>,
}
