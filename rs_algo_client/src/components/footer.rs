use std::rc::Rc;

use yew::{function_component, html, Html, Properties};
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

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {

                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                    </div>
                </footer>
    }
}
