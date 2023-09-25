use super::api;
use crate::components::chart::Chart;
use crate::components::instruments_list::*;
use crate::components::loading::Loading;
use rs_algo_shared::helpers::comp::*;
use rs_algo_shared::helpers::date::*;
use rs_algo_shared::helpers::symbols::{crypto, forex, sp500};
use rs_algo_shared::models::watch_instrument::*;
use rs_algo_shared::scanner::instrument::*;
use rs_algo_shared::scanner::candle::*;
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

    let forex_symbols = forex::get_symbols();
    let crypto_symbols = crypto::get_symbols();

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

    let on_symbol_click = {
        let use_instruments_url = use_instruments_url.clone();
        Callback::from(move |instruments_url: String| {
            log::info!("[CLIENT] Selecting {}", &instruments_url);
            let use_instruments_url = use_instruments_url.clone();
            use_instruments_url.set(instruments_url);
            open_modal();
        })
    };

    let on_action_click = {
        let use_watch_instruments = use_watch_instruments.clone();
        let use_portfolio_instruments = use_portfolio_instruments.clone();
        let use_loading = use_loading.clone();

        Callback::from(
            move |(action_type, list_type, inst): (ActionType, ListType, CompactInstrument)| {
                let use_watch_instruments = use_watch_instruments.clone();
                let use_portfolio_instruments = use_portfolio_instruments.clone();
                let portfolio_url = portfolio_url.clone();
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
                    match list_type {
                        _x if action_type == ActionType::WatchListAdd => {
                            let watch_list = &*use_watch_instruments;
                            let mut current_items = watch_list.clone();
                            if !current_items.contains(&inst) {
                                current_items.push(inst.clone());
                                use_watch_instruments.set(current_items);
                            }
                            api::upsert_watch_instrument(watch_list_url.clone(), watch_instrument)
                                .await
                                .unwrap()
                        }
                        _x if action_type == ActionType::WatchListDelete => {
                            let watch_list = &*use_watch_instruments;
                            let current_items: Vec<CompactInstrument> = watch_list
                                .iter()
                                .filter(|x| inst.symbol != x.symbol)
                                .map(|x| x.clone())
                                .collect();
                            use_watch_instruments.set(current_items);
                            api::delete_watch_instrument(watch_list_url.clone(), watch_instrument)
                                .await
                                .unwrap()
                        }
                        _x if action_type == ActionType::PortfolioAdd => {
                            let portfolio = &*use_portfolio_instruments;
                            let mut current_items = portfolio.clone();
                            if !current_items.contains(&inst) {
                                current_items.push(inst.clone());
                                use_portfolio_instruments.set(current_items);
                            }
                            api::upsert_portfolio_instrument(
                                portfolio_url.clone(),
                                watch_instrument,
                            )
                            .await
                            .unwrap()
                        }
                        _x if action_type == ActionType::PortfolioDelete => {
                            let portfolio = &*use_portfolio_instruments;
                            let current_items: Vec<CompactInstrument> = portfolio
                                .iter()
                                .filter(|x| inst.symbol != x.symbol)
                                .map(|x| x.clone())
                                .collect();
                            use_portfolio_instruments.set(current_items);
                            api::delete_portfolio_instrument(
                                portfolio_url.clone(),
                                watch_instrument,
                            )
                            .await
                            .unwrap()
                        }
                        _ => api::upsert_watch_instrument(watch_list_url.clone(), watch_instrument)
                            .await
                            .unwrap(),
                    };
                    use_loading.set(false);
                });
            },
        )
    };

    fn candle_type_to_string(candle_type: &CandleType) -> u8 {
    match candle_type {
        CandleType::Reversal => 1,
        CandleType::Karakasa => 2,
        CandleType::BullishGap => 3,
        CandleType::Engulfing => 4,
        CandleType::BearishKarakasa => 5,
    
        _ => 0, 
        // Add more cases for other candle types if needed
    }
}

    let mut candles: Vec<CompactInstrument> = use_instruments
        .iter()
        .filter(|x| x.current_candle == CandleType::Reversal
            || x.current_candle == CandleType::Karakasa
            || x.current_candle == CandleType::BearishKarakasa
            || x.current_candle == CandleType::Engulfing
            || x.current_candle == CandleType::BullishGap)
        .cloned()
        .collect();

   candles.sort_by_key(|x| candle_type_to_string(&x.current_candle));

        
    let mut suggested: Vec<CompactInstrument> = use_instruments
        .iter()
        .filter(|x| match x.patterns.local_patterns.last() {
            Some(last_pat) => {
                last_pat.date > to_dbtime(Local::now() - Duration::days(3))
                    && !last_pat.active.active
            }
            None => false,
        })
        .map(|x| x.clone())
        .collect();

    suggested.retain(|x| !candles.contains(x));


    let mut activated: Vec<CompactInstrument> = use_instruments
        .iter()
        .filter(|x| match x.patterns.local_patterns.last() {
            Some(last_pat) => {
                last_pat.active.active
                    && last_pat.active.date > to_dbtime(Local::now() - Duration::days(3))
            }
            None => false,
        })
        .map(|x| x.clone())
        .collect();

        activated.retain(|x| !candles.contains(x));


    let mut strategy: Vec<CompactInstrument> = use_instruments
        .iter()
        .filter(|x| {
            x.current_price <= x.indicators.bb.current_b && x.prev_price >= x.indicators.bb.prev_b
        })
        .map(|x| x.clone())
        .collect();

            strategy.retain(|x| !candles.contains(x));


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

    let forex: Vec<CompactInstrument> = use_instruments
        .iter()
        .filter(|x| symbol_in_list(&x.symbol, &forex_symbols))
        .map(|x| x.clone())
        .collect();

    let crypto: Vec<CompactInstrument> = use_instruments
        .iter()
        .filter(|x| symbol_in_list(&x.symbol, &crypto_symbols))
        .map(|x| x.clone())
        .collect();

    html! { 
        <div class="tile is-ancestor is-vertical ">
            <div class="section is-child hero">
                <div class="hero-body container pb-0">
                        //<label class="label">{ "Query" }</label>
                     <h1 class="navbar-item is-size-2">{ "Screener" }</h1>

                        //<textarea id="query_box" class="textarea is-link is-invisible" placeholder="Textarea" cols="60" rows="0" value={ {format!("{}", *use_query)}}></textarea>
                        // <button id="leches" class="button" onclick={on_query_send}>{ "Search" }</button>
                        //<div width="400" height="400"></div>
                        <Loading loading={ *use_loading } />
                </div>
            </div>
            <Chart url={(*use_instruments_url).clone()}/>
           <div class="container">
                <div class="notification is-fluid ">
                    <h2 class="navbar-item is-size-3">{ "Portfolio" }</h2>
                    <InstrumentsList list_type={ ListType::PortFolio } on_symbol_click={ on_symbol_click.clone()} on_action_click={ on_action_click.clone()} instruments={(*use_portfolio_instruments).clone()} />
                    <h2 class="navbar-item is-size-3">{ "Watch List" }</h2>
                    <InstrumentsList list_type={ ListType::WatchList } on_symbol_click={ on_symbol_click.clone()} on_action_click={ on_action_click.clone()} instruments={(*use_watch_instruments).clone()} />
                    <h2 class="navbar-item is-size-3">{ "Strategy" }</h2>
                    <InstrumentsList list_type={ ListType::Strategy } on_symbol_click={ on_symbol_click.clone() } on_action_click={ on_action_click.clone() } instruments={strategy} />
                    <h2 class="navbar-item is-size-3">{ "Candles" }</h2>
                    <InstrumentsList list_type={ ListType::Candles } on_symbol_click={ on_symbol_click.clone() } on_action_click={ on_action_click.clone() } instruments={candles} />
                    <h2 class="navbar-item is-size-3">{ "New patterns" }</h2>
                    <InstrumentsList list_type={ ListType::NewPatterns } on_symbol_click={ on_symbol_click.clone() } on_action_click={ on_action_click.clone() } instruments={suggested} />
                    <h2 class="navbar-item is-size-3">{ "Pattern activated" }</h2>
                    <InstrumentsList list_type={ ListType::Activated } on_symbol_click={ on_symbol_click.clone() } on_action_click={ on_action_click.clone() } instruments={activated} />
                    <h2 class="navbar-item is-size-3">{ "Forex" }</h2>
                    <InstrumentsList list_type={ ListType::Forex } on_symbol_click={ on_symbol_click.clone() } on_action_click={ on_action_click.clone() } instruments={forex} />
                    <h2 class="navbar-item is-size-3">{ "Crypto" }</h2>
                    <InstrumentsList list_type={ ListType::Crypto } on_symbol_click={ on_symbol_click.clone() } on_action_click={ on_action_click.clone() } instruments={crypto} />
                    <h2 class="navbar-item is-size-3">{ "Commodities " }</h2>
                    <InstrumentsList list_type={ ListType::Commodities } on_symbol_click={ on_symbol_click } on_action_click={ on_action_click } instruments={commodities} />

            </div>
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct InstrumentsProps {
    use_instruments: Vec<CompactInstrument>,
}
