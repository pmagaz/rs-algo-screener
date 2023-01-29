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
    #[at("/strategies/:id/:instrument")]
    InstrumentsStrategy { id: String, instrument: String },
    #[at("/strategy/:id/:strategy/:time_frame")]
    Strategy {
        id: String,
        strategy: String,
        time_frame: String,
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
            id,
            strategy,
            time_frame,
        } => {
            html! { <Strategy id={ id } instrument= { "none" } strategy={ strategy } time_frame={ time_frame}/> }
        }

        Route::InstrumentsStrategy { id, instrument } => {
            html! { <Strategy id={ id } instrument={ instrument } strategy={"none"} time_frame={"none"}/> }
        }
    }
}
