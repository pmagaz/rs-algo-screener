use super::api;
use crate::components::loading::Loading;
use crate::pages::strategy::components::strategy_detail::StrategyDetail;

use crate::components::chart::Chart;
use rs_algo_shared::models::backtest_instrument::*;
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

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(Strategy)]
pub fn strategy(props: &Props) -> Html {
    let Props { id } = props;
    let base_url = get_base_url();
    let backtested_strategy_url = [
        base_url.replace(&["strategy/", id].concat(), "").as_str(),
        "api/backtest/strategies/",
        id,
    ]
    .concat();
    let use_backtest_instruments = use_state(|| vec![]);
    let use_loading = use_state(|| true);
    let use_strategy_url = use_state(|| String::from(""));

    {
        let use_backtest_instruments = use_backtest_instruments.clone();
        let use_loading = use_loading.clone();
        let backtested_strategy_url = backtested_strategy_url.clone();
        use_effect_with_deps(
            move |_| {
                log::info!("[CLIENT] API call...");
                let use_loading = use_loading.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let query = get_query_value();
                    let strategy =
                        api::get_backtest_strategy_instruments(&backtested_strategy_url, query)
                            .await
                            .unwrap();
                    use_backtest_instruments.set(strategy);
                    use_loading.set(false);
                });
                || ()
            },
            (),
        );
    }

    let on_symbol_click = {
        let use_strategy_url = use_strategy_url.clone();
        Callback::from(move |backtested_strategy_url: String| {
            log::info!("[CLIENT] Selecting {}", &backtested_strategy_url);
            let use_strategy_url = use_strategy_url.clone();
            use_strategy_url.set(backtested_strategy_url);
            open_modal();
        })
    };

    html! {
        <div class="tile is-ancestor is-vertical ">
            <div class="section is-child hero">
                <div class="hero-body container pb-0">
                     <h1 class="navbar-item is-size-2">{ id }</h1>

                        <Loading loading={ *use_loading} />
                </div>
            </div>
            <Chart url={(*use_strategy_url).clone()}/>
           <div class="container">
                <div class="notification is-fluid ">
                <StrategyDetail on_symbol_click={ on_symbol_click.clone()} backtested_instruments={(*use_backtest_instruments).clone()} />
            </div>
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct StrategyProps {
    use_backtest_instruments: Vec<BackTestInstrumentResult>,
}
