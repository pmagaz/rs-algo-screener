use crate::pages::home::Home;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    // #[at("/posts/:id")]
    // Post { id: u64 },
    // #[at("/posts")]
    // Posts,
    // #[at("/authors/:id")]
    // Author { id: u64 },
    // #[at("/authors")]
    // Authors,
    #[at("/")]
    Home,
    // #[not_found]
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
