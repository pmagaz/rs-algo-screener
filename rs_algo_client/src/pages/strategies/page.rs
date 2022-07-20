use super::api;
use crate::components::loading::Loading;
use crate::pages::strategies::components::stratagies_list::StrategiesList;

use crate::components::chart::Chart;
use rs_algo_shared::models::backtest_strategy::*;
use rs_algo_shared::models::market::*;
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

#[function_component(Strategies)]
pub fn strategies() -> Html {
    let base_url = get_base_url();
    let strategies_url = [
        base_url.replace("strategies", "").as_str(),
        "api/backtest/strategies",
    ]
    .concat();
    let use_strategies = use_state(|| vec![]);
    let use_loading = use_state(|| true);
    let use_strategies_url = use_state(|| String::from(""));

    {
        let use_strategies = use_strategies.clone();
        let use_loading = use_loading.clone();
        let strategies_url = strategies_url.clone();
        use_effect_with_deps(
            move |_| {
                log::info!("[CLIENT] API call...");
                let use_loading = use_loading.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let query = get_query_value();
                    let strategies = api::get_strategies(&strategies_url, query).await.unwrap();
                    use_strategies.set(strategies);
                    use_loading.set(false);
                });
                || ()
            },
            (),
        );
    }

    let on_strategy_click = {
        let use_strategies_url = use_strategies_url.clone();
        Callback::from(move |strategies_url: String| {
            log::info!("[CLIENT] Selecting {}", &strategies_url);
            let use_strategies_url = use_strategies_url.clone();
            use_strategies_url.set(strategies_url);
            open_modal();
        })
    };

    let stock_strategies: Vec<BackTestStrategyResult> = use_strategies
        .iter()
        .filter(|x| x.market == Market::Stock)
        .map(|x| x.clone())
        .collect();

    let forex_strategies: Vec<BackTestStrategyResult> = use_strategies
        .iter()
        .filter(|x| x.market == Market::Forex)
        .map(|x| x.clone())
        .collect();

    let crypto_strategies: Vec<BackTestStrategyResult> = use_strategies
        .iter()
        .filter(|x| x.market == Market::Crypto)
        .map(|x| x.clone())
        .collect();

    html! {
        <div class="tile is-ancestor is-vertical ">
            <div class="section is-child hero">
                <div class="hero-body container pb-0">
                     <h1 class="navbar-item is-size-2">{ "Strategies" }</h1>
                        <Loading loading={ *use_loading} />
                </div>
            </div>
            <Chart url={(*use_strategies_url).clone()}/>
           <div class="container">
                <div class="notification is-fluid ">
                    <h2 class="navbar-item is-size-3">{ "Stocks" }</h2>
                    <StrategiesList market={ Market::Stock } strategies={(stock_strategies)} />
                    <h2 class="navbar-item is-size-3">{ "Forex" }</h2>
                    <StrategiesList market={ Market::Forex } strategies={(forex_strategies)} />
                    <h2 class="navbar-item is-size-3">{ "Crytpo" }</h2>
                    <StrategiesList market={ Market::Crypto } strategies={(crypto_strategies)} />
            </div>
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct StrategiesProps {
    use_strategies: Vec<BackTestStrategyResult>,
}
