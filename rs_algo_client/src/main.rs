use dotenv::dotenv;
use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod pages;
mod render_image;
mod routes;
use components::footer::Footer;
use components::header::Header;
use render_image::Backend;
use routes::{switch, Route};
use std::env;

pub enum Msg {}

pub struct App {}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("[CLIENT] Loading app...");
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

fn main() {
    dotenv().ok();
    //let port = env::var("RENDER_TO_IMAGE").expect("RENDER_TO_IMAGE not found");
    // log::info!("[aaaaaaa] Loading app... {}", port);

    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
