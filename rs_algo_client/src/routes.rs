use crate::pages::home::page::Home;
use crate::pages::instruments::page::Instruments;
use crate::pages::strategies::page::Strategies;
use crate::pages::strategy::page::Strategy;

use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/strategies")]
    Strategies,
    #[at("/strategies/instruments")]
    Instruments,
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
        Route::Strategies => {
            html! { <Strategies /> }
        }
        Route::Instruments => {
            html! { <Instruments /> }
        }
        Route::Strategy {
            market,
            strategy,
            stype,
        } => {
            html! { <Strategy market={ market } strategy={strategy } stype={stype}/> }
        }
    }
}
