use crate::pages::bots::page::Bots;
use crate::pages::home::page::Home;
use crate::pages::strategies::page::Strategies;
use crate::pages::strategy::page::Strategy;

use yew::{html, Html};
use yew_router::prelude::*;

pub struct Instrument {
    market: String,
}

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/bots/")]
    Bots,
    #[at("/strategies/")]
    Strategies,
    #[at("/strategies/:instrument")]
    InstrumentsStrategy { instrument: String },
    #[at("/strategy/:market/:strategy/:stype/")]
    Strategy {
        market: String,
        strategy: String,
        stype: String,
    },
}

pub fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Bots => {
            html! { <Bots /> }
        }
        Route::Strategies => {
            html! { <Strategies  /> }
        }
        Route::Strategy {
            market,
            strategy,
            stype,
        } => {
            html! { <Strategy market={ market } strategy={ strategy } stype={stype} instrument= { "none" }/> }
        }

        Route::InstrumentsStrategy { instrument } => {
            html! { <Strategy market={ "instrument_strategy" } strategy={ "strategy" } stype={"stype"} instrument={ instrument }/> }
        }
    }
}
