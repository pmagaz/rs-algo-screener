use crate::pages::home::page::Home;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    // #[at("/posts/:id")]
    // Post { id: u64 },
    #[at("/")]
    Home,
    // #[at("/404")]
    // NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes.clone() {
        // Route::Post { id } => {
        //     html! { <Post seed={id} /> }
        // }
        Route::Home => {
            html! { <Home /> }
        }
    }
}
