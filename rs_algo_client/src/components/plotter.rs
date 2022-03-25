use yew::{function_component, html, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub url: String,
}

#[function_component(Plotter)]
pub fn plotter(props: &Props) -> Html {
    let Props { url } = props;
    log::info!("[CLIENT] Plotter {:?}", &props);
    html! {

    <div id="modal" class="modal">
      <div class="modal-background"></div>

      <div class="modal-content">
        <div class="box">
              <img src={format!("{}", url)} />
        </div>
      </div>

      <button class="modal-close is-large" aria-label="close"></button>
    </div>

        }
}
