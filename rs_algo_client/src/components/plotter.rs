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

      <div class="modal" id="modal">
      <div class="modal-background" id="modal-background"></div>
      <div class="modal-content">
          <img src={format!("{}", url)} width="1024" height="768"/>
      </div>
      <button class="modal-close is-large" aria-label="close"></button>
    </div>
        }
}
