use crate::pages::home::page::Home;
use crate::pages::strategies::page::Strategies;
use crate::pages::strategy::page::Strategy;

use rs_algo_shared::models::backtest_instrument::*;

use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/strategies")]
    Strategies,
    #[at("/strategy/:market/:name")]
    Strategy { market: String, name: String },
}

pub fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Strategies => {
            html! { <Strategies /> }
        }
        Route::Strategy { market, name } => {
            html! { <Strategy market={ market } name={ name }/> }
        }
    }
}
