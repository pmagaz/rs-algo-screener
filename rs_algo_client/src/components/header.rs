use crate::routes::Route;
use std::rc::Rc;
use yew::{classes, function_component, html, html::Scope, Html, Properties};
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub seed: u32,
}

// #[derive(PartialEq, Debug)]
// pub struct Footer {}

// impl Reducible for Footer {
//     type Action = u32;

//     fn reduce(self: Rc<Self>, action: u32) -> Rc<Self> {
//         Self {
//             inner: Author::generate_from_seed(action),
//         }
//         .into()
//     }
// }

#[function_component(Header)]
pub fn header() -> Html {
    //let Self { navbar_active, .. } = *self;
    let navbar_active = false;
    let active_class = if !navbar_active { "is-active" } else { "" };

    html! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <h1 class="navbar-item is-size-3">{ "RS-ALGO-SCREENER" }</h1>


                <button class={classes!("navbar-burger", "burger", active_class)}
                    aria-label="menu" aria-expanded="false"
                   // onclick={link.callback(|_| Msg::ToggleNavbar)}
                >
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </button>
            </div>
            <div class={classes!("navbar-menu", active_class)}>
                <div class="navbar-start">
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                        { "Home" }
                    </Link<Route>>
                    <div class="navbar-item has-dropdown is-hoverable">
                        <div class="navbar-link">
                            { "More" }
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
