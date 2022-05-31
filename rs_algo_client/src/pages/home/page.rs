use super::api;
use crate::components::loading::Loading;
use crate::pages::home::components::instruments_list::InstrumentsList;

use crate::components::chart::Chart;
use rs_algo_shared::models::instrument::*;
use rs_algo_shared::models::watch_instrument::*;
use wasm_bindgen::prelude::*;

use rs_algo_shared::helpers::date::*;

use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Properties};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = get_query_value)]
    fn get_query_value() -> String;
    #[wasm_bindgen(js_name = get_base_url)]
    fn get_base_url() -> String;
    fn open_modal();

}

#[function_component(Home)]
pub fn home() -> Html {
    let base_url = get_base_url();
    let instruments_url = [base_url.as_str(), "api/instruments"].concat();
    let watch_list_url = [base_url.as_str(), "api/watchlist"].concat();
    let portfolio_url = [base_url.as_str(), "api/portfolio"].concat();
    let use_instruments = use_state(|| vec![]);
    let use_watch_instruments = use_state(|| vec![]);
    let use_portfolio_instruments = use_state(|| vec![]);
    let use_loading = use_state(|| true);
    let use_instruments_url = use_state(|| String::from(""));

    {
        let use_instruments = use_instruments.clone();
        let use_watch_instruments = use_watch_instruments.clone();
        let use_portfolio_instruments = use_portfolio_instruments.clone();
        let use_loading = use_loading.clone();
        let instruments_url = instruments_url.clone();
        let watch_list_url = watch_list_url.clone();
        let portfolio_url = portfolio_url.clone();
        use_effect_with_deps(
            move |_| {
                log::info!("[CLIENT] API call...");
                let use_loading = use_loading.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let query = get_query_value();

                    use_portfolio_instruments.set(
                        api::get_portfolio_instruments(&portfolio_url)
                            .await
                            .unwrap(),
                    );

                    use_watch_instruments
                        .set(api::get_watch_instruments(&watch_list_url).await.unwrap());

                    use_instruments
                        .set(api::get_instruments(&instruments_url, query).await.unwrap());

                    use_loading.set(false);
                });
                || ()
            },
            (),
        );
    }

    // let on_query_send = {
    //     let use_instruments = use_instruments.clone();
    //     let use_query = use_query.clone();
    //     let use_loading = use_loading.clone();

    //     Callback::from(move |_e: MouseEvent| {
    //         let use_instruments = use_instruments.clone();
    //         let use_query = use_query.clone();
    //         let use_loading = use_loading.clone();
    //         let query = get_query_value();
    //         use_query.set(query.clone());
    //         use_loading.set(true);
    //         let instruments_url = instruments_url.clone();

    //         wasm_bindgen_futures::spawn_local(async move {
    //             use_instruments.set(api::get_instruments(&instruments_url, query).await.unwrap());
    //             use_loading.set(false);
    //         });
    //     })
    // };

    let on_symbol_click = {
        let use_instruments_url = use_instruments_url.clone();
        Callback::from(move |instruments_url: String| {
            log::info!("[CLIENT] Selecting {}", &instruments_url);
            let use_instruments_url = use_instruments_url.clone();
            use_instruments_url.set(instruments_url);
            open_modal();
        })
    };

    let on_watch_click = {
        let use_watch_instruments = use_watch_instruments.clone();
        let use_portfolio_instruments = use_portfolio_instruments.clone();
        let use_loading = use_loading.clone();

        Callback::from(move |inst: CompactInstrument| {
            let use_watch_instruments = use_watch_instruments.clone();
            let use_portfolio_instruments = use_portfolio_instruments.clone();
            let watch_list_url = watch_list_url.clone();
            let use_loading = use_loading.clone();
            use_loading.set(true);

            let fake_date = to_dbtime(Local::now() - Duration::days(1000));
            let watch_instrument = WatchInstrument {
                symbol: inst.symbol.clone(),
                alarm: Alarm {
                    active: false,
                    completed: false,
                    price: 0.0,
                    date: fake_date,
                    condition: AlarmCondition::None,
                },
            };

            wasm_bindgen_futures::spawn_local(async move {
                let watch_list_url = watch_list_url.clone();
                let _res = api::upsert_watch_instrument(&watch_list_url, watch_instrument)
                    .await
                    .unwrap();
                use_watch_instruments.set(vec![inst.clone()]);
                use_loading.set(false);
            });
        })
    };

    let suggested: Vec<CompactInstrument> = use_instruments
        .iter()
        .filter(|x| {
            (x.patterns.local_patterns.len() > 0
                && x.patterns.local_patterns.last().unwrap().date
                    > to_dbtime(Local::now() - Duration::days(4))
                && !x.patterns.local_patterns.last().unwrap().active.active)
        })
        .map(|x| x.clone())
        .collect();

    let activated: Vec<CompactInstrument> = use_instruments
        .iter()
        .filter(|x| {
            (x.patterns.local_patterns.len() > 0
                && x.patterns.local_patterns.last().unwrap().active.date
                    > to_dbtime(Local::now() - Duration::days(4)))
        })
        .map(|x| x.clone())
        .collect();

    let strategy: Vec<CompactInstrument> = use_instruments
        .iter()
        .filter(|x| {
            x.current_price <= x.indicators.bb.current_b && x.prev_price >= x.indicators.bb.prev_b
        })
        .map(|x| x.clone())
        .collect();

    let commodities: Vec<CompactInstrument> = use_instruments
        .iter()
        .filter(|x| {
            x.symbol == "US500"
                || x.symbol == "US100"
                || x.symbol == "GOLD"
                || x.symbol == "OIL"
                || x.symbol == "SILVER"
        })
        .map(|x| x.clone())
        .collect();

    let crypto: Vec<CompactInstrument> = use_instruments
        .iter()
        .filter(|x| {
            x.symbol == "BITCOIN"
                || x.symbol == "ETHEREUM"
                || x.symbol == "RIPPLE"
                || x.symbol == "DOGECOIN"
                || x.symbol == "POLKADOT"
                || x.symbol == "SOLANA"
        })
        .map(|x| x.clone())
        .collect();

    html! {
        <div class="tile is-ancestor is-vertical ">
            <div class="section is-child hero">
                <div class="hero-body container pb-0">
                        //<label class="label">{ "Query" }</label>
                     <h1 class="navbar-item is-size-2">{ "RS Screener" }</h1>

                        //<textarea id="query_box" class="textarea is-link is-invisible" placeholder="Textarea" cols="60" rows="0" value={ {format!("{}", *use_query)}}></textarea>
                        // <button id="leches" class="button" onclick={on_query_send}>{ "Search" }</button>
                        //<div width="400" height="400"></div>
                        <Loading loading={ *use_loading} />
                </div>
            </div>
            <Chart url={(*use_instruments_url).clone()}/>
           <div class="container">
                <div class="notification is-fluid ">
                    <h2 class="navbar-item is-size-3">{ "Portfolio" }</h2>
                    <InstrumentsList on_symbol_click={ on_symbol_click.clone()} on_watch_click={ on_watch_click.clone()} instruments={(*use_portfolio_instruments).clone()} />
                    <h2 class="navbar-item is-size-3">{ "Watch List" }</h2>
                    <InstrumentsList on_symbol_click={ on_symbol_click.clone()} on_watch_click={ on_watch_click.clone()} instruments={(*use_watch_instruments).clone()} />
                    <h2 class="navbar-item is-size-3">{ "Strategy" }</h2>
                    <InstrumentsList on_symbol_click={ on_symbol_click.clone() } on_watch_click={ on_watch_click.clone() } instruments={strategy} />
                    <h2 class="navbar-item is-size-3">{ "New patterns" }</h2>
                    <InstrumentsList on_symbol_click={ on_symbol_click.clone() } on_watch_click={ on_watch_click.clone() } instruments={suggested} />
                      <h2 class="navbar-item is-size-3">{ "Pattern activated" }</h2>
                    <InstrumentsList on_symbol_click={ on_symbol_click.clone() } on_watch_click={ on_watch_click.clone() } instruments={activated} />
                    <h2 class="navbar-item is-size-3">{ "Commodities " }</h2>
                    <InstrumentsList on_symbol_click={ on_symbol_click.clone() } on_watch_click={ on_watch_click.clone() } instruments={commodities} />
                     <h2 class="navbar-item is-size-3">{ "Crypto" }</h2>
                    <InstrumentsList on_symbol_click={ on_symbol_click } on_watch_click={ on_watch_click } instruments={crypto} />

            </div>
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct InstrumentsProps {
    use_instruments: Vec<CompactInstrument>,
}
