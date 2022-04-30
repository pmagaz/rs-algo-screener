use yew::{function_component, html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="content has-text-centered">
                { "Powered by " }
                <a href="https://yew.rs">{ "Rust" }</a>
            </div>
        </footer>
    }
}
