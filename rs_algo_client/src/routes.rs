use crate::pages::home::page::Home;
use crate::pages::strategies::page::Strategies;

use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/strategies")]
    Strategies,
}

pub fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Strategies => {
            html! { <Strategies /> }
        }
    }
}
