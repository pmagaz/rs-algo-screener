use super::api;

use round::round;
use rs_algo_shared::models::CompactInstrument;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlElement, HtmlInputElement, MouseEvent};

use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html, Properties};
#[function_component(Home)]
pub fn home() -> Html {
    let url = "http://192.168.1.10/api/instruments";
    //let url = "http://localhost:8000/api/instruments";
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
                    instruments.set(api::get_instruments(url, data.to_owned()).await.unwrap());
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
                    let value = window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id("query_box")
                        .unwrap()
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .unwrap()
                        .value();
                    // .dyn_into::<web_sys::HtmlInputElement>()
                    // .unwrap()
                    // .inner_text();
                    // .dyn_into::<HtmlInputElement>()
                    // .unwrap()
                    // .value();
                    //.inner_html();
                    // .text_content();
                    // .inner_html();
                    //.dyn_into::<HtmlInputElement>()
                    let data = r#"{"current_candle": "Karakasa"}"#;
                    log::info!("[CLIENT] query {:?}", value);
                    instruments.set(api::get_instruments(url, data.to_owned()).await.unwrap());
                });
            })
        }
    };

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
                        <form name="form">
                            <textarea id="query_box" class="textarea is-primary" placeholder="Textarea" cols="50" value="hola"></textarea>
                            <button id="leches" class="button" onclick={on_query_send}>{ "Search" }</button>
                        </form>

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
                    <th><abbr>{ "P. Target" }</abbr></th>
                    <th><abbr>{ "MacD" }</abbr></th>
                    <th><abbr>{ "Stoch" }</abbr></th>
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

#[function_component(InstrumentsList)]
fn instrument_list(
    InstrumentsProps {
        instruments,
        // on_click,
    }: &InstrumentsProps,
) -> Html {
    //let on_click = on_click.clone();
    let base_url = "http://192.168.1.10/api/instruments?symbol=";

    instruments
        .iter()
        .map(|instrument| {
            let on_instrument_select = {
                //let on_click = on_click.clone();
                let instrument = instrument.clone();
                //Callback::from(move |_| on_click.emit(instrument.clone()))
            };
            html! {
                <tr>
                    <td> <a href={format!("{}{}", base_url, instrument.symbol)}>{format!("{}", instrument.symbol)}</a></td>
                    <td> {format!("{}", round(instrument.current_price,2))}</td>
                    <td> {format!("{:?}", instrument.current_candle)}</td>
                    <td> {format!("{:?}", instrument.patterns.local_patterns[0].pattern_type)}</td>
                    <td> {format!("{:?}%", instrument.patterns.local_patterns[0].active.change.ceil())}</td>
                    <td> {format!("{:?} / {:?}", round(instrument.indicators.macd.current_a, 2), round(instrument.indicators.macd.current_b, 2))}</td>
                    <td> {format!("{:?} / {:?}", round(instrument.indicators.stoch.current_a, 2), round(instrument.indicators.stoch.current_b, 2))}</td>
                    <td> {format!("{}", instrument.updated)}</td>
                </tr>
            }
        })
        .collect()
}
