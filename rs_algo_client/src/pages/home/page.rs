use super::api;
use crate::components::loading::Loading;
use crate::pages::home::components::instruments_list::InstrumentsList;

use crate::components::plotter::Plotter;
use round::round;
use rs_algo_shared::models::CompactInstrument;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Properties};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = get_query_value)]
    fn get_query_value() -> String;
    fn open_modal();

}

#[function_component(Home)]
pub fn home() -> Html {
    let url = "http://192.168.1.10/api/instruments";
    let instruments = use_state(|| vec![]);
    let use_loading = use_state(|| true);
    let use_query = use_state(|| String::from(""));
    let use_url = use_state(|| String::from(""));

    {
        let instruments = instruments.clone();
        let use_loading = use_loading.clone();

        use_effect_with_deps(
            move |_| {
                log::info!("[CLIENT] API call...");
                let use_loading = use_loading.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let query = get_query_value();
                    instruments.set(api::get_instruments(url, query).await.unwrap());
                    use_loading.set(false);
                });
                || ()
            },
            (),
        );
    }

    let on_query_send = {
        let instruments = instruments.clone();
        let use_query = use_query.clone();
        let use_loading = use_loading.clone();

        Callback::from(move |_e: MouseEvent| {
            let instruments = instruments.clone();
            let use_query = use_query.clone();
            let use_loading = use_loading.clone();
            let query = get_query_value();
            use_query.set(query.clone());
            use_loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                instruments.set(api::get_instruments(url, query).await.unwrap());
                use_loading.set(false);
            });
        })
    };

    let on_symbol_click = {
        let use_url = use_url.clone();
        Callback::from(move |url: String| {
            log::info!("[CLIENT] Selecting {}", &url);
            let use_url = use_url.clone();
            use_url.set(url);
            open_modal();
        })
    };

    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="section is-child hero">
                <div class="hero-body container pb-0">
                <div class="field">
                     <Loading loading={ *use_loading} />
                    <label class="label">{ "Query" }</label>
                    <div class="control">
                        <textarea id="query_box" class="textarea is-link" placeholder="Textarea" cols="50" value={ {format!("{}", *use_query)}}></textarea>
                        <button id="leches" class="button" onclick={on_query_send}>{ "Search" }</button>
                    </div>
                    </div>
                </div>
            </div>
            <Plotter url={(*use_url).clone()}/>
           <div class="container">
                <div class="notification is-fluid">
            <table class="table is-bordered">
                <thead class="has-background-grey-lighter">
                    <tr>
                    <th><abbr>{ "Symbol" }</abbr></th>
                    <th><abbr>{ "Price" }</abbr></th>
                    <th><abbr>{ "Candle" }</abbr></th>
                    <th><abbr>{ "Pattern" }</abbr></th>
                    <th><abbr>{ "Direction" }</abbr></th>
                    <th><abbr>{ "Target" }</abbr></th>
                    <th><abbr>{ "Activated" }</abbr></th>
                    <th><abbr>{ "Stoch" }</abbr></th>
                    <th><abbr>{ "MacD" }</abbr></th>
                    <th><abbr>{ "Emas" }</abbr></th>
                    <th><abbr>{ "Rsi" }</abbr></th>
                    <th><abbr>{ "Updated" }</abbr></th>
                    </tr>
                </thead>
                 <tbody>
                          <InstrumentsList on_symbol_click={ on_symbol_click } instruments={(*instruments).clone()} />
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
