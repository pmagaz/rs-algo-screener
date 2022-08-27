use dotenv::dotenv;
use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod helpers;
mod pages;
mod routes;

use components::footer::Footer;
use components::header::Header;
use routes::{switch, Route};

pub enum Msg {}

pub struct App {}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("[CLIENT] Loading App...");
        leches();
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Header />
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <Footer />
            </BrowserRouter>
        }
    }
}

fn leches() {}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
