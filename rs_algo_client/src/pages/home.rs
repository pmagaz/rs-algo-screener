use super::api;
use dotenv::dotenv;

use rs_algo_shared::models::CompactInstrument;
use std::env;
use web_sys::{window, Url};
use yew::utils;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html, Properties};
#[function_component(Home)]
pub fn home() -> Html {
    dotenv().ok();

    let instruments = use_state(|| vec![]);
    {
        //let is_loading = props.is_loading.clone();
        let instruments = instruments.clone();
        use_effect_with_deps(
            move |_| {
                log::info!("[CLIENT] API call...");
                wasm_bindgen_futures::spawn_local(async move {
                    let data = r#"{"current_candle": "Karakasa"}"#;
                    // let query = r#"{"symbol":"patterns.local_patterns": { "$elemMatch" : { "active.active": false} }}"#;
                    let res = api::get_instruments(data.to_owned()).await.unwrap();
                    let fetched_instruments: Vec<CompactInstrument> = res.json().await.unwrap();
                    instruments.set(fetched_instruments);
                });
                || ()
            },
            (),
        );
    }

    //let selected_instrument = use_state(|| None);
    let on_instrument_click = {
        //  let selected_instrument = selected_instrument.clone();
        Callback::from(move |instrument: CompactInstrument| {
            log::info!("[CLIENT] Instrument selected {:}", instrument.symbol);
            let url = Url::new(
                &[
                    "http://192.168.1.10/api/instruments?symbol=",
                    &instrument.symbol,
                ]
                .concat(),
            )
            .unwrap();
            log::info!("[CLIENT] API call... {:?}", url);
            let location = window().unwrap().location().assign(
                &[
                    "http://192.168.1.10/api/instruments?symbol=",
                    &instrument.symbol,
                ]
                .concat(),
            );
        })
    };

    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <h1 class="title is-1">{ "Instruments" }</h1>
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
                    <th><abbr>{ "Updated" }</abbr></th>
                    </tr>
                </thead>
                 <tbody>
                          <InstrumentsList instruments={(*instruments).clone()} on_click={on_instrument_click.clone()} />
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
    on_click: Callback<CompactInstrument>,
}

#[function_component(InstrumentsList)]
fn instrument_list(
    InstrumentsProps {
        instruments,
        on_click,
    }: &InstrumentsProps,
) -> Html {
    let on_click = on_click.clone();

    instruments
        .iter()
        .map(|instrument| {
            let on_instrument_select = {
                let on_click = on_click.clone();
                let instrument = instrument.clone();
                Callback::from(move |_| on_click.emit(instrument.clone()))
            };

            html! {
                <tr onclick={on_instrument_select}>
                    <td> {format!("{}", instrument.symbol)}</td>
                    <td> {format!("{}", instrument.current_price)}</td>
                    <td> {format!("{:?}", instrument.current_candle)}</td>
                    <td> {format!("{}", instrument.updated)}</td>
                </tr>
            }
        })
        .collect()
}
