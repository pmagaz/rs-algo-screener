use std::rc::Rc;

use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

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
